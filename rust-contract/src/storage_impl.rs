use crate::*;
use near_contract_standards::storage_management::{
    StorageBalance, StorageBalanceBounds, StorageManagement,
};

const NEAR_TOKEN_STORAGE: StorageUsage = 16;
const TOKEN_ID_STORAGE: StorageUsage = 64;
const TOKEN_BALANCE_STORAGE: StorageUsage = TOKEN_ID_STORAGE + NEAR_TOKEN_STORAGE;
const ACCOUNT_STORAGE: StorageUsage = NEAR_TOKEN_STORAGE + TOKEN_BALANCE_STORAGE;
const ACC_ID_STORAGE: StorageUsage = 64;
/// As a key for LookupMap, 4 bytes length would be added to the head
const ACC_ID_AS_KEY_STORAGE: StorageUsage = ACC_ID_STORAGE + 4;
const LOOKUPMAP_STORAGE_PER_ACCOUNT: StorageUsage = ACC_ID_AS_KEY_STORAGE + ACCOUNT_STORAGE;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
pub struct TokenBalance {
    pub token_account_id: AccountId,
    pub balance: NearToken,
}

#[derive(BorshSerialize, BorshDeserialize, Default, Clone)]
pub struct Account {
    pub storage_balance: NearToken,
    pub token_balance: Option<TokenBalance>,
}

impl Contract {
    pub fn internal_storage_cost(&self) -> NearToken {
        NearToken::from_yoctonear(
            LOOKUPMAP_STORAGE_PER_ACCOUNT as u128 * env::storage_byte_cost().as_yoctonear(),
        )
    }

    pub fn internal_storage_registered(&self, account_id: &AccountId) -> bool {
        self.accounts.contains_key(account_id)
    }
}

#[near_bindgen]
impl StorageManagement for Contract {
    #[payable]
    fn storage_deposit(
        &mut self,
        account_id: Option<AccountId>,
        // Our contract's storage min bound is exactly same with storage max bound
        // Thus whether or not registration only is true, this function is expected to behave the same way
        _registration_only: Option<bool>,
    ) -> StorageBalance {
        let amount = env::attached_deposit();

        let account_id = account_id.unwrap_or(env::predecessor_account_id());

        if self.internal_storage_registered(&account_id) {
            Promise::new(env::predecessor_account_id()).transfer(amount);
        } else {
            self.accounts.insert(
                account_id.clone(),
                Account {
                    storage_balance: self.internal_storage_cost(),
                    token_balance: None,
                },
            );

            let refund = amount
                .checked_sub(self.internal_storage_cost())
                .expect(ER11_ACC_NOT_ENOUGH_STORAGE_DEPOSIT);

            if refund.gt(&NearToken::from_yoctonear(0)) {
                Promise::new(env::predecessor_account_id()).transfer(refund);
            }
        }

        self.storage_balance_of(account_id).unwrap()
    }

    #[payable]
    fn storage_withdraw(&mut self, amount: Option<NearToken>) -> StorageBalance {
        assert_one_yocto();

        // Our contract's storage min bound is exactly same with storage max bound
        // Thus user is expected to not able to withdraw any storage balance
        assert!(
            amount.unwrap_or(NearToken::from_yoctonear(0)).is_zero(),
            "{}",
            ER12_ACC_NOT_ENOUGH_STORAGE_BALANCE_FOR_WITHDRAWAL
        );

        self.storage_balance_of(env::predecessor_account_id())
            .unwrap()
    }

    #[payable]
    fn storage_unregister(&mut self, force: Option<bool>) -> bool {
        assert_one_yocto();

        let account_id = env::predecessor_account_id();
        let force_unregister = force.unwrap_or(false);

        let account = self
            .accounts
            .get(&account_id)
            .expect(ER10_ACC_NOT_REGISTERED);
        let storage_balance = account.storage_balance;

        if account.token_balance.is_some() && !force_unregister {
            panic!("{}", ER13_ACC_UNCLAIMED_TOKEN_BALANCE_WARNING);
        }

        // User will lost all remaining unclaimed balance if force unregister is true
        // They should have expected that behaviour since they are purposely setting force to be true
        self.accounts.remove(&account_id);

        assert!(
            env::account_balance().gt(&storage_balance),
            "{}",
            ER14_CONTRACT_NOT_ENOUGH_BALANCE_FOR_ACC_UNREGISTER
        );

        Promise::new(account_id.clone()).transfer(storage_balance);

        true
    }

    fn storage_balance_bounds(&self) -> StorageBalanceBounds {
        let storage_cost = self.internal_storage_cost();

        StorageBalanceBounds {
            min: storage_cost,
            max: Some(storage_cost),
        }
    }

    fn storage_balance_of(&self, account_id: AccountId) -> Option<StorageBalance> {
        if self.internal_storage_registered(&account_id) {
            let storage_balance = self.accounts.get(&account_id).unwrap().storage_balance;

            Some(StorageBalance {
                total: storage_balance,
                available: storage_balance
                    .checked_sub(self.internal_storage_cost())
                    .unwrap(),
            })
        } else {
            None
        }
    }
}
