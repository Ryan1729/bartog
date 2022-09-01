[gdb]
path=./rust-gdb

[commands]
Compile bartog=shell cargo b --bin bartog --profile debugging
Run bartog=file target/debugging/bartog;run&