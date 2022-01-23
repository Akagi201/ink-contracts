#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod incrementer {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Incrementer {
        /// Stores a single `i32` value on the storage.
        value: i32,
        my_value: ink_storage::collections::HashMap<AccountId, i32>,
    }

    impl Incrementer {
        /// Constructor that initializes the `i32` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            Self {
                value: init_value,
                my_value: ink_storage::collections::HashMap::new(),
            }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }

        #[ink(message)]
        pub fn inc(&mut self, by: i32) {
            self.value += by;
        }

        #[ink(message)]
        pub fn get_mine(&self) -> i32 {
            self.my_value_or_zero(&self.env().caller())
        }

        fn my_value_or_zero(&self, of: &AccountId) -> i32 {
            *self.my_value.get(of).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn inc_mine(&mut self, by: i32) {
            let caller = self.env().caller();
            let current_value = self.my_value_or_zero(&caller);
            self.my_value.insert(caller, current_value + by);
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let incrementer = Incrementer::default();
            assert_eq!(incrementer.get(), 0);
        }

        #[ink::test]
        fn it_works() {
            let mut incrementer = Incrementer::new(10);
            assert_eq!(incrementer.get(), 10);
            incrementer.inc(5);
            assert_eq!(incrementer.get(), 15);
        }

        #[ink::test]
        fn my_value_works() {
            let mut incrementer = Incrementer::new(11);
            assert_eq!(incrementer.get(), 11);
            assert_eq!(incrementer.get_mine(), 0);
            incrementer.inc_mine(5);
            assert_eq!(incrementer.get_mine(), 5);
            incrementer.inc_mine(10);
            assert_eq!(incrementer.get_mine(), 15);
        }
    }
}
