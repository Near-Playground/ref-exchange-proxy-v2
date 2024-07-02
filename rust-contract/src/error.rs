// Account & Storage Related
pub const ER10_ACC_NOT_REGISTERED: &str = "Account not registered";
pub const ER11_ACC_NOT_ENOUGH_STORAGE_DEPOSIT: &str = "Insufficient deposit to cover storage cost.";
pub const ER12_ACC_NOT_ENOUGH_STORAGE_BALANCE_FOR_WITHDRAWAL: &str = "Not enough balance to withdraw.";
pub const ER13_ACC_UNCLAIMED_TOKEN_BALANCE_WARNING: &str = "Unclaimed token balance detected. Please withdraw it before unregistering.";
pub const ER14_CONTRACT_NOT_ENOUGH_BALANCE_FOR_ACC_UNREGISTER: &str = "Contract don't have enough balance to refund storage deposit.";

// Unclaimed Token Balance Related
pub const ER20_NO_UNCLAIMED_TOKEN_BALANCE: &str = "No unclaimed token balance detected.";

// Callback Related
pub const ER30_NO_PROMISE_RESULT_DETECTED: &str = "This function can only be called as callback.";

// Send Tokens Related
pub const ER40_SEND_TOKENS_FAILED: &str = "Failed to send tokens. Saving tokens to unclaimed_token_balance.";