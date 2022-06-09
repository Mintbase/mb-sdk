#[cfg(feature = "de")]
use near_sdk::serde::Deserialize;
#[cfg(feature = "ser")]
use near_sdk::serde::Serialize;

#[cfg_attr(feature = "all", derive(Clone, Debug))]
#[cfg_attr(feature = "ser", derive(Serialize))]
#[cfg_attr(feature = "de", derive(Deserialize))]
#[cfg_attr(
    any(feature = "ser", feature = "de"),
    serde(crate = "near_sdk::serde")
)]
pub struct TokenOffer {
    /// The id of this `Offer` is the num of the previous `Offer` + 1. Generated
    /// from the field `Token::num_offers`.
    pub id: u64,
    /// The price the Offerer has posted.
    pub price: u128,
    /// The account who originated the `Offer`.
    pub from: near_sdk::AccountId,
    /// When the `Offer` was made.
    pub timestamp: u64, // NearTime in original definition
    /// When the `Offer` will expire.
    pub timeout: u64, // NearTime in original definition
}
