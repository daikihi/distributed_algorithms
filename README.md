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


# memo : installing spec-kit

when you are using mac os

How to install uv command.

You should not use both of pip and uv at same time.


```bash
mkdir some_tmp_dir
cd some_tmp_dir
curl -LsSf https://astral.sh/uv/install.sh | sh
vim ~/.zshrc
source ~/.zshrc
```

```bash
cd your_project_dir
git clone https://github.com/github/spec-kit.git
cd spec-kit
uv pip install .
cd ../
specify init --here
# answer some question for specify
```
