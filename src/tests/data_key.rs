use core::borrow::BorrowMut;

use super::std::println;

use soroban_sdk::xdr::{Limited, ReadXdr, ScBytes, ScVal, ToXdr};

use crate::base::data_key::DataKey;

use super::setup::SetupStreamTest;

#[test]
fn test_data_key_xdr() {
    let vars = SetupStreamTest::setup(200);
    let mut key1 = DataKey::LatestStreamId.to_xdr(&vars.env);

    // let sc = ScVal::read_xdr(key1);

    ScBytes::read_xdr(key1.borrow_mut());

    println!("{:?}", sc);
    // let byte = key1.to_xdr(&vars.env);
    // println!("{:?}", byte.len());
    // println!("{:?}", byte.len());
    // println!("{:?}", byte.len());
    // println!("{:?}", byte.len());
    // println!("{:?}", byte.len());
    //
    // let mut out = [0u8; 44];
    // byte.copy_into_slice(&mut out);
    //
    // println!("{:?}", out);
    //
    // let c = str::from_utf8(&out).unwrap();
    //
    // println!("{:?}", c);

    // let v = key1.to_val();
    //
    // let c = v.to_xdr(&vars.env);
    //
    // println!("kdfhdkfh");
    // println!("{:?}", v);
    // println!("{:?}", c);
    // println!("kdfhdkfh");
    // println!("kdfhdkfh");
    // println!("kdfhdkfh");
}
