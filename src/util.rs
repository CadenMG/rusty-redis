use std::mem::transmute;
use bytes::Bytes;

pub const SUCCESS_AND_DATA: u8 = 1;
pub const SUCCESS_AND_NO_DATA: u8 = 2;
pub const FAILED: u8 = 3;

pub fn set_request(key: &str, val: &[u8]) -> Vec<u8> {
    let mut set = vec![0, 0, 0, 3, 0, 0, 0, 3, 115, 101, 116];
    let key_bytes = key.as_bytes();
    let key_len_bytes = as_bytes(key_bytes.len() as u32);
    let val_len_bytes = as_bytes(val.len() as u32);
    set.extend(key_len_bytes);
    set.extend(key_bytes);
    set.extend(val_len_bytes);
    set.extend(val);
    set
}

pub fn get_request(key: &str) -> Vec<u8> {
    let mut get = vec![0, 0, 0, 2, 0, 0, 0, 3, 103, 101, 116];
    let key_bytes = key.as_bytes();
    let key_len_bytes = as_bytes(key_bytes.len() as u32);
    get.extend(key_len_bytes);
    get.extend(key_bytes);
    get
}

pub fn del_request(key: &str) -> Vec<u8> {
    let mut del = vec![0, 0, 0, 2, 0, 0, 0, 3, 100, 101, 108];
    let key_bytes = key.as_bytes();
    let key_len_bytes = as_bytes(key_bytes.len() as u32);
    del.extend(key_len_bytes);
    del.extend(key_bytes);
    del 
}

#[inline]
fn as_bytes(n: u32) -> [u8; 4] {
    unsafe { transmute(n.to_be()) }
}

#[inline]
pub fn parse_num(b1: u8, b2: u8, b3: u8, b4: u8) -> usize {
    ((b1 as usize) << 24) |
    ((b2 as usize) << 16) |
    ((b3 as usize) << 8) |
    ((b4 as usize) << 0)
}

#[inline]
pub fn parse_string(bytes: &[u8]) -> String {
    std::str::from_utf8(bytes).unwrap().to_string()
}

pub fn to_response(status: u8, bytes: Option<Bytes>) -> Vec<u8> {
    // Responses start with a 32 bit status
    let mut res = vec![0, 0, 0, status];

    // Success and data is Some
    if status == SUCCESS_AND_DATA {
        res.extend(bytes.unwrap());
    } else if status == FAILED {
        res.extend(bytes.unwrap())
    }
    res
}

pub fn from_response(res: &Vec<u8>) -> String {
    let status = parse_num(res[0], res[1], res[2], res[3]) as u8;
    if status == SUCCESS_AND_DATA {
        format!("{:?}", res.get(4..).unwrap())
    } else if status == SUCCESS_AND_NO_DATA {
        "Success".to_string()
    } else if status == FAILED {
        format!("Failed with message: {}", parse_string(&res[4..]))
    } else {
        "Unknown response".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_bytes() {
        let one = as_bytes(1);
        assert_eq!(one, [0, 0, 0, 1]);
        let max = as_bytes(u32::MAX);
        assert_eq!(max, [255, 255, 255, 255]);
    }

    #[test]
    fn test_set_request() {
        let res = set_request("key", b"val");
        let expect = vec![
            0, 0, 0, 3,         // 3 words
            0, 0, 0, 3,         // next word len of 3
            115, 101, 116,      // set
            0, 0, 0, 3,         // next word len of 3
            107, 101, 121,      // key
            0, 0, 0, 3,         // next word len of 3
            118, 97, 108];      // val
        assert_eq!(res, expect)
    }

    #[test]
    fn test_get_request() {
        let res = get_request("test");
        let expect = vec![
            0, 0, 0, 2,         // 2 words
            0, 0, 0, 3,         // next word len of 3
            103, 101, 116,      // get
            0, 0, 0, 4,         // next word len of 4
            116, 101, 115, 116];// test
        assert_eq!(res, expect)
    }

    #[test]
    fn test_del_request() {
        let res = del_request("test");
        let expect = vec![
            0, 0, 0, 2,         // 2 words
            0, 0, 0, 3,         // next word len of 3
            100, 101, 108,      // del
            0, 0, 0, 4,         // next word len of 4
            116, 101, 115, 116];// test
        assert_eq!(res, expect)
    }
}
