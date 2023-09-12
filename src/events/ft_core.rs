use near_events::{
    near_event_data,
    near_event_data_log,
};
use near_sdk::json_types::U128;

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[near_event_data_log(
    standard = "nep141",
    version = "1.0.0",
    event = "ft_mint"
)]
pub struct FtMintLog {
    pub owner_id: String,
    pub amount: U128,
    pub memo: Option<String>,
}

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[near_event_data_log(
    standard = "nep141",
    version = "1.0.0",
    event = "ft_burn"
)]
pub struct FtBurnLog {
    pub owner_id: String,
    pub amount: U128,
    pub memo: Option<String>,
}

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[near_event_data(
    standard = "nep141",
    version = "1.0.0",
    event = "ft_transfer"
)]
pub struct FtTransferLog {
    pub old_owner_id: String,
    pub new_owner_id: String,
    pub amount: U128,
    pub memo: Option<String>,
}
