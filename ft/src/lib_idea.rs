/*!
Fungible Token implementation with JSON serialization.
NOTES:
  - The maximum balance value is limited by U128 (2**128 - 1).
  - JSON calls should pass U128 as a base-10 string. E.g. "100".
  - The contract optimizes the inner trie structure by hashing account IDs. It will prevent some
    abuse of deep tries. Shouldn't be an issue, once NEAR clients implement full hashing of keys.
  - The contract tracks the change in storage before and after the call. If the storage increases,
    the contract requires the caller of the contract to attach enough deposit to the function call
    to cover the storage cost.
    This is done to prevent a denial of service attack on the contract by taking all available storage.
    If the storage decreases, the contract will issue a refund for the cost of the released storage.
    The unused tokens from the attached deposit are also refunded, so it's safe to
    attach more deposit than required.
  - To prevent the deployed contract from being modified or deleted, it should not have any access
    keys on its account.
*/
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;
use near_sdk::{env, log, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner: AccountId,
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNTEyIiBoZWlnaHQ9IjUxMiIgdmlld0JveD0iMCAwIDUxMiA1MTIiIGZpbGw9Im5vbmUiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+CjxjaXJjbGUgY3g9IjI1NiIgY3k9IjI1NiIgcj0iMjU2IiBmaWxsPSIjRkZCOTUxIi8+CjxwYXRoIGQ9Ik0yMjAgMzQzLjA5NFYzMzcuNzE5TDIxNS45OSAzMzQuMTQxQzIwMC45NCAzMjAuNzA4IDE4My42NTQgMzAyLjQgMTY5LjAyMyAyODEuNDY5QzE1Mi4zMiAyNTcuNTczIDE0MCAyMzIuNjYzIDE0MCAyMDhDMTQwIDE3Ni4zMzQgMTUzLjQyNCAxNTAuNDIxIDE3NC4yOTMgMTMzLjY3MUMxOTUuNDQ5IDExNi42OSAyMjQuMjA0IDEwOCAyNTYgMTA4QzI4Ny43OTYgMTA4IDMxNi41NTEgMTE2LjY5IDMzNy43MDcgMTMzLjY3MUMzNTguNTc2IDE1MC40MjEgMzcyIDE3Ni4zMzQgMzcyIDIwOEMzNzIgMjMyLjY2MyAzNTkuNjggMjU3LjU3MyAzNDIuOTc3IDI4MS40NjlDMzI4LjM0NiAzMDIuNCAzMTEuMDYgMzIwLjcwOCAyOTYuMDEgMzM0LjE0MUwyOTIgMzM3LjcxOVYzNDMuMDk0VjM4NEMyOTIgMzk0Ljg2MiAyODIuODYyIDQwNCAyNzIgNDA0SDI0MEMyMjkuMTM4IDQwNCAyMjAgMzk0Ljg2MiAyMjAgMzg0VjM0My4wOTRaTTI4MS4xMjUgMzMySDI4NS42NDRMMjg5LjA0MSAzMjkuMDE5QzMwMy41NTQgMzE2LjI4MSAzMjEuNjc0IDI5Ny45ODQgMzM2LjQyOSAyNzYuODc1QzM1MS45NDMgMjU0LjY4MSAzNjQgMjI4Ljc1OCAzNjQgMjA4QzM2NCAxNzcuODA3IDM1Mi4zNjQgMTU1LjcwNyAzMzIuNjk5IDEzOS45MjNDMzEzLjMxOCAxMjQuMzY3IDI4Ni4wNTcgMTE2IDI1NiAxMTZDMjI1Ljk0MyAxMTYgMTk4LjY4MiAxMjQuMzY3IDE3OS4zMDEgMTM5LjkyM0MxNTkuNjM2IDE1NS43MDcgMTQ4IDE3Ny44MDcgMTQ4IDIwOEMxNDggMjI4Ljc1OCAxNjAuMDU3IDI1NC42ODEgMTc1LjU3MSAyNzYuODc1QzE5MC4zMjYgMjk3Ljk4NCAyMDguNDQ2IDMxNi4yODEgMjIyLjk1OSAzMjkuMDE5TDIyNi4zNTYgMzMySDIzMC44NzVIMjgxLjEyNVoiIGZpbGw9IiNGRkI5NTEiIHN0cm9rZT0iYmxhY2siIHN0cm9rZS13aWR0aD0iMjQiLz4KPHBhdGggZD0iTTIwOCAyNzJWMjI0TDI1NiAyNzJMMzA0IDIyNFYyNzIiIHN0cm9rZT0iYmxhY2siIHN0cm9rZS13aWR0aD0iMjQiLz4KPC9zdmc+Cg==";

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(owner_id: AccountId, _total_supply: U128) -> Self {
        Self::new(
            owner_id,
            // total_supply,
            U128::from(0),
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "INIT Token".to_string(),
                // symbol: "IDEA".to_string(),
                symbol: "INIT".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 24,
            },
        )
    }

    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(
        owner_id: AccountId,
        total_supply: U128,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        let mut this = Self {
            owner: owner_id,
            token: FungibleToken::new(b"a".to_vec()),
            metadata: LazyOption::new(b"m".to_vec(), Some(&metadata)),
        };
        this.token.internal_register_account(&this.owner);
        this.token.internal_deposit(&this.owner, total_supply.into());
        near_contract_standards::fungible_token::events::FtMint {
            owner_id: &this.owner,
            amount: &total_supply,
            memo: Some("Initial tokens supply is minted"),
        }
        .emit();
        this
    }

    pub fn mint(
        &mut self,
        to: AccountId,
        amount: U128,
    ) {
        // get first account from token.accounts
        let owner = &self.owner;
        // check if owner equals callee
        assert_eq!(owner, &env::signer_account_id(), "Only owner can mint");

        self.token.internal_deposit(&to, amount.into());
        near_contract_standards::fungible_token::events::FtMint {
            owner_id: &to,
            amount: &amount,
            memo: Some("Minted"),
        }
        .emit();
    }

    fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
        log!("Closed @{} with {}", account_id, balance);
    }

    fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
        log!("Account @{} burned {}", account_id, amount);
    }
}

