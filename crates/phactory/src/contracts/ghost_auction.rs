use anyhow::Result;
use std::collections::BTreeMap;
use std::time::{Duration, SystemTime};
use log::info;
use parity_scale_codec::{Decode, Encode};
use phala_mq::MessageOrigin;
use serde::{Deserialize, Serialize};
use serde_json;
use lazy_static;

use surf;

use super::{TransactionError, TransactionResult};
use crate::contracts;
use crate::contracts::{AccountId, NativeContext};
use crate::side_task::async_side_task::AsyncSideTask;
extern crate runtime as chain;

use phala_types::messaging::GhostAuctionCommand;

type Command = GhostAuctionCommand;

lazy_static! {
    // 10000...000, used to tell if this is a NFT
    static ref TYPE_NF_BIT: U256 = U256::from(1) << 255;
    // 1111...11000...00, used to store the type in the upper 128 bits
    static ref TYPE_MASK: U256 = U256::from(!u128::MIN) << 128;
    // 00...00011....11, used to get NFT index in the lower 128 bits
    static ref NF_INDEX_MASK: U256 = U256::from(!u128::MIN);
}

/// The payloads of the Telegram `sendMessage` request
/// refer to: https://core.telegram.org/bots/api#sendmessage
#[derive(Deserialize, Serialize)]
struct TgMessage {
    chat_id: String,
    text: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct RmrkNft {
    nft_id: String,
    block: BlockNumber,
    metadata: String,
    name: String,
}

#[derive(Encode, Decode, Debug, Clone, PartialEq)]
pub struct Auction {
    rmrk_nft: RmrkNft,
    amount: u64,
    start_time: u64,
    end_time: u64,
    bidder: AccountId,
    settled: bool,
}

pub struct GhostAuction {
    rmrk_nft: RmrkNft,
    reserve_price: u64,
    min_bid_increase: u8,
    duration: u64,
    auction: Auction,
}

pub struct GhostAuctioneerBot {
    owner: AccountId,
    bot_token: String,
    chat_id: String,
    ghost_auction: GhostAuction,
}

#[derive(Encode, Decode, Debug, Clone)]
pub enum Request {
    QueryOwner,
    QueryBotToken,
    QueryChatId,
    QueryNft,
    QueryNftTopBid,
    QueryAuctionTimeLeft,
}

#[derive(Encode, Decode, Debug, Clone, PartialEq)]
pub enum Response {
    Owner(AccountId),
    BotToken(String),
    ChatId(String),
    Nft(String),
    NftTopBid(u64),
    TimeLeft(u64),
}

#[derive(Encode, Decode, Debug)]
pub enum Error {
    OriginUnavailable,
    NotAuthorized,
    NoAuctionDetected,
    NoNftDetected,
}
/*
impl RmrkNft {
    pub fn new(nft_id: String, block: BlockNumber, metadata: String, name: String) -> Self {
        RmrkNft {
            nft_id,
            block,
            metadata,
            name,
        }
    }
}
*/
impl Auction {
    pub fn new(rmrk_nft: RmrkNft,
               amount: u64,
               start_time: u64,
               end_time: u64,
               bidder: AccountId,
               settled: bool) -> Self {
        Auction {
            rmrk_nft,
            amount,
            start_time,
            end_time,
            bidder,
            settled,
        }
    }
}

impl GhostAuction {
    pub fn new() -> Self {
        GhostAuction {
            rmrk_nft: Default::default(),
            reserve_price: Default::default(),
            min_bid_increase: Default::default(),
            duration: Default::default(),
            auction: Default::default(),
        }
    }
}

impl GhostAuctioneerBot {
    pub fn new() -> Self {
        GhostAuctioneerBot {
            owner: Default::default(),
            bot_token: Default::default(),
            chat_id: Default::default(),
            ghost_auction: Default::default(),
        }
    }
}

// Alice is the pre-defined root account in dev mode
const ALICE: &str = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
// RMRK 1.0 HTTP URI
const RMRK_URI: String = "https://singular.rmrk.app/api/rmrk1/nft/";

impl contracts::NativeContract for GhostAuctioneerBot {
    type Cmd = Command;
    type QReq = Request;
    type QResp = Result<Response, Error>;

