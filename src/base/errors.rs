use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u64)]
pub enum CustomErrors {
    InvalidAmount = 10,
    InvalidReceiver = 11,
    InvalidStartDate = 12,
    InvalidCliffDate = 13,
    InvalidCancellableDate = 14,
    InvalidVestingDates = 15,
    StreamNotStartedYet = 20,
    StreamAlreadyCanceled = 21,
    StreamAlreadySettled = 22,
    StreamNotCancellableYet = 23,
    StreamNotFound = 24,
    StreamIsCanceled = 25,
    SpecifiedAmountIsGreaterThanWithdrawable = 26,
    AmountUnderflows = 27,
}