near_contract_standards::impl_fungible_token_core!(Contract, token, on_tokens_burned);
near_contract_standards::impl_fungible_token_storage!(Contract, token, on_account_closed);

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, Balance};

    use super::*;

    const TOTAL_SUPPLY: Balance = 1_000_000_000_000_000;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new_default_meta(accounts(1).into(), TOTAL_SUPPLY.into());
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.ft_total_supply().0, TOTAL_SUPPLY);
        assert_eq!(contract.ft_balance_of(accounts(1)).0, TOTAL_SUPPLY);
    }

    #[test]
    #[should_panic(expected = "The contract is not initialized")]
    fn test_default() {
        let context = get_context(accounts(1));
        testing_env!(context.build());
        let _contract = Contract::default();
    }

    #[test]
    fn test_transfer() {
        let mut context = get_context(accounts(2));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(2).into(), TOTAL_SUPPLY.into());
        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(contract.storage_balance_bounds().min.into())
            .predecessor_account_id(accounts(1))
            .build());
        // Paying for account registration, aka storage deposit
        contract.storage_deposit(None, None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(2))
            .build());
        let transfer_amount = TOTAL_SUPPLY / 3;
        contract.ft_transfer(accounts(1), transfer_amount.into(), None);

        testing_env!(context
            .storage_usage(env::storage_usage())
            .account_balance(env::account_balance())
            .is_view(true)
            .attached_deposit(0)
            .build());
        assert_eq!(contract.ft_balance_of(accounts(2)).0, (TOTAL_SUPPLY - transfer_amount));
        assert_eq!(contract.ft_balance_of(accounts(1)).0, transfer_amount);
    }
}
