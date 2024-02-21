Nathan Rowan and Trevin Vaughan

Instructions for running:
```sh
cargo run -- --help
```

```
Command line arguments accepted by the scanner

Usage: part1 [OPTIONS] [INPUT_FILES]...

Arguments:
  [INPUT_FILES]...
          toyc source files

Options:
  -d, --debug <DEBUG>
          Display messages that aid in tracing the compilation process

          Possible values:
          - all:     All messages
          - scanner: Scanner messages only

  -v, --verbose
          Display all information

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

To open crate documentation:
```sh
cargo doc --open
```

Example run:
```sh
cargo run -- --debug scanner tests/scanTest.tc tests/allTokens.tc
```