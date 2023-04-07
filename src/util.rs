use std::mem::transmute;

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
