use near_events::near_event_data;

use crate::types::nft_metadata::NftContractMetadata;

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[near_event_data(standard = "mb_store", version = "0.1.0", event = "deploy")]
pub struct MbStoreDeployData {
    pub contract_metadata: NftContractMetadata,
    pub owner_id: String,
    pub store_id: String,
}

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[near_event_data(
    standard = "mb_store",
    version = "0.1.0",
    event = "change_setting"
)]
pub struct MbStoreChangeSettingData {
    pub granted_minter: Option<String>,
    pub revoked_minter: Option<String>,
    pub new_owner: Option<String>,
    pub new_icon_base64: Option<String>,
    pub new_base_uri: Option<String>,
}
