// We now import the NearToken type
use near_sdk::{env, near, AccountId, NearToken, Promise};

// We define our constant as a NearToken from the start
const TOKEN_CREATION_FEE: NearToken = NearToken::from_yoctonear(100_000_000_000_000_000_000_000); // 0.5 NEAR

#[near(contract_state)]
pub struct Contract {
    pub owner_id: AccountId,
}

impl Default for Contract {
    fn default() -> Self {
        env::panic_str("Contract must be initialized");
    }
}

#[near]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
        }
    }

    #[payable]
    pub fn create_token(
        &mut self,
        token_name: String,
        // We add underscores to the unused variables
        _token_symbol: String,
        _token_icon: String,
        _token_decimals: u8,
    ) {
        let attached_deposit = env::attached_deposit();

        // Now we are comparing two NearToken types, which is correct
        assert!(
            attached_deposit.ge(&TOKEN_CREATION_FEE),
            "Attached deposit must be at least {} yoctoNEAR", TOKEN_CREATION_FEE.as_yoctonear()
        );

        // This comparison also works correctly now
        if attached_deposit.gt(&TOKEN_CREATION_FEE) {
            // And subtraction works too
            let refund = attached_deposit.saturating_sub(TOKEN_CREATION_FEE);
            Promise::new(env::predecessor_account_id()).transfer(refund);
        }

        env::log_str(&format!(
            "Token '{}' created by {}.",
            token_name,
            env::predecessor_account_id()
        ));
    }
}