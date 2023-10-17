use super::types::Amounts;

pub fn calculate_amounts(
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
