use near_sdk::store::{LookupMap, LookupSet};
use near_sdk::{assert_one_yocto, env, log, near_bindgen, AccountId, BorshStorageKey, Gas, NearToken, PanicOnDefault, Promise, PromiseOrValue, PromiseResult, StorageUsage};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

mod storage_impl;
use storage_impl::*;

mod error;
use error::*;

mod contracts;
use contracts::*;

mod withdraw_impl;

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



