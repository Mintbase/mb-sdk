use near_events::{
    near_event_data,
    near_event_data_log,
};

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[near_event_data_log(
    standard = "nep141",
    version = "1.0.0",
    event = "ft_mint"
)]
pub struct FtMintLog {
    pub owner_id: String,
    pub amount: String,
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
    pub amount: String,
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
    pub amount: String,
    pub memo: Option<String>,
}
