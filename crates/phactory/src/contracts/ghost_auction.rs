use anyhow::Result;
use std::collections::BTreeMap;
use std::time::{Duration, SystemTime};
use log::info;
use parity_scale_codec::{Decode, Encode};
use phala_mq::{contract_id256, MessageOrigin};
use phala_types::contract::command_topic;
use serde::{Deserialize, Serialize};
use serde_json::{error, Value};
use chain::BlockNumber;

use surf;

use super::{TransactionError, TransactionResult};
use crate::contracts;
use crate::contracts::{AccountId, NativeContext};
use crate::secret_channel::Payload;
use crate::side_task::async_side_task::AsyncSideTask;
extern crate runtime as chain;

use phala_types::messaging::GhostAuctionCommand;

type Command = GhostAuctionCommand;
/// Ghost Auctioneer Bot
/// The bot is consists of the following:
///     Owner
///     Bot Token from Telegram
///     Chat Id from Telegram Group
///     RMRK NFT Id for querying the RMRK HTTP endpoint
///     Minimum Reserve Price
///     Auto-Increment per Request
///     Current Bidder
#[derive(Default)]
pub struct GhostAuctioneerBot {
    owner: AccountId,
    bot_token: String,
    chat_id: String,
    nft_id: String,
    reserve_price: u64,
    auto_bid_increase: u64,
    bidder: AccountId,
    settled: bool,
}

/// The payloads of the Telegram `sendMessage` request
/// refer to: https://core.telegram.org/bots/api#sendmessage
#[derive(Deserialize, Serialize)]
struct TgMessage {
    chat_id: String,
    text: String,
}

#[derive(Serialize, Deserialize)]
struct RmrkNft {
    id: String,
    name: String,
    metadata: String,
    block: BlockNumber,
}

#[derive(Encode, Decode, Debug, Clone)]
pub enum Request {
    QueryOwner,
    QueryBotToken,
    QueryChatId,
    QueryNft,
    QueryNftPrice,
    QueryNextBidPrice,
}

#[derive(Encode, Decode, Debug, Clone, PartialEq)]
pub enum Response {
    Owner(AccountId),
    BotToken(String),
    ChatId(String),
    NftId(String),
    NftPrice(String),
    NextBidPrice(String),
}

#[derive(Encode, Decode, Debug)]
pub enum Error {
    OriginUnavailable,
    NotAuthorized,
    NoAuctionDetected,
    NoNftDetected,
}

impl GhostAuctioneerBot {
    pub fn new() -> Self {
        GhostAuctioneerBot {
            owner: Default::default(),
            bot_token: Default::default(),
            chat_id: Default::default(),
            nft_id: Default::default(),
            reserve_price: 1,
            auto_bid_increase: 1,
            bidder: Default::default(),
            settled: false,
        }
    }
}

