use near_sdk::ext_contract;
use crate::*;

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

#[ext_contract(ext_self)]
pub trait _RefExchangeProxy {
    fn callback_post_send_tokens(
        &mut self,
        account_id: AccountId,
        token_id: AccountId,
        amount: NearToken,
    ) -> Option<TokenBalance>;
}