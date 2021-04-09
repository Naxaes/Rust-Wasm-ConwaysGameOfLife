# Conway's Game of Life



### src
Contains the source code for Rust.

### www
Contains the source code for Javascript.

### pkg
Contains the generated bridge between Rust and Javascript.


### test
Tests for the code in 

### TODO
* What does `www/.bin/create-wasm.app.js` do?
* Are the travis.yml files used?
* Learn more about `www/package.json`.
* Learn more about `www/webpack.config.js`.
* What is `.appveyor.yml`?
* What is `.cargo.ok`?


### Profiling (MacOS)
1. Run benchmark `cargo +nightly bench | tee <outfile>.txt`.
2. Profile benchmark executable (mentioned in the stdout of the bench command) `instruments -l 30000 -t Time\ Profiler <target>`.
3. Open trace file `open -a Instruments <trace_file>`.
