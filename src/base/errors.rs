use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u64)]
pub enum CustomErrors {
    InvalidAmount = 0,
    InvalidReceiver = 1,
    InvalidStartDate = 2,
    InvalidCliffDate = 3,
    InvalidCancellableDate = 4,
    StreamNotStartedYet = 5,
    StreamAlreadyCanceled = 6,
    StreamAlreadySettled = 7,
    StreamNotCancellableYet = 8,
    StreamNotFound = 9,
    StreamIsCanceled = 10,
    SpecifiedAmountIsGreaterThanWithdrawable = 11,
    AmountUnderflows = 12,
}
