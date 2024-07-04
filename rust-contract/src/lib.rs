use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::store::{LookupMap, LookupSet};
use near_sdk::{
    assert_one_yocto, env, log, near_bindgen, AccountId, BorshStorageKey, Gas, NearToken,
    PanicOnDefault, Promise, PromiseOrValue, PromiseResult, StorageUsage,
};

mod storage_impl;
use storage_impl::*;

mod error;
use error::*;

mod contracts;
use contracts::*;

mod swap_impl;
use swap_impl::SwapAction;

mod withdraw_impl;

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    Accounts,
    RegisteredTokens,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    accounts: LookupMap<AccountId, Account>,
    exchange_fee: u16,
    owner_id: AccountId,
    registered_tokens: LookupSet<AccountId>,
    ref_exchange_id: AccountId,
    referral_id: Option<AccountId>,
}

impl Contract {
    pub fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "{}",
            ER70_MUST_BE_OWNER
        );
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner_id: AccountId,
        ref_exchange_id: AccountId,
        referral_id: Option<AccountId>,
        exchange_fee: u16,
    ) -> Self {
        assert!(exchange_fee < 10000, "{}", ER72_EXCHANGE_FEE_TOO_HIGH);

        Self {
            accounts: LookupMap::new(StorageKey::Accounts),
            exchange_fee,
            owner_id,
            registered_tokens: LookupSet::new(StorageKey::RegisteredTokens),
            ref_exchange_id,
            referral_id,
        }
    }

    #[payable]
    pub fn set_owner(&mut self, new_owner_id: AccountId) {
        assert_one_yocto();
        self.assert_owner();
        self.owner_id = new_owner_id;
    }

    #[payable]
    pub fn set_referral_id(&mut self, referral_id: Option<AccountId>) {
        assert_one_yocto();
        self.assert_owner();
        self.referral_id = referral_id;
    }

    #[payable]
    pub fn set_ref_exchange_id(&mut self, _ref_exchange_id: AccountId) {
        panic!("{}", ER71_REF_EXCHANGE_ID_SHOULD_NOT_BE_CHANGED);
    }

    #[payable]
    pub fn set_exchange_fee(&mut self, exchange_fee: u16) {
        assert!(exchange_fee < 10000, "{}", ER72_EXCHANGE_FEE_TOO_HIGH);
        self.assert_owner();
        self.exchange_fee = exchange_fee;
    }

    #[payable]
    pub fn withdraw_near(&mut self, _amount: NearToken, _receiver_id: AccountId) {
        panic!("{}", ER73_WITHDRAW_NEAR_NOT_ALLOWED);
    }

    #[payable]
    pub fn withdraw_ft(
        &mut self,
        token_id: AccountId,
        amount: NearToken,
        receiver_id: AccountId,
    ) -> Promise {
        assert_one_yocto();
        self.assert_owner();
        ext_fungible_token::ext(token_id).ft_transfer(receiver_id, amount, None)
    }

    #[payable]
    pub fn withdraw_ft_from_ref_finance(
        &mut self,
        token_id: AccountId,
        amount: NearToken,
    ) -> Promise {
        assert_one_yocto();
        self.assert_owner();
        ext_ref_exchange::ext(self.ref_exchange_id.clone()).withdraw(token_id, amount, None, None)
    }

    #[payable]
    pub fn register_token(&mut self, token_id: AccountId) {
        // Owner should make sure to register storage depsoit before register the token
        assert_one_yocto();
        self.assert_owner();
        self.registered_tokens.insert(token_id);
    }
}
