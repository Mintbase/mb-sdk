use near_events::{
    near_event_data,
    near_event_data_log,
};
#[cfg(feature = "de")]
use near_sdk::serde::Deserialize;
#[cfg(feature = "ser")]
use near_sdk::serde::Serialize;

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[near_event_data_log(
    standard = "nep171",
    version = "1.0.0",
    event = "nft_mint"
)]
pub struct NftMintLog {
    pub owner_id: String,
    pub token_ids: Vec<String>,
    pub memo: Option<String>,
}

// #[near_event_data(standard = "nep171", version = "1.0.0", event = "nft_mint")]
// pub struct NftMintData(Vec<NftMintLog>);

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[near_event_data_log(
    standard = "nep171",
    version = "1.0.0",
    event = "nft_burn"
)]
pub struct NftBurnLog {
    pub owner_id: String,
    pub authorized_id: Option<String>,
    pub token_ids: Vec<String>,
    pub memo: Option<String>,
}

// #[near_event_data(standard = "nep171", version = "1.0.0", event = "nft_burn")]
// pub struct NftBurnData(Vec<NftBurnLog>);

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[cfg_attr(feature = "ser", derive(Serialize))]
#[cfg_attr(feature = "de", derive(Deserialize))]
#[cfg_attr(
    any(feature = "ser", feature = "de"),
    serde(crate = "near_sdk::serde")
)]
pub struct NftTransferLog {
    pub authorized_id: Option<String>,
    pub old_owner_id: String,
    pub new_owner_id: String,
    pub token_ids: Vec<String>,
    pub memo: Option<String>,
}

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[near_event_data(
    standard = "nep171",
    version = "1.0.0",
    event = "nft_transfer"
)]
pub struct NftTransferData(Vec<NftTransferLog>);
