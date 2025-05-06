# sheila

`sheila` lets you use movies as animated wallpapers for your Wayland session.

## Installation

`sheila` is written in Rust and published on [crates.io](https://crates.io/crates/sheila):

```sh
cargo install sheila
```

## Usage

sheila consists of a client and a server.

To start the server, run:

```sh
sheila server
```

While the server is running, you can send it movie files to play:

```sh
sheila client play --monitor DP-1 movie0.mp4 movie1.mp4
```

Use the `--random` option to have the server play a random movie:

```sh
sheila client play --monitor DP-1 --random movie0.mp4 movie1.mp4
```
``