    /// Return the contract id which uniquely identifies the contract
    fn id(&self) -> contracts::ContractId32 { contracts::GHOST_AUCTIONEER_BOT }

    /// Handle the Commands from transactions on the blockchain. This method doesn't respond.
    fn handle_command(
        &mut self,
        context: &mut NativeContext,
        origin: MessageOrigin,
        cmd: Command,
    ) -> TransactionResult {
        info!("Command received: {:?}", &cmd);

        // we want to limit the sender who can use the Commands to the pre-define root account
        let sender = match &origin {
            MessageOrigin::AccountId(account) => AccountId::from(*account.as_fixed_bytes()),
            _ => return Err(TransactionError::BadOrigin),
        };
        let alice = contracts::account_id_from_hex(ALICE)
            .expect("should not failed with valid address; qed.");
        match cmd {
            Command::SetOwner { owner } => {
                if sender != alice {
                    return Err(TransactionError::BadOrigin);
                }
                self.owner = AccountId::from(*owner.as_fixed_bytes());
                Ok(())
            }
            Command::SetupBot { token, chat_id } => {
                if sender != alice && sender != self.owner {
                    return Err(TransactionError::BadOrigin);
                }
                self.bot_token = token;
                self.chat_id = chat_id;
                Ok(())
            }
            Command::SetupGhostAuction { nft_id, reserve_price, min_bid_increase, duration } => {
                if sender != alice && sender != self.owner {
                    return Err(TransactionError::BadOrigin);
                }
                // Check if valid NFT ID
                if nft_id & *TYPE_NF_BIT != *TYPE_NF_BIT {
                    return Err(Error::NoNftDetected);
                }

                let bot_token = self.bot_token.clone();
                let chat_id = self.chat_id.clone();
                let query_nft_uri = format!("{}{}", RMRK_URI, nft_id);

                // This Command triggers the use of `AsyncSideTask`, it first send a HTTP request to get the current BTC
                // price from https://min-api.cryptocompare.com/, then sends the price to a Telegram bot through another
                // HTTP request
                //
                // To ensure the state consistency, the time to start the task and the time to upload the HTTP response
                // to chain must be determined. In this case, we start the task in the current `block_number`, and report
                // the result, whether succeeded or failed, to the chain after `duration`
                //
                // Report the result after 2 blocks no matter whether has received the HTTP response
                let block_number = context.block.block_number;
                let duration = 2;

                let task = AsyncSideTask::spawn(
                    block_number,
                    duration,
                    async move {
                        // Do network request in this block and return the result.
                        // Do NOT send mq message in this block.
                        log::info!("Side task starts to verify NFT exists");
                        let mut resp = match surf::get(
                            query_nft_uri,
                        )
                            .send()
                            .await()
                        {
                            Ok(r) => r,
                            Err(err) => {
                                return format!("Network error: {:?}", err)
                            }
                        };
                        let result = match resp.body_string().await {
                            Ok(body) => body,
                            Err(err) => {
                                format!("Network error: {:?}", err)
                            }
                        };
                        log::info!("Side task got RMRK NFT response: {}", result);

                        let rmrk_nft_vec: Vec<RmrkNft> = serde_json::from_str(result.as_str()).expect("broken RMRK NFT result");
                        // If results are an empty array then the NFT ID doesn't exists
                        if rmrk_nft_vec.is_empty() == true {
                            return Err(Error::NoNftDetected);
                        }

                        let rmrk_nft: RmrkNft = rmrk_nft_vec[0];
                        let rmrk_nft_id = rmrk_nft.nft_id;
                        log::info!("RMRK NFT ID: {}", rmrk_nft_id);
                        let rmrk_nft_name = rmrk_nft.name;
                        log::info!("RMRK NFT name: {}", rmrk_nft_name);
                        let rmrk_nft_metadata = rmrk_nft.metadata;
                        log::info!("RMRK NFT metadata: {}", rmrk_nft_metadata);
                        let rmrk_nft_block = rmrk_nft.block;
                        log::info!("RMRK NFT ID: {}", rmrk_nft_block);

                        let start_time = SystemTime::now()
                            .checked_add(Duration::from_secs(600)) // 10 minutes
                            .expect("Failed to generate new start time for Ghost Auction");
                        let end_time = SystemTime::now()
                            .checked_add(Duration::from_secs(24 * 3600)) // 24 hours
                            .expect("Failed to generate new start time for Ghost Auction");
                        let auction: Auction =
                            Auction::new(
                                rmrk_nft.clone(),
                                reserve_price,
                                start_time,
                                end_time,
                                self.owner,
                                false
                            );

                        let ghost_auction: GhostAuction =
                            GhostAuction::new(
                                rmrk_nft.clone(),
                                reserve_price,
                                min_bid_increase,
                                86400,
                                auction.clone()
                            );

                        self.ghost_auction = ghost_auction.clone();

                        let text = format!("Ghost Auction Alert in 10 minutes for NFT ID: {}", rmrk_nft_id);
                        let uri = format!(
                            "https://api.telegram.org/bot{}/{}",
                            bot_token, "sendMessage"
                        );
                        // Report new ghost auction created by owner
                        let data = &TgMessage { chat_id, text };

                        let mut resp = match surf::post(uri)
                            .body_json(data)
                            .expect("should not fail with valid data; qed.")
                            .await
                        {
                            Ok(r) => r,
                            Err(err) => {
                                return format!("Network error: {:?}", err);
                            }
                        };
                        let result = match resp.body_string().await {
                            Ok(body) => body,
                            Err(err) => {
                                format!("Network error: {:?}", err)
                            }
                        };
                        log::info!("Side task sent new Ghost Auction info: {}", result);
                        result
                    },
                    |_result, _context| {
                        // You can send deterministic number of transactions in the result process
                        // In this case, we don't send the price since it has already been reported to the TG bot above
                    },
                );
                context.block.side_task_man.add_task(task);

                Ok(())
            }
        }
    }

