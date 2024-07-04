use crate::*;
use near_sdk::env;

pub const FEE_DIVISOR: u128 = 10000;

/// Single swap action.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SwapAction {
    pub pool_id: u64,
    pub token_in: AccountId,
    pub amount_in: Option<NearToken>,
    pub token_out: AccountId,
    pub min_amount_out: NearToken,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
struct Message {
    actions: Vec<SwapAction>,
}

impl Contract {
    pub fn internal_safe_parse_swap_actions(
        &self,
        msg: String,
        token_in: AccountId,
        amount_in: NearToken,
    ) -> Vec<SwapAction> {
        let message: Message =
            near_sdk::serde_json::from_str(&msg).expect(ER50_FAIL_TO_PARSE_SWAP_ACTIONS);

        let mut actions = message.actions;

        assert!(!actions.is_empty(), "{}", ER51_EMPTY_SWAP_ACTIONS);

        assert_eq!(
            actions.first().map(|action| &action.token_in),
            Some(&token_in)
        );

        // Change the first element "amount_in" to what we received subtract by the fee
        if let Some(first_action) = actions.first_mut() {
            first_action.amount_in = Some(amount_in);
        }

        // Stripping away all "amount_in" from second elements onwards
        for action in actions.iter_mut().skip(1) {
            action.amount_in = None;
        }

        actions
    }
}

#[near_bindgen]
impl Contract {
    pub fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: NearToken,
        msg: String,
    ) -> PromiseOrValue<NearToken> {
        assert!(
            self.internal_storage_registered(&sender_id),
            "{}",
            ER10_ACC_NOT_REGISTERED
        );

        assert!(
            self.accounts
                .get(&sender_id)
                .unwrap()
                .token_balance
                .is_none(),
            "{}",
            ER53_ACC_UNCLAIMED_TOKEN_BALANCE_WARNING
        );

        let token_in = env::predecessor_account_id();
        let amount_in = amount;
        let fee = amount_in
            .checked_mul(self.exchange_fee as u128)
            .unwrap()
            .checked_div(FEE_DIVISOR)
            .unwrap();
        let amount_deposited = amount_in.checked_sub(fee).unwrap();

        let actions =
            self.internal_safe_parse_swap_actions(msg, token_in.clone(), amount_deposited);

        let token_out = actions.last().unwrap().token_out.clone();

        assert!(
            self.registered_tokens.contains(&token_out),
            "{}",
            ER52_SWAP_RESULT_TOKEN_NOT_SUPPORTED
        );

        ext_fungible_token::ext(token_in.clone())
            .with_static_gas(Gas::from_tgas(40))
            .with_attached_deposit(NearToken::from_yoctonear(1))
            .ft_transfer_call(
                self.ref_exchange_id.clone(),
                amount_deposited,
                None,
                "".into(),
            )
            .then(
                ext_self::ext(env::current_account_id())
                    .with_static_gas(Gas::from_tgas(170))
                    .callback_post_ref_finance_deposit(
                        sender_id,
                        token_in,
                        amount_in,
                        amount_deposited,
                        actions,
                    ),
            );

        PromiseOrValue::Value(NearToken::from_yoctonear(0))
    }

    #[private]
    pub fn callback_post_ref_finance_deposit(
        &mut self,
        account_id: AccountId,
        token_in: AccountId,
        amount_in: NearToken,
        amount_deposited: NearToken,
        actions: Vec<SwapAction>,
    ) -> Promise {
        assert_eq!(
            env::promise_results_count(),
            1,
            "{}",
            ER30_NO_PROMISE_RESULT_DETECTED
        );

        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                let token_out = actions.last().unwrap().token_out.clone();

                ext_ref_exchange::ext(self.ref_exchange_id.clone())
                    .with_static_gas(Gas::from_tgas(50))
                    .with_attached_deposit(NearToken::from_yoctonear(1))
                    .swap(actions, self.referral_id.clone())
                    .then(
                        ext_self::ext(env::current_account_id())
                            .with_static_gas(Gas::from_tgas(110))
                            .callback_post_ref_finance_swap(
                                account_id,
                                token_in,
                                amount_in,
                                amount_deposited,
                                token_out,
                            ),
                    )
            }
            PromiseResult::Failed => self.internal_send_tokens(account_id, token_in, amount_in),
        }
    }

    #[private]
    pub fn callback_post_ref_finance_swap(
        &mut self,
        account_id: AccountId,
        token_in: AccountId,
        amount_in: NearToken,
        amount_deposited: NearToken,
        token_out: AccountId,
    ) -> Promise {
        assert_eq!(
            env::promise_results_count(),
            1,
            "{}",
            ER30_NO_PROMISE_RESULT_DETECTED
        );

        match env::promise_result(0) {
            PromiseResult::Successful(result) => {
                let amount_out = NearToken::from_yoctonear(
                    String::from_utf8(result)
                        .unwrap()
                        .trim_matches('"')
                        .to_string()
                        .parse()
                        .unwrap(),
                );

                ext_ref_exchange::ext(self.ref_exchange_id.clone())
                    .with_static_gas(Gas::from_tgas(50))
                    .with_attached_deposit(NearToken::from_yoctonear(1))
                    .withdraw(token_out.clone(), amount_out, None, None)
                    .then(
                        ext_self::ext(env::current_account_id())
                            .with_static_gas(Gas::from_tgas(50))
                            .callback_post_ref_finance_withdraw(account_id, token_out, amount_out),
                    )
            }
            PromiseResult::Failed => ext_ref_exchange::ext(self.ref_exchange_id.clone())
                .with_static_gas(Gas::from_tgas(50))
                .with_attached_deposit(NearToken::from_yoctonear(1))
                .withdraw(token_in.clone(), amount_deposited, None, None)
                .then(
                    ext_self::ext(env::current_account_id())
                        .with_static_gas(Gas::from_tgas(50))
                        .callback_post_ref_finance_withdraw(account_id, token_in, amount_in),
                ),
        }
    }

    #[private]
    pub fn callback_post_ref_finance_withdraw(
        &mut self,
        account_id: AccountId,
        token_out: AccountId,
        amount_out: NearToken,
    ) -> Option<Promise> {
        assert_eq!(
            env::promise_results_count(),
            1,
            "{}",
            ER30_NO_PROMISE_RESULT_DETECTED
        );

        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                Some(self.internal_send_tokens(account_id, token_out, amount_out))
            }
            PromiseResult::Failed => {
                let mut account = self.accounts.get(&account_id).unwrap().clone();

                log!("{}", ER60_REF_EXCHANGE_WITHDRAWAL_FAILED);

                account.token_balance = Some(TokenBalance {
                    token_account_id: token_out,
                    balance: amount_out,
                });

                self.accounts.insert(account_id, account);

                None
            }
        }
    }
}
