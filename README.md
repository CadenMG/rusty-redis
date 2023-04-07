# Rusty Redis
A Rust implementation of a subset of [Redis](https://redis.io/)


# Quick Start
To start a Redis server, execute from the project root:
`cargo r --bin server`

To use the built-in Redis client, execute from the project root:
`cargo r --bin client -- {cmd}`

Where `cmd` is one of:
- `get key`
- `set key val`
- `del key`


# Supported Commands
## Get
Gets the value for the given key or returns nothing if the key DNE

Ex usage:
`get key` -> `val`
`get key` -> `()`

## Set
Sets the value to the given key and returns the previous value for the key, if any

Ex usage:
`set key val` -> `()`
`set key val` -> `old val`

## Del
Delets and returns the value for the given key, or returns nothing if the key DNE

Ex usage:
`del key` -> `val`
`del key` -> `()`

# Command / Response Protocol
## Command
The command protocol is as follows:
```
+------+------+-------+------+------+-----+------+------+
| nstr | len1 | str1  | len2 | str2 | ... | lenn | strn |
+------+------+-------+------+------+-----+------+------+
```
Where nstr is the number of strings, and the len is the length of the following string. 
Both are 32 bit ints.

## Response
The response is a 32-bit status code followed by the response string:
```
+--------+---------+
| status | data... |
+--------+---------+
```
`status` is one of:
- `1` if the command was successful and data is non-empty
- `2` if the command was successful and data is empty
- `3` if the command was not successful 
