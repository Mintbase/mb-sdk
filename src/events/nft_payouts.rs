use near_events::near_event_data;
use near_sdk::AccountId;

#[cfg_attr(feature = "all", derive(Debug, Clone))]
#[near_event_data(
    standard = "mb_store",
    version = "0.1.0",
    event = "nft_set_split_owners"
)]
pub struct NftSetSplitOwnerData {
    pub token_ids: Vec<String>,
    pub split_owners: std::collections::HashMap<AccountId, u16>,
}
