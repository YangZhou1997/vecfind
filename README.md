## Prerequisite

I am using the latest [rust-sgx](https://github.com/fortanix/rust-sgx). You need to put `vecfind` into the `rust-sgx/examples`, or change the relative path in Cargo.toml
I am using rust-nightly-2019-05-22.

### Test vecfind in enclave; 
```shell
./test.sh
```
It will print time value of around **92s**

### Test vecfind in normal world: 
```shell
cd app
cargo run --release
time cargo run --release
```
It will print time value of around **40s**
