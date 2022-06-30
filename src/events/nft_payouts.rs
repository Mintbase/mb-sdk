use near_events::near_event_data;
#[cfg(feature = "de")]
use near_sdk::serde::Deserialize;
#[cfg(feature = "ser")]
use near_sdk::serde::Serialize;
use near_sdk::{
    env,
    json_types::U64,
    AccountId,
};

#[cfg_attr(feature = "all", derive(Debug, Clone))]
#[near_event_data(
    standard = "mb_store",
    version = "0.1.0",
    event = "nft_set_split_owners"
)]
pub struct NftSetSplitOwnerData {
    pub token_ids: Vec<U64>,
    pub split_owners: HashMap<AccountId, u16>,
}
