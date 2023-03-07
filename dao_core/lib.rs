#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::contract]
mod dao_core {
    use default_contract::default_contract::{DefaultContract, DefaultContractRef};
    use contract_helper::traits::contract_base::contract_base::contractbase_external::ContractBase;
    use ink::env::call;
    use ink::prelude::string::{String, ToString};
    use ink::prelude::vec::Vec;
    use contract_helper::traits::types::*;

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct DaoCore {
        /// Stores a single `bool` value on the storage.
        value: bool,
        function_a: AccountId,
        function_b: AccountId,
    }

    impl DaoCore {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool, function_a:AccountId, function_b:AccountId) -> Self {
            Self { 
                value: init_value,
                function_a: function_a,
                function_b: function_b,
            }
        }

        #[ink(message)]
        pub fn call_function_a(&mut self) {
            let caller_string = "ajYMsCKsEAhEvHpeA4XqsfiA9v1CdzZPrCfS6pEfeGHW9j8".to_string();
            let mut instance: DefaultContractRef = ink::env::call::FromAccountId::from_account_id(self.function_a);

            match instance.execute_interface("test_a1_function".to_string(),"functiona1,test,".to_string()+&caller_string) {
                Ok(()) => (),
                Err(_) => ink::env::debug_println!("######################### execute_interface Error"),
            }
        }

        #[ink(message)]
        pub fn call_function_b(&mut self) {
            let mut instance: DefaultContractRef = ink::env::call::FromAccountId::from_account_id(self.function_b);
            match instance.execute_interface("test_b1_function".to_string(),"functionb,test".to_string()) {
                Ok(()) => (),
                Err(_) => ink::env::debug_println!("######################### execute_interface Error"),
            }
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
            let dao_core = DaoCore::default();
            assert_eq!(dao_core.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut dao_core = DaoCore::new(false);
            assert_eq!(dao_core.get(), false);
            dao_core.flip();
            assert_eq!(dao_core.get(), true);
        }
    }
}
