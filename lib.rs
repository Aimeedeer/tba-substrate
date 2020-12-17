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
        owner: AccountId,
    }
    
    /// The error types.
    #[derive(Debug, PartialEq, Eq, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if the transfer failed.
        TransferFailed,
        /// Insufficient funds to execute transfer.
        InsufficientFunds,
        /// Transfer failed because it would have brought the contract's
        /// balance below the subsistence threshold.
        /// This is necessary to keep enough funds in the contract to
        /// allow for a tombstone to be created.
        BelowSubsistenceThreshold,
    }
    
    impl TbaSubstrate {
        #[ink(constructor)]
        pub fn new(price: u128) -> Self {
	    let message = String::from("Initialize the Big Announcement contract");
	    let owner = Self::env().caller();
            ink_env::debug_println(&format!("thanks for instantiation {:?}, and price {}", &message, price));
	    
            Self {
		message,
		price,
		owner,
	    }
        }

	#[ink(message, payable)]
	pub fn set_message(&mut self, new_msg: String) {
	    let new_price = self.env().transferred_balance();
	    ink_env::debug_println(&format!("new price {:?}", new_price));

	    if new_price > self.price {
		self.price = new_price;
		self.message = new_msg;
		
		ink_env::debug_println(&format!("Thanks for posting the message {:?}", &self.message));
	    } else {
		ink_env::debug_println(&format!("Your price shoule be greater than the current price: {}.", self.price));
	    }	   
	}
/*
	#[ink(message, payable)]
	pub fn withdraw(&self) -> Result<(), Error> {
            let caller = self.env().caller();
	    if assert_eq!(caller, self.owner) {
		let balance = self.env().balance();
		if balance > 0 {
		    self.env()
			.transfer(self.owner, balance)
			.map_err(|err| {
			    match err {
				ink_env::Error::BelowSubsistenceThreshold => {
				    Error::BelowSubsistenceThreshold
				}
				_ => Error::TransferFailed,
			    }
			})
		} else {
                    return Err(Error::InsufficientFunds)
		}
	    }
	    
	    Ok(())
	}
*/	    
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

        use ink_env::{
            call,
            test,
        };
        use ink_lang as ink;
	
        #[ink::test]
        fn new_works() {
            let tba = TbaSubstrate::new(340282366920938463463374607431768211455);
            assert_eq!(tba.get_message(), "Initialize the Big Announcement contract");
	    assert_eq!(tba.get_price(), 340282366920938463463374607431768211455);
        }

	    
        #[ink::test]
        fn set_works() {
            let accounts = default_accounts();
            let mut tba = create_contract(100);
	    
            set_sender(accounts.eve);
            set_balance(accounts.eve, 9876);

	    println!("alice's balance: {}", get_balance(accounts.alice));
	    assert_eq!(get_balance(accounts.eve), 9876);

	    tba.set_message("new message".to_string());
	    
	    assert_eq!(tba.get_message(), "new message");
    	    assert_eq!(tba.get_price(), 1234);

	    println!("eve's balance: {}", get_balance(accounts.eve));
	    println!("alice's balance after: {}", get_balance(accounts.alice));

        }
	
        fn default_accounts(
        ) -> ink_env::test::DefaultAccounts<ink_env::DefaultEnvironment> {
            ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Off-chain environment should have been initialized already")
        }

        fn set_balance(account_id: AccountId, balance: Balance) {
            ink_env::test::set_account_balance::<ink_env::DefaultEnvironment>(
                account_id, balance,
            )
            .expect("Cannot set account balance");
        }

        fn get_balance(account_id: AccountId) -> Balance {
            ink_env::test::get_account_balance::<ink_env::DefaultEnvironment>(account_id)
                .expect("Cannot set account balance")
        }
	
        fn create_contract(initial_balance: Balance) -> TbaSubstrate {
            let accounts = default_accounts();
            set_sender(accounts.alice);
            set_balance(contract_id(), initial_balance);

            TbaSubstrate::new(1)
        }
	
        fn contract_id() -> AccountId {
            ink_env::test::get_current_contract_account_id::<ink_env::DefaultEnvironment>(
            )
            .expect("Cannot get contract id")
        }
	
        fn set_sender(sender: AccountId) {
            let callee = ink_env::account_id::<ink_env::DefaultEnvironment>()
                .unwrap_or([0x0; 32].into());
            test::push_execution_context::<Environment>(
                sender,
                callee,
                1000000,
                1234,
                test::CallData::new(call::Selector::new([0x00; 4])), // dummy
            );
        }
	
	/*
        #[ink::test]
        fn withdraw_works() -> Result<(), ink_env::Error> {
            let caller: AccountId = self.env().caller();

	    let mut tba = TbaSubstrate::new(1122334455);
            assert_eq!(tba.withdraw(), Ok(()));

            // then
            assert_eq!(get_balance(accounts.eve), 0);
        }
	 */

    } 
}

