# distributed_algorithms

This project is just my studying in rust.

## How to execute

```
cargo run mode
```

Modes:

  simple   - Run simple ping pong
  
  multiple - Run multiple ping pong
  
  lc - Run lamport clock

ex

```
cargo run lc
```

## Algorithms

### PingPong

this is the most simple ping pong program using multi-threading and mpsc channel. This implementation has only one both way of sending and receiving message.

#### simple ping pong

### lamport clock
LamportClock is one of the simplest logical clock algorithm. It is good excersize trying to write distributed algorithm.
