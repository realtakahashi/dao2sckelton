#![cfg_attr(not(feature = "std"), no_std)]

pub use self::function_a::{FunctionA, FunctionARef};

#[openbrush::contract]
mod function_a {
    // use communication_base::communication_base::{CommunicationBase, CommunicationBaseRef};
    use contract_helper::common::common_logics::*;
    use contract_helper::traits::contract_base::contract_base::*;
    use contract_helper::traits::types::types::*;
    use default_contract::default_contract::{DefaultContract, DefaultContractRef};
    use ink::prelude::string::{String, ToString};
    use ink::prelude::vec::Vec;
    use ink::storage::traits::StorageLayout;
    use openbrush::storage::Mapping;
    use scale::{Decode, Encode};

    #[ink(storage)]
    pub struct FunctionA {
        list_of_a: Mapping<u128, AInfo>,
        dao_address: Option<AccountId>,
        command_list: Vec<String>,
        next_index: u128,
        // communication_base_address: AccountId,
    }

    impl ContractBase for FunctionA {
        #[ink(message)]
        fn get_dao_address(&self) -> Option<AccountId> {
            self.dao_address
        }

        #[ink(message)]
        fn get_caller_check_specs(&self, command: String) -> Option<CallerCheckSpecs> {
            match command.as_str() {
                "test_a1_function" => Some(CallerCheckSpecs::DaoMemeber),
                _ => None,
            }
        }

        #[ink(message)]
        fn get_data(&self, target_function: String) -> Vec<Vec<u8>> {
            let mut result: Vec<Vec<u8>> = Vec::new();
            match target_function.as_str() {
                "get_list_of_a_value" => {
                    let list: Vec<AInfo> = self.get_list_of_a_value();
                    for value in list.iter() {
                        result.push(value.encode());
                    }
                }
                _ => (),
            }
            result
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
            vec_of_parameters: Vec<String>,
        ) -> core::result::Result<(), ContractBaseError> {
            match command.as_str() {
                "test_a1_function" => self._test_a1_function(vec_of_parameters),
                "test_a2_function" => self._test_a2_function(vec_of_parameters),
                _ => Err(ContractBaseError::CommnadNotFound),
            }
        }
    }

    impl FunctionA {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                list_of_a: Mapping::default(),
                dao_address: None,
                command_list: [
                    "test_a1_function".to_string(),
                    "test_a2_function".to_string(),
                ]
                .to_vec(),
                next_index: 0,
                // communication_base_address: communication_base_address,
            }
        }

        #[ink(message)]
        pub fn extarnal_get_data_interface(&self,target_function:String) -> Vec<Vec<u8>> {
            self.get_data(target_function)
        }

        #[ink(message)]
        pub fn extarnal_execute_interface(&mut self, command:String, parameters_csv:String) -> core::result::Result<(), ContractBaseError>{
            self._execute_interface(command, parameters_csv)
        }

        #[ink(message)]
        pub fn call_other_contract(&mut self, target_contract :AccountId, target_function:String,parameters_csv:String) -> core::result::Result<(), ContractBaseError>{
            let mut instance: DefaultContractRef =
                ink::env::call::FromAccountId::from_account_id(target_contract);
            instance.extarnal_execute_interface(target_function, parameters_csv)
        }

        #[ink(message)]
        pub fn get_list_of_a_value(&self) -> Vec<AInfo> {
            let mut result: Vec<AInfo> = Vec::new();
            for i in 0..self.next_index {
                match self.list_of_a.get(&i) {
                    Some(value) => result.push(value.clone()),
                    None => (),
                }
            }
            result
        }

        #[ink(message)]
        pub fn get_functionb_value(&self, function_b_address: AccountId) -> Vec<BInfo> {
            // let mut instance: CommunicationBaseRef =
            //     ink::env::call::FromAccountId::from_account_id(self.communication_base_address);
            // let get_value: Vec<Vec<u8>> = instance
            //     .get_data_from_contract(function_b_address, "get_list_of_b_value".to_string());

            let instance: DefaultContractRef =
                ink::env::call::FromAccountId::from_account_id(function_b_address);
            let get_value: Vec<Vec<u8>> = instance.extarnal_get_data_interface("get_list_of_b_value".to_string());

            let mut return_value: Vec<BInfo> = Vec::new();
            for value in get_value.iter() {
                let mut array_value: &[u8] = value.as_slice().try_into().unwrap();
                let decode_value = match BInfo::decode(&mut array_value.clone()) {
                    Ok(value) => return_value.push(value),
                    Err(_) => (),
                };
            }
            return_value
        }

        fn _test_a1_function(
            &mut self,
            vec_of_parameters: Vec<String>,
        ) -> core::result::Result<(), ContractBaseError> {
            ink::env::debug_println!("######################### value is {:?}", vec_of_parameters);
            let account_id: AccountId = convert_string_to_accountid(&vec_of_parameters[2]);
            let a_info: AInfo = AInfo {
                id: self.next_index,
                string_data: vec_of_parameters[0].clone(),
                target_AccountId: account_id,
            };
            self.list_of_a.insert(&self.next_index, &a_info);
            self.next_index += 1;
            Ok(())
        }

        fn _test_a2_function(
            &mut self,
            vec_of_parameters: Vec<String>,
        ) -> core::result::Result<(), ContractBaseError> {
            ink::env::debug_println!("######################### value is {:?}", vec_of_parameters);
            let account_id: AccountId = convert_string_to_accountid(&vec_of_parameters[2]);
            let a_info: AInfo = AInfo {
                id: self.next_index,
                string_data: vec_of_parameters[0].clone(),
                target_AccountId: account_id,
            };
            self.list_of_a.insert(&self.next_index, &a_info);
            self.next_index += 1;
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
            let function_a = FunctionA::default();
            assert_eq!(function_a.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut function_a = FunctionA::new(false);
            assert_eq!(function_a.get(), false);
            function_a.flip();
            assert_eq!(function_a.get(), true);
        }
    }
}
