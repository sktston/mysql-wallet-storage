#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(i32)]
pub enum ErrorCode {
    Success = 0,
    InvalidStorageHandle = 1,
    InvalidSearchHandle = 2,
    InvalidRecordHandle = 3,
    InvalidEncoding = 4,
    InvalidJSON = 5,
    ConnectionError = 6,
    DatabaseError = 7,
    UnknownWalletName = 8,
    UnknownRecord = 9,
    TypeNotFetched = 10,
    ValueNotFetched = 11,
    TagsNotFetched = 12,
    RecordAlreadExists = 14,
    UnknownTag = 15,
    TagAlreadyExists =16,
    TagDataTooLong = 17,
}

macro_rules! check_result {
    ($r: expr, $e: expr) => {
        match $r {
            Err(err) => { println!("{}", err); return $e },
            Ok(x) => x
        }
    }
}

macro_rules! check_option {
    ($o: expr, $e: expr) => {
        match $o {
            None => return $e,
            Some(x) => x
        }
    }
}