use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u64)]
pub enum CustomErrors {
    InvalidAmount = 1000,
    InvalidReceiver = 1001,
    InvalidStartDate = 1002,
    InvalidCliffDate = 1003,
    InvalidCancellableDate = 1004,

    StreamNotStartedYet = 2000,

    StreamAlreadyCanceled = 3000,
    StreamAlreadySettled = 3001,
    StreamNotCancellableYet = 3002,

    StreamNotFound = 4000,

    StreamIsCanceled = 5000,
    SpecifiedAmountIsGreaterThanWithdrawable = 5001,
    AmountUnderflows = 5002,
}
