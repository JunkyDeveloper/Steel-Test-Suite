# Flint&Steel

The goal of this project is to build testing suite for [SteelMC](https://github.com/4lve/SteelMC).
Which should test automatic blocks, items and more.

## Current Plan
 - [ ] Write a chunk generator
 - [ ] Write a runner which handels the communication with SteelMC

## Usage

Currently, all tests will be stored in `./test` directory.

The basic configuration will run all tests which are available
```bash
cargo run --package flint-steel --bin flint-steel 
```

### Indexing
would be recommended with a lot of tests, to have it available for a quicker start or if you want to only run some tests

```bash
cargo run --package flint-steel --bin flint-steel -- index 
```

### Scoped tests
You have also the option to run tests from different tags a tag is specified in the test file.
Each tag is seperated via a space.

```bash
cargo run --package flint-steel --bin flint-steel -- redstone wall fences 
```

To run all tests without a tag you can do:
```bash
cargo run --package flint-steel --bin flint-steel -- default 
```
You can add the default tag to all other tags

## Test files
Currently the test files will be in json written.
This describes the full test cases if what needs to be tested and done.

## Future Ideas
 - Mock Client to run tests on vanilla server and on every minecraft server
 - implement a powerful testing api which can be implemented by other minecraft server to use also this suite
 - For more repetitive test cases there will be [jsonnet](https://jsonnet.org/) available