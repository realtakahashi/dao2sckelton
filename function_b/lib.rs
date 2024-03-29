#![cfg_attr(not(feature = "std"), no_std)]

pub use self::function_b::{FunctionB, FunctionBRef};

#[openbrush::contract]
mod function_b {
    // use communication_base::communication_base::{CommunicationBase, CommunicationBaseRef};
    use contract_helper::traits::contract_base::contract_base::*;
    use contract_helper::traits::types::types::*;
    use ink::prelude::string::{String, ToString};
    use ink::prelude::vec::Vec;
    use ink::storage::traits::StorageLayout;
    use openbrush::storage::Mapping;
    use scale::{Decode, Encode};
    use default_contract::default_contract::{DefaultContract, DefaultContractRef};

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct FunctionB {
        /// Stores a single `bool` value on the storage.
        value: bool,
        list_of_b: Mapping<u128, BInfo>,
        dao_address: Option<AccountId>,
        command_list: Vec<String>,
        next_id: u128,
        // communication_base_address: AccountId,
    }

    impl ContractBase for FunctionB {
        #[ink(message)]
        fn get_dao_address(&self) -> Option<AccountId> {
            self.dao_address
        }

        #[ink(message)]
        fn get_caller_check_specs(&self, command: String) -> Option<CallerCheckSpecs> {
            match command.as_str() {
                "test_b1_function" => Some(CallerCheckSpecs::DaoMemeber),
                "test_b2_function" => Some(CallerCheckSpecs::DaoMemeber),
                "test_b3_function" => Some(CallerCheckSpecs::DaoMemeber),
                "test_b4_function" => Some(CallerCheckSpecs::DaoMemeber),
                _ => None,
            }
        }

        #[ink(message)]
        fn get_data(&self, target_function: String) -> Vec<Vec<u8>> {
            let mut result: Vec<Vec<u8>> = Vec::new();
            match target_function.as_str() {
                "get_list_of_b_value" => {
                    let list: Vec<BInfo> = self.get_list_of_b_value();
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
                "test_b1_function" => self._test_b1_function(vec_of_parameters),
                "test_b2_function" => self._test_b2_function(vec_of_parameters),
                "test_b3_function" => self._test_b3_function(vec_of_parameters),
                "test_b4_function" => self._test_b4_function(vec_of_parameters),
                _ => Err(ContractBaseError::CommnadNotFound),
            }
        }
    }

