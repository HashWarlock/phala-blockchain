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

use phala_types::messaging::AuctionMarketCommand;
use chain::Balance;

type Command = AuctionMarketCommand;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct RmrkNft {
    nft_id: String,
    name: String,
    metadata: String,
    block: BlockNumber,
    collection_id: String,
}

#[derive(Encode, Decode, Debug, Clone)]
pub struct Auction {
    nft_id: String,
    amount: Balance,
    duration: u64,
    buffer: u64,
    auto_bid_increase: Balance,
    bidder: AccountId,
    settled: bool,
}

#[derive(Default)]
pub struct AuctionBot {
    owner: AccountId,
    bot_name: String,
    bot_token: String,
    chat_id: String,
    auctions: BTreeMap<String, Auction>,
    active: bool,
}

/// The payloads of the Telegram `sendMessage` request
/// refer to: https://core.telegram.org/bots/api#sendmessage
#[derive(Deserialize, Serialize)]
struct TgMessage {
    chat_id: String,
    text: String,
}

pub struct AuctionMarket {
    auctions: BTreeMap<AccountId, AuctionBot>,
}

/// Query requests. The end users can only query the contract states by sending requests.
/// Queries are not supposed to write to the contract states.
#[derive(Encode, Decode, Debug, Clone)]
pub enum Request {
    /// Query owner of contract
    QueryOwner,
    /// Query bot token associated with the bot name
    QueryBotToken { bot_name: String },
    /// Query Telegram chat id associated with the bot name
    QueryChatId { bot_name: String },
    /// Query auction status of an NFT id
    QueryNftStatus { nft_id: String },
    /// List NFT auctions run by owner
    ListAuctions,
}

#[derive(Encode, Decode, Debug, Clone, PartialEq)]
pub enum Response {
    Owner(AccountId),
    BotToken(String),
    ChatId(String),
    NftStatus(Auction),
    AuctionsList(Vec<Auction>)
}

#[derive(Encode, Decode, Debug)]
pub enum Error {
    OriginUnavailable,
    NotAuthorized,
    NoNftDetected,
    NoAuctionsDetected,
}

impl RmrkNft {
    pub fn new(
        nft_id: String,
        name: String,
        metadata: String,
        block: BlockNumber,
        collection_id: String
    ) -> Self {
        RmrkNft { nft_id, name, metadata, block, collection_id } }
}

impl Auction {
    pub fn new(
        nft_id: String,
        amount: Balance,
        duration: u64,
        buffer: u64,
        auto_bid_increase: Balance,
        bidder: AccountId,
    ) -> Self {
        Auction { nft_id, amount, duration, buffer, auto_bid_increase, bidder, settled: false }
    }
}

impl AuctionBot {
    pub fn new() -> Self {
        AuctionBot {
            owner: Default::default(),
            bot_name: Default::default(),
            bot_token: Default::default(),
            chat_id: Default::default(),
            auctions: BTreeMap::new(),
            active: true,
        }
    }
}

impl AuctionMarket {
    pub fn new() -> Self {
        AuctionMarket {
            auctions: BTreeMap::new(),
        }
    }
}

// Alice is the pre-defined root account in dev mode
const ALICE: &str = "d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
// RMRK 1.0 HTTP URI
const RMRK_URI: &str = "https://singular.rmrk.app/api/rmrk1/nft/";
// RMRK 1.0 HTTP URL Link
const RMRK_NFT_LINK: &str = "https://singular.rmrk.app/collectibles/";
