use crate::base::errors::CustomErrors;

use super::setup::{SetupStreamTest, StreamFields};

#[test]
fn test_get_stream_should_return_the_correct_data() {
    let (vars, id) = SetupStreamTest::setup_with_stream_created(StreamFields::default());

    let stream = vars.contract.get_lockup(&id);

    assert_eq!(stream.sender.clone(), vars.admin.clone());
}

#[test]
fn test_get_stream_should_revert_when_stream_does_not_exist() {
    let (vars, _) = SetupStreamTest::setup_with_stream_created(StreamFields::default());

    let stream = vars.contract.try_get_lockup(&1);

    assert_eq!(stream, Err(Ok(CustomErrors::LockupNotFound)));
}
