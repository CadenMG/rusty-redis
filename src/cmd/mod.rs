use std::fmt;
use std::str;

mod get;
use get::Get;

mod set;
use set::Set;

mod del;
use del::Del;

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Clone)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unable to parse message")
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    len: usize,
    msg: String,
}

impl Message {
    pub fn new(msg: String) -> Message {
        Message {
            len: msg.len(),
            msg: msg,
        }
    }

    pub fn parse_bytes(bytes: &Vec<u8>, n: usize) -> Result<Message> {
        let len = ((bytes[0] as usize) << 24) |
            ((bytes[1] as usize) << 16) |
            ((bytes[2] as usize) << 8) |
            ((bytes[3] as usize) << 0);

        if n - 4 != len {
            return Err(ParseError);
        }

        let msg = str::from_utf8(&bytes[4..n]).expect("").to_string();
        Ok(Message { len, msg })
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.msg)
    }
}


#[derive(Debug, Clone)]
pub enum Command {
    Message(Message),
    Get(Get),
    Set(Set),
    Del(Del),
}
