use near_events::near_event_data;
#[cfg(feature = "de")]
use near_sdk::serde::Deserialize;
#[cfg(feature = "ser")]
use near_sdk::serde::Serialize;
use near_sdk::{
    json_types::U128,
    AccountId,
};

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[near_event_data(
    standard = "mb_market",
    version = "0.2.0",
    event = "nft_list"
)]
pub struct NftListData {
    pub kind: String,
    pub nft_contract_id: AccountId,
    pub nft_token_ids: Vec<String>,
    pub nft_approval_ids: Vec<u64>,
    pub nft_owner_id: AccountId,
    pub price: U128,
    // pub ft_contract: Option<AccountId>,
    // pub ft_amount: Balance,
}

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[near_event_data(
    standard = "mb_market",
    version = "0.2.0",
    event = "nft_unlist"
)]
pub struct NftUnlistData {
    pub nft_contract_id: AccountId,
    pub nft_token_ids: Vec<String>,
}

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[cfg_attr(feature = "ser", derive(Serialize))]
#[cfg_attr(feature = "de", derive(Deserialize))]
#[cfg_attr(
    any(feature = "ser", feature = "de"),
    serde(crate = "near_sdk::serde")
)]
pub struct NftSaleLog {
    pub nft_contract_id: AccountId,
    pub nft_token_id: String,
}

#[near_event_data(
    standard = "mb_market",
    version = "0.2.0",
    event = "nft_sale"
)]
pub struct NftSaleData(Vec<NftSaleLog>);

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[near_event_data(
    standard = "mb_market",
    version = "0.2.0",
    event = "nft_make_offer"
)]
pub struct NftMakeOfferData {
    pub nft_contract_id: AccountId,
    pub nft_token_id: String,
    pub offer_id: u64,
    pub price: U128,
}

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[near_event_data(
    standard = "mb_market",
    version = "0.2.0",
    event = "nft_withdraw_offer"
)]
pub struct NftWithdrawOfferData {
    pub nft_contract_id: AccountId,
    pub nft_token_id: String,
    pub offer_id: u64,
}
