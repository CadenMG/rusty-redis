# Rusty Redis
A Rust implementation of a subset of [Redis](https://redis.io/)


# Quick Start
To start a Redis server, execute from the project root:
`cargo r --bin server`

To start a Redis client, execute from the project root:
`cargo r --bin client`


# Supported Commands
## Get
Gets the value for the given key or returns an error

Ex usage:
`get key` -> `val`
## Set
Sets the value to the given key

Ex usage:
`set key val` -> `Success`
## Del
Delets the value for the given key or returns an error

Ex usage:
`del key` -> `Success`
