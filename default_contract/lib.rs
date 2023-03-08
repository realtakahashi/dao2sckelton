#![cfg_attr(not(feature = "std"), no_std)]

pub use self::default_contract::{DefaultContract, DefaultContractRef};

#[openbrush::contract]
pub mod default_contract {
    use contract_helper::traits::contract_base::contract_base::*;
    use ink::prelude::string::{String,ToString};
    use ink::prelude::vec::Vec;

    #[ink(storage)]
    pub struct DefaultContract {
        dao_address: Option<AccountId>,
        command_list: Vec<String>,
    }

    impl ContractBase for DefaultContract {
        #[ink(message)]
        fn get_dao_address(&self) -> Option<AccountId> {
            self.dao_address
        }

        #[ink(message)]
        fn get_caller_check_specs(&self, command:String) -> Option<CallerCheckSpecs>{
            match command.as_str() {
                "test_function" => Some(CallerCheckSpecs::DaoMemeber),
                _ => None,
            }
        }

        #[ink(message)]
        fn get_data(&self,target_function:String) -> Vec<Vec<u8>> {
            let return_value:Vec<Vec<u8>> = Vec::new();
            return_value
        }

        fn _set_dao_address_impl(
            &mut self,
            dao_address: AccountId,
        ) -> core::result::Result<(), ContractBaseError> {
            self.dao_address = Some(dao_address);
            Ok(())
        }

        fn _get_command_list(&self) -> &Vec<String> {
            &self.command_list
        }

        fn _function_calling_switch(
            &mut self,
            command: String,
            _vec_of_parameters: Vec<String>,
        ) -> core::result::Result<(), ContractBaseError> {
            match command.as_str() {
                "test_function" => self._test_function(),
                _ => Err(ContractBaseError::CommnadNotFound),
            }
        }

    }

    impl DefaultContract {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                dao_address: None,
                command_list: [
                    "test_function".to_string(),
                ].to_vec(),            
            }
        }

        fn _test_function(&self) -> core::result::Result<(), ContractBaseError> {
            ink::env::debug_println!("########## source caller ############### value is {:?}", self.env().caller());
            Ok(())
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
            let contract_base = ContractBase::default();
            assert_eq!(contract_base.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut contract_base = ContractBase::new(false);
            assert_eq!(contract_base.get(), false);
            contract_base.flip();
            assert_eq!(contract_base.get(), true);
        }
    }
}
