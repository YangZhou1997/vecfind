set -e

#Build custom runner
cd runner
cargo +nightly build --release
cd -

#Build APP
cd app
cargo +nightly build --target=x86_64-fortanix-unknown-sgx --release
cd -

#Convert the APP
ftxsgx-elf2sgxs app/target/x86_64-fortanix-unknown-sgx/release/app --heap-size 0x5d80000 --stack-size 0x5d80000 --threads 1

#Execute
runner/target/release/runner app/target/x86_64-fortanix-unknown-sgx/release/app.sgxs
