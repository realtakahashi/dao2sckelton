use ink::storage::traits::StorageLayout;
use openbrush::traits::AccountId;
use ink::prelude::string::{String};


#[derive(Debug, Clone, scale::Encode, scale::Decode, PartialEq)]
#[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
pub enum CallerCheckSpecs {
    Owner,
    DaoMemeber,
    ElectoralCommissioner,
}

#[derive(Debug, Clone, scale::Encode, scale::Decode, PartialEq)]
#[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
pub struct AInfo {
    pub id: u128,
    pub string_data: String,
    pub target_AccountId: AccountId,
}

#[derive(Debug, Clone, scale::Encode, scale::Decode, PartialEq)]
#[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
pub struct BInfo {
    pub id: u128,
    pub title: String,
    pub outline: String,
    pub description: String,
    pub github_url: String,
    pub target_function: String,
    pub parameters: String,
}

