use std::collections::HashMap;

use near_sdk::AccountId;

// TODO: there should probably be a v2 of this message
#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[cfg_attr(feature = "ser", derive(near_sdk::serde::Serialize))]
#[cfg_attr(feature = "de", derive(near_sdk::serde::Deserialize))]
#[cfg_attr(
    any(feature = "ser", feature = "de"),
    serde(crate = "near_sdk::serde")
)]
pub struct MintbaseMintMemo {
    pub royalty: Option<Royalty>,
    pub split_owners: Option<SplitOwners>,
    // not needed, queried via RPC call
    pub meta_id: Option<String>,
    // not needed, queried via RPC call
    pub meta_extra: Option<String>,
    // not needed, inferred by tx caller
    pub minter: String,
}

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[cfg_attr(feature = "ser", derive(near_sdk::serde::Serialize))]
#[cfg_attr(feature = "de", derive(near_sdk::serde::Deserialize))]
#[cfg_attr(
    any(feature = "ser", feature = "de"),
    serde(crate = "near_sdk::serde")
)]
pub struct Royalty {
    pub split_between: HashMap<AccountId, SafeFraction>,
    pub percentage: SafeFraction,
}

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[cfg_attr(feature = "ser", derive(near_sdk::serde::Serialize))]
#[cfg_attr(feature = "de", derive(near_sdk::serde::Deserialize))]
#[cfg_attr(
    any(feature = "ser", feature = "de"),
    serde(crate = "near_sdk::serde")
)]
pub struct SplitOwners {
    pub split_between: HashMap<AccountId, SafeFraction>,
}

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[cfg_attr(feature = "ser", derive(near_sdk::serde::Serialize))]
#[cfg_attr(feature = "de", derive(near_sdk::serde::Deserialize))]
#[cfg_attr(
    any(feature = "ser", feature = "de"),
    serde(crate = "near_sdk::serde")
)]
pub struct SafeFraction {
    pub numerator: u32,
}
