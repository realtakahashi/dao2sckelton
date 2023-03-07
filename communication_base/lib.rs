#![cfg_attr(not(feature = "std"), no_std)]

pub use self::communication_base::{CommunicationBase, CommunicationBaseRef};

#[ink::contract]
pub mod communication_base {
    use default_contract::default_contract::{DefaultContract, DefaultContractRef};
    use contract_helper::traits::contract_base::contract_base::contractbase_external::ContractBase;
    use ink::{prelude::string::String};
    use ink::prelude::vec::Vec;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct CommunicationBase {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }

    impl CommunicationBase {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        #[ink(message)]
        pub fn get_data_from_contract(&self, target_contract:AccountId, target_function:String) -> Vec<Vec<u8>> {
            let mut instance: DefaultContractRef = ink::env::call::FromAccountId::from_account_id(target_contract);
            instance.get_data(target_function)
        }

    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let communication_base = CommunicationBase::default();
            assert_eq!(communication_base.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut communication_base = CommunicationBase::new(false);
            assert_eq!(communication_base.get(), false);
            communication_base.flip();
            assert_eq!(communication_base.get(), true);
        }
    }
}
