use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, require, AccountId};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub owner_id: AccountId,
    pub is_initialized: bool,
}

// We need this because AccountId does not have a default value.
impl Default for Contract {
    fn default() -> Self {
        Self {
            owner_id: "system".parse().unwrap(), // Placeholder
            is_initialized: false,
        }
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        require!(!env::state_exists(), "The contract is already initialized");
        Self {
            owner_id: env::predecessor_account_id(),
            is_initialized: true,
        }
    }
}
