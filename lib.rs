#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod tba_substrate {
    use alloc::{string::String, format};
    
    #[ink(storage)]
    pub struct TbaSubstrate {
        message: String,
	price: u128,
    }

    impl TbaSubstrate {
        #[ink(constructor)]
        pub fn new(price: u128) -> Self {
	    let message = String::from("Initialize the Big Announcement contract"); 
            ink_env::debug_println(&format!("thanks for instantiation {:?}, and price {}", &message, price));
	    
            Self {
		message,
		price,
	    }
        }

	#[ink(message, payable)]
	pub fn set_message(&mut self, new_msg: String) {
	    let new_price = 99999999999999;
	    
	    self.price = new_price;
	    self.message = new_msg;
	    
	    ink_env::debug_println(&format!("Thanks for posting the message {:?}", &self.message));

	    /*
	    let new_price = self.env().transferred_balance();

	    if new_price > self.price {
		self.price = new_price;
		self.message = new_msg;
		
		ink_env::debug_println(&format!("Thanks for posting the message {:?}", &self.message));
	    } else {
		ink_env::debug_println(&format!("You should pay a greater price than current one {}", self.price));
	    }	   
	    */
	}
	    
        #[ink(message)]
        pub fn get_message(&self) -> String {
            self.message.clone()
        }

        #[ink(message)]
	pub fn get_price(&self) -> u128 {
	    self.price
	}
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        #[test]
        fn new_works() {
            let tba = TbaSubstrate::new(340282366920938463463374607431768211455);
            assert_eq!(tba.get_message(), "Initialize the Big Announcement contract");
	    assert_eq!(tba.get_price(), 340282366920938463463374607431768211455);
        }

        #[test]
        fn set_works() {
            let mut tba = TbaSubstrate::new(1122334455);
            tba.set_message("new message".to_string());
	    assert_eq!(tba.get_message(), "new message");
    	    assert_eq!(tba.get_price(), 99999999999999);
        } 
    } 
}