    impl FunctionB {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        // pub fn new(init_value: bool, communication_base_address: AccountId) -> Self {
        pub fn new(init_value: bool) -> Self {
            Self {
                value: init_value,
                list_of_b: Mapping::default(),
                dao_address: None,
                command_list: [
                    "test_b1_function".to_string(),
                    "test_b2_function".to_string(),
                    "test_b3_function".to_string(),
                    "test_b4_function".to_string(),
                ]
                .to_vec(),
                next_id: 0,
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

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        #[ink(message)]
        pub fn get_list_of_b_value(&self) -> Vec<BInfo> {
            let mut return_value: Vec<BInfo> = Vec::new();
            for i in 0..self.next_id {
                match self.list_of_b.get(&i) {
                    Some(value) => return_value.push(value),
                    None => (),
                }
            }
            return_value
        }

        #[ink(message)]
        pub fn call_other_contract(&mut self, target_contract :AccountId, target_function:String,parameters_csv:String) -> core::result::Result<(), ContractBaseError>{
            let mut instance: DefaultContractRef =
                ink::env::call::FromAccountId::from_account_id(target_contract);
            instance.extarnal_execute_interface(target_function, parameters_csv)
        }

        #[ink(message)]
        pub fn add_function_a_value(
            &mut self,
            function_a_address: AccountId,
        ) -> core::result::Result<(), ContractBaseError> {
            let caller_string = "ajYMsCKsEAhEvHpeA4XqsfiA9v1CdzZPrCfS6pEfeGHW9j8".to_string();

            let mut instance: DefaultContractRef =
                ink::env::call::FromAccountId::from_account_id(function_a_address);
            instance.extarnal_execute_interface("test_a1_function".to_string(), "functiona1,test,".to_string() + &caller_string)

            // let mut instance: CommunicationBaseRef =
            //     ink::env::call::FromAccountId::from_account_id(self.communication_base_address);
            // instance.call_execute_interface_of_function(
            //     function_a_address,
            //     "test_a1_function".to_string(),
            //     "functiona1,test,".to_string() + &caller_string,
            // )
        }

        #[ink(message)]
        pub fn get_functiona_value(&self, function_a_address: AccountId) -> Vec<AInfo> {
            // let instance: CommunicationBaseRef =
            //     ink::env::call::FromAccountId::from_account_id(self.communication_base_address);                
            // let get_value: Vec<Vec<u8>> = instance
            //     .get_data_from_contract(function_a_address, "get_list_of_a_value".to_string());

            let instance: DefaultContractRef =
            ink::env::call::FromAccountId::from_account_id(function_a_address);
            let get_value: Vec<Vec<u8>> = instance.extarnal_get_data_interface("get_list_of_a_value".to_string());

            let mut return_value: Vec<AInfo> = Vec::new();
            for value in get_value.iter() {
                let array_value: &[u8] = value.as_slice().try_into().unwrap();
                match AInfo::decode(&mut array_value.clone()) {
                    Ok(value) => return_value.push(value),
                    Err(_) => (),
                };
            }
            return_value
        }

        fn _test_b1_function(
            &mut self,
            vec_of_parameters: Vec<String>,
        ) -> core::result::Result<(), ContractBaseError> {
            ink::env::debug_println!("######################### value is {:?}", vec_of_parameters);
            let test_info: BInfo = BInfo {
                id: self.next_id,
                title: vec_of_parameters[0].clone(),
                outline: "outline".to_string(),
                description: "descritpion".to_string(),
                github_url: "githurl".to_string(),
                target_function: "function".to_string(),
                parameters: "param".to_string(),
            };
            self.list_of_b.insert(&self.next_id, &test_info);
            self.next_id += 1;
            Ok(())
        }

        fn _test_b2_function(
            &mut self,
            vec_of_parameters: Vec<String>,
        ) -> core::result::Result<(), ContractBaseError> {
            ink::env::debug_println!("######################### value is {:?}", vec_of_parameters);
            let test_info: BInfo = BInfo {
                id: self.next_id,
                title: "title2".to_string(),
                outline: vec_of_parameters[0].clone(),
                description: "descritpion".to_string(),
                github_url: "githurl".to_string(),
                target_function: "function".to_string(),
                parameters: "param".to_string(),
            };
            self.list_of_b.insert(&self.next_id, &test_info);
            self.next_id += 1;
            Ok(())
        }

        fn _test_b3_function(
            &mut self,
            vec_of_parameters: Vec<String>,
        ) -> core::result::Result<(), ContractBaseError> {
            ink::env::debug_println!("######################### value is {:?}", vec_of_parameters);
            let test_info: BInfo = BInfo {
                id: self.next_id,
                title: "title3".to_string(),
                outline: vec_of_parameters[0].clone(),
                description: "descritpion".to_string(),
                github_url: "githurl".to_string(),
                target_function: "function".to_string(),
                parameters: "param".to_string(),
            };
            self.list_of_b.insert(&self.next_id, &test_info);
            self.next_id += 1;
            Ok(())
        }

        fn _test_b4_function(
            &mut self,
            vec_of_parameters: Vec<String>,
        ) -> core::result::Result<(), ContractBaseError> {
            ink::env::debug_println!("######################### value is {:?}", vec_of_parameters);
            let test_info: BInfo = BInfo {
                id: self.next_id,
                title: "title4".to_string(),
                outline: vec_of_parameters[0].clone(),
                description: "descritpion".to_string(),
                github_url: "githurl".to_string(),
                target_function: "function".to_string(),
                parameters: "param".to_string(),
            };
            self.list_of_b.insert(&self.next_id, &test_info);
            self.next_id += 1;
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
            let function_b = FunctionB::default();
            assert_eq!(function_b.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut function_b = FunctionB::new(false);
            assert_eq!(function_b.get(), false);
            function_b.flip();
            assert_eq!(function_b.get(), true);
        }
    }
}