// Alice is the pre-defined root account in dev mode
const ALICE: &str = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
// RMRK 1.0 HTTP URI
const RMRK_URI: &str = "https://singular.rmrk.app/api/rmrk1/nft/";
// RMRK 1.0 HTTP URL Link
const RMRK_NFT_LINK: &str = "https://singular.rmrk.app/collectibles/";
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

        let alice = contracts::account_id_from_hex(ALICE)
            .expect("should not failed with valid address; qed.");
        match cmd {
            Command::SetOwner { owner } => {
                if origin.account()? != alice {
                    return Err(TransactionError::BadOrigin);
                }
                self.owner = AccountId::from(*owner.as_fixed_bytes());
                Ok(())
            }
            Command::SetupBot { token, chat_id } => {
                let sender = origin.account()?;
                if sender != alice && sender != self.owner {
                    return Err(TransactionError::BadOrigin);
                }
                self.bot_token = token;
                self.chat_id = chat_id;
                Ok(())
            }
            Command::SetupGhostAuction { nft_id, reserve_price, auto_bid_increase } => {
                let sender = origin.account()?;
                if sender != alice && sender != self.owner {
                    return Err(TransactionError::BadOrigin);
                }

                let bot_token = self.bot_token.clone();
                let chat_id = self.chat_id.clone();
                let query_nft_uri = format!("{}{}", RMRK_URI, nft_id.clone());
                self.reserve_price = reserve_price.clone();
                self.auto_bid_increase = auto_bid_increase.clone();
                self.bidder = self.owner.clone();
                self.settled = false;

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

                /*let mq = context.mq().clone();
                let my_id = self.id();*/

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
                            .await
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

                        let mut rmrk_nft_vec: Vec<RmrkNft> = serde_json::from_str(result.as_str()).expect("broken RMRK NFT result");

                        let rmrk_nft = rmrk_nft_vec.remove(0);
                        let rmrk_nft_id = rmrk_nft.id;
                        log::info!("RMRK NFT ID: {}", rmrk_nft_id);
                        let rmrk_nft_name = rmrk_nft.name;
                        log::info!("RMRK NFT name: {}", rmrk_nft_name);
                        let rmrk_nft_metadata = rmrk_nft.metadata;
                        log::info!("RMRK NFT metadata: {}", rmrk_nft_metadata);
                        let rmrk_nft_block = rmrk_nft.block;
                        log::info!("RMRK NFT BlockNumber: {}", rmrk_nft_block);
                        let rmrk_nft_url = format!("{}{}", RMRK_NFT_LINK, nft_id.clone());

                        let text = format!("New Ghost Auction Alert!!!\nNFT ID: {}\nNFT Name: {}\nNFT Metadata: {}\nNFT BlockNumber: {}\nRMRK URL: {}\nStarting Bid Price: {}",
                                           rmrk_nft_id,
                                           rmrk_nft_name,
                                           rmrk_nft_metadata,
                                           rmrk_nft_block,
                                           rmrk_nft_url,
                                           reserve_price
                        );
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

                        // let bidder = sender.clone();
                        result
                    },
                    |_result, _context| {
                        // You can send deterministic number of transactions in the result process
                        /*log::info!("Side task establishing new NFT ghost auction: {}", nft_id.clone());
                        // Send to SetTopBid to create new bid.
                        let command = Command::SetTopBid { bidder, reserve_price };
                        let message = Payload::Plain(command);
                        mq.sendto(&message, command_topic(contract_id256(my_id)));*/
                    },
                );
                context.block.side_task_man.add_task(task);

                Ok(())
            }
            /*Command::SubmitAutoBid {} => {
                let sender = origin.account()?;
                if sender == alice || sender == self.owner || self.settled != false {
                    return Err(TransactionError::BadOrigin);
                }
                let nft_id = self.nft_id.clone();
                self.reserve_price.saturating_add(self.auto_bid_increase);
                self.bidder = sender.clone();
                log::info!("New bid from {} at {} KSM", self.bidder.clone(), self.reserve_price.clone());

                Ok(())
            }
            Command::SettleAuction{} => {
                let sender = origin.account()?;
                if sender != alice && sender != self.owner {
                    return Err(TransactionError::BadOrigin);
                }
                log::info!("NFT ID: {} auction has closed. Sold for {} KSM to Account Id {}", self.nft_id, self.reserve_price, self.bidder);

                self.settled = true;
                Ok(())
            }*/
            Command::SubmitAutoBid {} => {
                let sender = origin.account()?;
                if sender == alice || sender == self.owner || self.settled != false {
                    return Err(TransactionError::BadOrigin);
                }
                let bot_token = self.bot_token.clone();
                let chat_id = self.chat_id.clone();
                let nft_id = self.nft_id.clone();
                let top_bid = self.reserve_price.clone();
                let auto_bid_increase = self.auto_bid_increase.clone();
                let reserve_price = top_bid.saturating_add(auto_bid_increase);
                // Update the reserve_price with new bid amount
                self.reserve_price = reserve_price;
                self.bidder = sender;
                let block_number = context.block.block_number;
                let duration = 2;

                let task = AsyncSideTask::spawn(
                    block_number,
                    duration,
                    async move {
                        // Do network request in this block and return the result.
                        // Do NOT send mq message in this block.
                        log::info!("Side task starts to submit new bid on NFT");

                        let text = format!("NFT ID: {} has a NEW TOP BID!!!\nNew top bid is now {} KSM", nft_id, reserve_price);
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

                        log::info!("Side task sent new auto bid of {} KSM info: {}", reserve_price, result);
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

            Command::SettleAuction {} => {
                let sender = origin.account()?;
                let bot_token = self.bot_token.clone();
                let chat_id = self.chat_id.clone();
                let nft_id = self.nft_id.clone();
                let price = self.reserve_price.clone();
                let bidder = self.bidder.clone();
                self.settled = true;
                let settled = self.settled.clone();
                log::info!("Side task sent ghost auction settled: {}", settled);

                let block_number = context.block.block_number;
                let duration = 2;

                let task = AsyncSideTask::spawn(
                    block_number,
                    duration,
                    async move {
                        // Do network request in this block and return the result.
                        // Do NOT send mq message in this block.
                        log::info!("Side task starts to settle auction on NFT");

                        let text = format!("NFT ID: {} auction has closed!!!\nSold for {} KSM to Account Id {}", nft_id, price, bidder);
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
        origin:Option<&chain::AccountId>,
        req: Request,
    ) -> Result<Response, Error> {
        info!("Query received: {:?}", &req);

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
                let nft_id = Some(self.nft_id.clone()).ok_or(Error::NoNftDetected)?;
                Ok(Response::NftId(self.nft_id.clone()))
            }
            Request::QueryNftPrice => {
                let nft_price = Some(self.reserve_price.clone()).ok_or(Error::NoNftDetected)?;
                Ok(Response::NftPrice(self.reserve_price.clone().to_string()))
            }
            Request::QueryNextBidPrice => {
                let mut top_bid = Some(self.reserve_price.clone()).ok_or(Error::NoAuctionDetected)?;
                let auto_incr_bid_by = Some(self.auto_bid_increase.clone()).ok_or(Error::NoAuctionDetected)?;
                top_bid.saturating_add(auto_incr_bid_by);
                Ok(Response::NextBidPrice(top_bid.clone().to_string()))
            }
        }
    }
}