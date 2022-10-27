use std::collections::HashMap;

use near_events::{
    near_event_data,
    near_event_data_log,
};
#[cfg(feature = "de")]
use near_sdk::serde::Deserialize;
#[cfg(feature = "ser")]
use near_sdk::serde::Serialize;
use near_sdk::{
    json_types::U128,
    AccountId,
};

use crate::types::mb_market_v01::TokenOffer;

// ----------------------------- create listing ----------------------------- //
#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[cfg_attr(feature = "ser", derive(Serialize))]
#[cfg_attr(feature = "de", derive(Deserialize))]
#[cfg_attr(
    any(feature = "ser", feature = "de"),
    serde(crate = "near_sdk::serde")
)]
pub struct NftListLog {
    pub list_id: String,
    pub price: U128,
    pub token_key: String,
    pub owner_id: String,
    pub autotransfer: bool,
    pub approval_id: String,
    pub token_id: String,
    pub store_id: String,
}

#[near_event_data(
    standard = "mb_market",
    version = "0.1.0",
    event = "nft_list"
)]
pub struct NftListData(Vec<NftListLog>);

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[near_event_data(
    standard = "mb_market",
    version = "0.1.0",
    event = "nft_update_list"
)]
pub struct NftUpdateListData {
    pub list_id: String,
    pub auto_transfer: Option<bool>,
    pub price: Option<U128>,
}

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[near_event_data_log(
    standard = "mb_market",
    version = "0.1.0",
    event = "nft_unlist"
)]
pub struct NftUnlistLog {
    pub list_id: String,
}

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[near_event_data(
    standard = "mb_market",
    version = "0.1.0",
    event = "nft_sold"
)]
pub struct NftSaleData {
    pub list_id: String,
    pub offer_num: u64,
    pub token_key: String,
    pub payout: HashMap<AccountId, U128>,
    // originally not in 0.1.0, but backwards-comptabile added
    pub mintbase_amount: Option<U128>,
}

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[cfg_attr(feature = "ser", derive(Serialize))]
#[cfg_attr(feature = "de", derive(Deserialize))]
#[cfg_attr(
    any(feature = "ser", feature = "de"),
    serde(crate = "near_sdk::serde")
)]
pub struct NftMakeOfferLog {
    pub offer: TokenOffer, // TODO: TokenOfferJson to stringify u128?
    pub list_id: String,
    pub token_key: String,
    pub offer_num: u64,
}

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[near_event_data(
    standard = "mb_market",
    version = "0.1.0",
    event = "nft_make_offer"
)]
pub struct NftMakeOfferData(Vec<NftMakeOfferLog>);

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[near_event_data(
    standard = "mb_market",
    version = "0.1.0",
    event = "nft_withdraw_offer"
)]
pub struct NftWithdrawOfferData {
    pub list_id: String,
    pub offer_num: u64,
}

// ----------------------- updating banlist/allowlist ----------------------- //
#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[near_event_data(
    standard = "mb_market",
    version = "0.1.0",
    event = "update_banlist"
)]
pub struct UpdateBanlistData {
    pub account_id: String,
    pub state: bool,
}

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[near_event_data(
    standard = "mb_market",
    version = "0.1.0",
    event = "update_allowlist"
)]
pub struct UpdateAllowlistData {
    pub account_id: String,
    pub state: bool,
}
