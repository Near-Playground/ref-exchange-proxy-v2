use crate::*;
use near_sdk::ext_contract;

#[ext_contract(ext_fungible_token)]
pub trait _FungibleTokenContract {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: NearToken, memo: Option<String>);

    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: NearToken,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<NearToken>;
}

#[ext_contract(ext_ref_exchange)]
pub trait _RefExchangeContract {
    fn swap(&mut self, actions: Vec<SwapAction>, referral_id: Option<AccountId>) -> NearToken;

    fn withdraw(
        &mut self,
        token_id: AccountId,
        amount: NearToken,
        unregister: Option<bool>,
        skip_unwrap_near: Option<bool>,
    ) -> Promise;
}

#[ext_contract(ext_self)]
pub trait _RefExchangeProxyContract {
    fn callback_post_ref_finance_deposit(
        &mut self,
        account_id: AccountId,
        token_in: AccountId,
        amount_in: NearToken,
        amount_deposited: NearToken,
        actions: Vec<SwapAction>,
    ) -> Promise;

    fn callback_post_ref_finance_swap(
        &mut self,
        account_id: AccountId,
        token_in: AccountId,
        amount_in: NearToken,
        amount_deposited: NearToken,
        token_out: AccountId,
    ) -> Promise;

    fn callback_post_ref_finance_withdraw(
        &mut self,
        account_id: AccountId,
        token_out: AccountId,
        amount_out: NearToken,
    ) -> Option<Promise>;

    fn callback_post_send_tokens(
        &mut self,
        account_id: AccountId,
        token_id: AccountId,
        amount: NearToken,
    ) -> Option<TokenBalance>;
}
