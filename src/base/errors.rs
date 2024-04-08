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
    LockupNotStartedYet = 20,
    LockupAlreadyCanceled = 21,
    LockupAlreadySettled = 22,
    LockupNotCancellableYet = 23,
    LockupNotFound = 24,
    LockupIsCanceled = 25,
    SpecifiedAmountIsGreaterThanWithdrawable = 26,
    AmountUnderflows = 27,
}
