use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug)]
#[repr(u64)]
pub enum CustomErrors {
    CreateStreamInvalidAmount = 1000,
    CreateStreamInvalidReceiver = 1001,
    CreateStreamInvalidStartDate = 1002,
    CreateStreamInvalidCliffDate = 1003,
    CreateStreamInvalidCancellableDate = 1004,

    WithdrawStreamNotStartedYet = 2000,

    CancelStreamAlreadyCanceled = 3000,
    CancelStreamAlreadySettled = 3001,
    CancelStreamNotCancellableYet = 3002,

    GetStreamNotFound = 4000,
}
