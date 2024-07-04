use crate::*;

impl Contract {
    pub fn internal_send_tokens(
        &mut self,
        account_id: AccountId,
        token_id: AccountId,
        amount: NearToken,
    ) -> Promise {
        ext_fungible_token::ext(token_id.clone())
            .with_attached_deposit(NearToken::from_yoctonear(1))
            .with_static_gas(Gas::from_tgas(20))
            .ft_transfer(account_id.clone(), amount, None)
            .then(
                ext_self::ext(env::current_account_id())
                    .with_static_gas(Gas::from_tgas(20))
                    .callback_post_send_tokens(account_id, token_id, amount),
            )
    }
}

#[near_bindgen]
impl Contract {
    #[private]
    pub fn callback_post_send_tokens(
        &mut self,
        account_id: AccountId,
        token_id: AccountId,
        amount: NearToken,
    ) -> Option<TokenBalance> {
        assert_eq!(
            env::promise_results_count(),
            1,
            "{}",
            ER30_NO_PROMISE_RESULT_DETECTED
        );

        match env::promise_result(0) {
            PromiseResult::Successful(_) => Some(TokenBalance {
                token_account_id: token_id,
                balance: amount,
            }),
            PromiseResult::Failed => {
                let mut account = self.accounts.get(&account_id).unwrap().clone();

                log!("{}", ER40_SEND_TOKENS_FAILED);

                account.token_balance = Some(TokenBalance {
                    token_account_id: token_id,
                    balance: amount,
                });

                self.accounts.insert(account_id, account);

                None
            }
        }
    }

    pub fn check_unclaimed_token_balance(&self, account_id: AccountId) -> Option<TokenBalance> {
        let account = self
            .accounts
            .get(&account_id)
            .expect(ER10_ACC_NOT_REGISTERED)
            .clone();
        account.token_balance
    }

    #[payable]
    pub fn withdraw_unclaimed_token_balance(&mut self) -> Promise {
        assert_one_yocto();

        let account_id = env::predecessor_account_id();
        let mut account = self
            .accounts
            .get(&account_id)
            .expect(ER10_ACC_NOT_REGISTERED)
            .clone();
        let token_balance = account
            .token_balance
            .clone()
            .expect(ER20_NO_UNCLAIMED_TOKEN_BALANCE);
        let token_id = token_balance.token_account_id.clone();
        let amount = token_balance.balance.clone();

        account.token_balance = None;
        self.accounts.insert(account_id.clone(), account);

        self.internal_send_tokens(account_id, token_id, amount)
    }
}
