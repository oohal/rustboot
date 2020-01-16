To build:

1. get the toolchain, you need nightly for reasons I don't remember

```
rustup toolchain install nightly-x86_64-unknown-linux-gnu
rustup default nightly

rustup component add rust-std-powerpc64-unknown-linux-gnu
rustup component add rustc-dev-powerpc64-unknown-linux-gnu

```

2. Clone this and the fdt-rs fork into a dir:

```
mkdir ~/rust
cd ~/rust
git clone git@github.com:oohal/rustboot.git
git clone git@github.com:oohal/fdt-rs.git
```

3. Build by running rustboot q.sh. If you're lucky it might even work
