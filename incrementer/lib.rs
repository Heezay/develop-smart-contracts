#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod incrementer {
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Incrementer {
        /// The incrementer value.
        value: i32,
        /// A mapping of key-value pairs
        my_map: Mapping<AccountId, i32>,
    }

    impl Incrementer {
        /// Create a new Incrementer contract with an initial supply.
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            let mut my_map = Mapping::default();
            let caller = Self::env().caller();
            my_map.insert(&caller, &0);

            Self {
                value: init_value,
                my_map,
            }
        }

        /// Returns default state for new incrementer instance.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                value: 0,
                my_map: Mapping::default(),
            }
        }

        /// Increment value by `i32` predicate. 
        #[ink(message)]
        pub fn inc(&mut self, by: i32) {
            self.value += by;
        }

        /// Allows the contract caller to get the `my_map` storage item and insert an incremented `value` into the mapping.
        #[ink(message)]
        pub fn inc_mine(&mut self, by: i32) {
            let caller = self.env().caller();
            let my_value = self.get_mine();
            self.my_map.insert(caller, &(my_value + by));
        }

        /// Returns the data of `value` with `i32` type.
        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }

        /// Read `my_map` using the Mapping API's `get()` method and return `my_map` for the contract caller.
        #[ink(message)]
        pub fn get_mine(&self) -> i32 {
            let caller = self.env().caller();
            self.my_map.get(&caller).unwrap_or_default()
        }

        /// Allows the contract caller to clear the `my_map` storage item from storage.
        #[ink(message)]
        pub fn remove_mine(&self) {
            let caller = self.env().caller();
            self.my_map.remove(&caller)
        }
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[ink::test]
        fn default_works() {
            let contract = Incrementer::default();
            assert_eq!(contract.get(), 0);
        }

        #[ink::test]
        fn it_works() {
            let mut contract = Incrementer::new(42);
            assert_eq!(contract.get(), 42);
            contract.inc(5);
            assert_eq!(contract.get(), 47);
            contract.inc(-50);
            assert_eq!(contract.get(), -3);
        }

        #[ink::test]
        fn my_map_works() {
            let contract = Incrementer::new(11);
            assert_eq!(contract.get(), 11);
            assert_eq!(contract.get_mine(), 0);
        }

        #[ink::test]
        fn inc_mine_works() {
            let mut contract = Incrementer::new(11);
            assert_eq!(contract.get_mine(), 0);
            contract.inc_mine(5);
            assert_eq!(contract.get_mine(), 5);
            contract.inc_mine(5);
            assert_eq!(contract.get_mine(), 10);
        }

        #[ink::test]
        fn remove_mine_works() {
            let mut contract = Incrementer::new(11);
            assert_eq!(contract.get_mine(), 0);
            contract.inc_mine(5);
            assert_eq!(contract.get_mine(), 5);
            contract.remove_mine();
            assert_eq!(contract.get_mine(), 0);
        }
    }
}
