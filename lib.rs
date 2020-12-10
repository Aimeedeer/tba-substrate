#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod tba_substrate {
    use alloc::string::String;
    
    #[ink(storage)]
    pub struct TbaSubstrate {
        /// Stores a single `bool` value on the storage.
        message: String,
    }

    impl TbaSubstrate {
        #[ink(constructor)]
        pub fn new(init_msg: String) -> Self {
            Self { message: init_msg }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(String::from("Initialize the Big Announcement contract"))
        }

        #[ink(message)]
	#[ink(payable)] 
        pub fn set_message(&mut self, new_msg: String) {
            self.message = new_msg;
        }

        #[ink(message)]
        pub fn get_message(&self) -> String {
            self.message.clone()
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
        #[test]
        fn default_works() {
            let tba_substrate = TbaSubstrate::default();
            assert_eq!(tba_substrate.get_message(), "Initialize the Big Announcement contract");
        }

        /// We test a simple use case of our contract.
        #[test]
        fn it_works() {
            let mut tba_substrate = TbaSubstrate::new(String::from("29D372BE8063788245AC697CBE0546525B64D39092A2DE06BA6FDCA4AEA5EB3F"));
            assert_eq!(tba_substrate.get_message(), "29D372BE8063788245AC697CBE0546525B64D39092A2DE06BA6FDCA4AEA5EB3F");
            tba_substrate.set_message(String::from("Initialize the Big Announcement contract"));
	    assert_eq!(tba_substrate.get_message(), "Initialize the Big Announcement contract");
        }
    }
}
