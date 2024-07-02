use near_sdk::store::{LookupMap, LookupSet};
use near_sdk::{assert_one_yocto, env, near_bindgen, AccountId, BorshStorageKey, NearToken, PanicOnDefault, Promise, StorageUsage};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

mod storage_impl;
use storage_impl::Account;

mod error;
use error::*;

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    Accounts,
    RegisteredTokens
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    accounts: LookupMap<AccountId, Account>,
    exchange_fee: u16,
    owner_id: AccountId,
    registered_tokens: LookupSet<AccountId>,
    ref_exchange_id: AccountId,
    referral_id: AccountId,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, ref_exchange_id: AccountId, referral_id: AccountId, exchange_fee: u16) -> Self {
        Self {
            accounts: LookupMap::new(StorageKey::Accounts),
            exchange_fee,
            owner_id,
            registered_tokens: LookupSet::new(StorageKey::RegisteredTokens),
            ref_exchange_id,
            referral_id,
        }
    }
}



