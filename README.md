This branch was renamed to "historical" to preserve my first in Rust's history

# Cowfetch
Cowsay+Neofetch made in Rust

## Building from source:
> I will assume you already have RUST installed on your system<br>
I haven't tested this on a windows host yet

```sh
git clone https://github.com/S0raWasTaken/cowfetch
cd cowfetch
cargo build --release
```

## Installing the built/downloaded binary
```sh
# I will assume you are in the project main directory
sudo mv ./target/release/cowfetch /usr/local/bin

# If you did download the binary from a release, just place it on /usr/local/bin and it should work
```
