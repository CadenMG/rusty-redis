use std::fmt;
use bytes::Bytes;

mod get;
use get::Get;
mod set;
use set::Set;
mod del;
use del::Del;

#[derive(Debug, Clone)]
pub struct ParseError;

type Result<T> = std::result::Result<T, ParseError>;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unable to parse command")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Get(Get),
    Set(Set),
    Del(Del),
}

impl Command {
    pub fn parse_bytes(bytes: &Vec<u8>, n: usize) -> Result<Command> {
        if n < 4 {
            return Err(ParseError);
        }

        // todo: abstract this / cleanup
        match parse_num(bytes[0], bytes[1], bytes[2], bytes[3]) {
            2 => {
                if n < 12 {
                    return Err(ParseError);
                }
                let cmd_len = parse_num(bytes[4], bytes[5], bytes[6], bytes[7]);
                if n < cmd_len + 12 {
                    return Err(ParseError);
                }
                let cmd_name = parse_string(&bytes[8..8+cmd_len]);
                let data_len = parse_num(
                    bytes[8+cmd_len],
                    bytes[9+cmd_len],
                    bytes[10+cmd_len],
                    bytes[11+cmd_len],
                );
                if n != cmd_len + 12 + data_len {
                    return Err(ParseError);
                }
                let data = &bytes[cmd_len+12..cmd_len+12+data_len];
                parse_command(&cmd_name.to_lowercase(), data)
            },
            3 => {
                if n < 16 {
                    return Err(ParseError);
                }
                let cmd_len = parse_num(bytes[4], bytes[5], bytes[6], bytes[7]);
                if n < cmd_len + 12 {
                    return Err(ParseError);
                }
                let cmd_name = parse_string(&bytes[8..8+cmd_len]);
                let data1_len = parse_num(
                    bytes[8+cmd_len],
                    bytes[9+cmd_len],
                    bytes[10+cmd_len],
                    bytes[11+cmd_len],
                );
                if n < cmd_len + 12 + data1_len {
                    return Err(ParseError);
                }
                let data1 = &bytes[cmd_len+12..cmd_len+12+data1_len];
                let data2_len = parse_num(
                    bytes[12+cmd_len+data1_len],
                    bytes[13+cmd_len+data1_len],
                    bytes[14+cmd_len+data1_len],
                    bytes[15+cmd_len+data1_len],
                );
                if n != cmd_len + 16 + data1_len + data2_len {
                    return Err(ParseError);
                }
                let data2 = &bytes[cmd_len+16+data1_len..cmd_len+16+data1_len+data2_len];
                parse_command_two_arg(&cmd_name.to_lowercase(), data1, data2)
            }
            _ => Err(ParseError)
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Command::Get(get) => get.to_string(),
            Command::Set(set) => set.to_string(),
            Command::Del(del) => del.to_string(),
        };
        write!(f, "{}", msg)
    }
}

#[inline]
fn parse_num(b1: u8, b2: u8, b3: u8, b4: u8) -> usize {
    ((b1 as usize) << 24) |
    ((b2 as usize) << 16) |
    ((b3 as usize) << 8) |
    ((b4 as usize) << 0)
}

#[inline]
fn parse_string(bytes: &[u8]) -> String {
    std::str::from_utf8(bytes).unwrap().to_string()
}

fn parse_command_two_arg(name: &str, data1: &[u8], data2: &[u8]) -> Result<Command> {
    match name {
        "set" => Ok(Command::Set(Set::new(parse_string(data1), Bytes::from(data2.to_vec())))),
        _ => Err(ParseError),
    }
}

fn parse_command(name: &str, data: &[u8]) -> Result<Command> {
    match name {
        "get" => Ok(Command::Get(Get::new(parse_string(data)))),
        "del" => Ok(Command::Del(Del::new(parse_string(data)))),
        _ => Err(ParseError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::{get_request, del_request, set_request};

    #[test]
    fn parse_get() {
        let get_valid = get_request("test");
        assert_eq!(
            Command::parse_bytes(&get_valid, get_valid.len()).unwrap(),
            Command::Get(Get::new("test")));
    }

    #[test]
    fn parse_del() {
        let del_valid = del_request("test");
        assert_eq!(
            Command::parse_bytes(&del_valid, del_valid.len()).unwrap(),
            Command::Del(Del::new("test")));
    }

    #[test]
    fn parse_set() {
        let set_valid = set_request("key", b"val");
        assert_eq!(
            Command::parse_bytes(&set_valid, set_valid.len()).unwrap(),
            Command::Set(Set::new("key", Bytes::from(b"val".to_vec()))));
    }
}