    // Handle a direct Query and respond to it. It shouldn't modify the contract state.
    fn handle_query(
        &mut self,
        origin:Option<&chain::AcountId>,
        req: Request,
    ) -> Result<Response, Error> {
        info!("Query received: {?:}", &req);

        let sender = origin.ok_or(Error::OriginUnavailable)?;
        let alice = contracts::account_id_from_hex(ALICE)
            .expect("should not failed with valid address; qed.");
        match req {
            Request::QueryOwner => Ok(Response::Owner(self.owner.clone())),
            Request::QueryBotToken => {
                if sender != &alice && sender != &self.owner {
                    return Err(Error::NotAuthorized);
                }

                Ok(Response::BotToken(self.bot_token.clone()))
            }
            Request::QueryChatId => {
                if sender != &alice && sender != &self.owner {
                    return Err(Error::NotAuthorized);
                }

                Ok(Response::ChatId(self.chat_id.clone()))
            }
            Request::QueryNft => {
                if sender != &alice && sender != &self.owner {
                    return Err(Error::NotAuthorized);
                }

                let nft_id = self.ghost_auction.rmrk_nft.nft_id.clone().ok_or(Error::NoNftDetected)?;
                Ok(Response::Nft(nft_id))
            }
            Request::QueryNftTopBid => {
                if sender != &alice && sender != &self.owner {
                    return Err(Error::NotAuthorized);
                }

                let top_bid = self.ghost_auction.auction.amount.clone().ok_or(Error::NoAuctionDetected)?;
                Ok(Response::NftTopBid(top_bid))
            }
            Request::QueryAuctionTimeLeft => {
                if sender != &alice && sender != &self.owner {
                    return Err(Error::NotAuthorized);
                }

                let time_left = self.ghost_auction.duration.clone().ok_or(Error::NoAuctionDetected)?;
                Ok(Response::TimeLeft(time_left))
            }
        }
    }
}