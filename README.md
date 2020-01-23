Rust cargo seems to do some weird/undocumented file caching when running build.sh scripts. Just run ./demo.sh to see.

## Expected behavior:

* Executing 'cargo run', 'rm demo.txt', 'cargo run' should work just fine.

## Seen behavior:

* The second run of 'cargo run' fails.
