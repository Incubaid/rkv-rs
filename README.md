### Prerequisites

1. rkv requires the most recent stable Rust compiler; it can be installed with
   `rustup`.

#### Installing Rust compiler with `rustup`

1. Install [`rustup.rs`](https://rustup.rs/).

2. Clone the source code:

   ```sh
   git clone https://github.com/Incubaid/rkv-rs.git
   cd rkv-rs
   ```

3. Make sure you have the right Rust compiler installed. Run

   ```sh
   rustup override set stable
   rustup update stable
   ```


#### Ubuntu

On Ubuntu, you need a few extra libraries to build rkv. Here's an `apt`
command that should install all of them. If something is still found to be
missing, please open an issue.

```sh
sudo apt-get install pkg-config libssl-dev
```

#### Arch Linux

On Arch Linux, you need a few extra libraries to build rkv. Here's a
`pacman` command that should install all of them. If something is still found
to be missing, please open an issue.

```sh
sudo pacman -S openssl
```

#### Fedora

On Fedora, you need a few extra libraries to build rkv. Here's a `dnf`
command that should install all of them. If something is still found to be
missing, please open an issue.

```sh
sudo dnf install openssl-devel
```


### Building

Once all the prerequisites are installed, compiling rkv should be easy:

```sh
cargo build --release
```

If all goes well, this should place a binary at `target/release/rkv`.

**Note:** On linux the resulting binary would be large due to `debug symbols`, use `strip target/release/rkv` to remove them.
