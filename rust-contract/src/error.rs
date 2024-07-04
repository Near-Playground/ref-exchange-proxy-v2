// Account & Storage Related
pub const ER10_ACC_NOT_REGISTERED: &str = "Account not registered";
pub const ER11_ACC_NOT_ENOUGH_STORAGE_DEPOSIT: &str = "Insufficient deposit to cover storage cost.";
pub const ER12_ACC_NOT_ENOUGH_STORAGE_BALANCE_FOR_WITHDRAWAL: &str =
    "Not enough balance to withdraw.";
pub const ER13_ACC_UNCLAIMED_TOKEN_BALANCE_WARNING: &str =
    "Unclaimed token balance detected. Please withdraw it before unregistering.";
pub const ER14_CONTRACT_NOT_ENOUGH_BALANCE_FOR_ACC_UNREGISTER: &str =
    "Contract don't have enough balance to refund storage deposit.";

// Unclaimed Token Balance Related
pub const ER20_NO_UNCLAIMED_TOKEN_BALANCE: &str = "No unclaimed token balance detected.";

// Callback Related
pub const ER30_NO_PROMISE_RESULT_DETECTED: &str = "This function can only be called as callback.";

// Send Tokens Related
pub const ER40_SEND_TOKENS_FAILED: &str =
    "Failed to send tokens. Saving tokens to unclaimed_token_balance.";

// Swap Related
pub const ER50_FAIL_TO_PARSE_SWAP_ACTIONS: &str = "Invalid swap actions format.";
pub const ER51_EMPTY_SWAP_ACTIONS: &str = "Must have at least one swap action.";
pub const ER52_SWAP_RESULT_TOKEN_NOT_SUPPORTED: &str =
    "The swap result tokens is not supported by this contract yet, please call register_token first.";
pub const ER53_ACC_UNCLAIMED_TOKEN_BALANCE_WARNING: &str =
    "Unclaimed token balance detected. Please withdraw it before swapping.";

// Ref Exchange Related
pub const ER60_REF_EXCHANGE_WITHDRAWAL_FAILED: &str = "Failed to withdraw from Ref Exchange.";

// Management Related
pub const ER70_MUST_BE_OWNER: &str = "Only owner can call this function.";
pub const ER71_REF_EXCHANGE_ID_SHOULD_NOT_BE_CHANGED: &str =
    "Ref Exchange ID should not be changed once intiialised.";
pub const ER72_EXCHANGE_FEE_TOO_HIGH: &str = "Exchange fee can not be higher than 9999 (99.99%)";
pub const ER73_WITHDRAW_NEAR_NOT_ALLOWED: &str =
    "Withdraw near is not allowed, might accidentally caused user storage deposit problem.";
