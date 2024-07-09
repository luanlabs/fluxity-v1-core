use super::{
    constants::MONTH_IN_SECONDS,
    types::{Amounts, Lockup, Rate},
};

pub fn calculate_stream_amounts(
    start_date: u64,
    end_date: u64,
    cliff_date: u64,
    current_date: u64,
    amount: i128,
) -> Amounts {
    if current_date <= start_date {
        return Amounts {
            sender_amount: amount,
            receiver_amount: 0,
        };
    }

    if current_date <= cliff_date {
        return Amounts {
            sender_amount: amount,
            receiver_amount: 0,
        };
    }

    if current_date >= end_date {
        return Amounts {
            sender_amount: 0,
            receiver_amount: amount,
        };
    }

    let total_date: i128 = (end_date - start_date).into();
    let proceeded_date: i128 = (current_date - start_date).into();

    let receiver_amount = amount * proceeded_date / total_date;
    let sender_amount = amount - receiver_amount;

    Amounts {
        sender_amount,
        receiver_amount,
    }
}

pub fn calculate_vesting_amounts(
    start_date: u64,
    end_date: u64,
    cliff_date: u64,
    current_date: u64,
    rate: Rate,
    amount: i128,
) -> Amounts {
    if current_date <= start_date || current_date <= cliff_date {
        return Amounts {
            sender_amount: amount,
            receiver_amount: 0,
        };
    }

    if current_date >= end_date {
        return Amounts {
            sender_amount: 0,
            receiver_amount: amount,
        };
    }

    let total_date: i128 = (end_date - start_date).into();
    let proceeded_date: i128 = (current_date - start_date).into();
    let rate_in_seconds = rate as i128;

    let times = proceeded_date / rate_in_seconds;
    let one_time_amount = amount * rate_in_seconds / total_date;

    // TODO: if duration / rate is not dividable, what happens? check all of them
    let receiver_amount = times * one_time_amount;
    let sender_amount = amount - receiver_amount;

    Amounts {
        sender_amount,
        receiver_amount,
    }
}

pub fn calculate_additional_time(lockup: &Lockup, adding_amount: i128) -> u64 {
    let duration: i128 = (lockup.end_date - lockup.start_date).into();

    (adding_amount * duration / lockup.amount)
        .try_into()
        .unwrap()
}

pub fn calculate_lockup_fee(start_date: u64, end_date: u64, monthly_fee: i128) -> i128 {
    let lockup_duration = end_date - start_date;
    let months = lockup_duration / MONTH_IN_SECONDS;

    let months_plus: i128 = (months + 1).into();

    months_plus * monthly_fee
}
