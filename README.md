# Fluxity V1 Contract

Fluxity is an streaming platform, this repository is the source code of the
Soroban smart contracts of Fluxity. All written in Rust.

## Functions

### create_stream

Creates an stream, transfers the total amount of the stream from the **sender** when called.
Uses token approvals to transfer the tokens. So make sure you have the required allowance before
calling this function.

Parameters:

    - **sender** is the address of the caller of the function, the one who creates the stream and transfers the total amount
    - **receiver** is the address of the receiver of the stream.
    - **token** is the address of the toke being streamed
    - **amount** is the total amount of the stream
    - **start_date** is the timestamp (in seconds) of the start of the stream.
    - **end_date** is the timestamp (in seconds) of the end of the stream.
    - **cliff_date** is the timestamp (in seconds) of the cliff of the stream.
    - **cancellable_date** is a timestamp (in seconds) that specifies when the stream can become cancellable
    - **rate** specifies the rate that user selected in the dashboard when creating an stream

Notes:

If you want to have a non-cancellable stream,
make sure to pass the **end_date** and **cancellable_date** the same date

If you want to have a cancellable stream from the beginning,
pass the **start_date** and **cancellable_date** the same date

Cliff date is optional, if you don't want the stream to have a cliff,
then pass **cliff_date** the same date as **start_date**

Rate field is just used to show the rate of the stream besides the amount
in the client and other than that it's useless. If you don't care what is going
to be shown on the stream, you can just pass any value to it.

For example, if rate is Daily, the client will divide the total
amount by 86400 (amount of seconds in a day) and show X / Daily where X is the amount divided.

### cancel_stream

Cancels an stream and returns the total streamed amount to the receiver and the remainings back to the sender.

Parameters:

    - **id** is the unique id of the stream

Notes:

Can only be called if the current timestamp is higher than the **cancellable_date** and the stream
is still active. If the stream is settled, or the timestamp is lower than the **cancellable_date**
then the function reverts.

### withdraw_stream

It's used to withdraw from the stream by the receiver. Can only be called if the stream is started
and stream has not settled (ended or cancelled).

Parameters:

    - **id** is the unique id of the stream
    - **amount** is the amount to withdraw, you can pass 0 to withdraw the maximum unlocked amount
