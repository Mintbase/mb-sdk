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

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[cfg_attr(feature = "ser", derive(Serialize))]
#[cfg_attr(feature = "de", derive(Deserialize))]
#[cfg_attr(
    any(feature = "ser", feature = "de"),
    serde(crate = "near_sdk::serde")
)]
pub struct NftApproveLog {
    pub token_id: String,
    pub approval_id: u64,
    pub account_id: String,
}

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[near_event_data(
    standard = "mb_store",
    version = "0.1.0",
    event = "nft_approve"
)]
pub struct NftApproveData(Vec<NftApproveLog>);

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[near_event_data(
    standard = "mb_store",
    version = "0.1.0",
    event = "nft_revoke"
)]
pub struct NftRevokeData {
    pub token_id: String,
    pub account_id: String,
}

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[near_event_data(
    standard = "mb_store",
    version = "0.1.0",
    event = "nft_revoke_all"
)]
pub struct NftRevokeAllData {
    pub token_id: String,
}
