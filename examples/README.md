# Examples

Simply run the following code:

```shell
cargo run --example md5 PATH/TO/FILE
```

All examples accept multiple arguments:

```shell
$ cargo run --example sha2-224 Cargo.toml LICENSE
Cargo.toml 1d32f0503ca43a98bb377c98e38d5e26fb8ebe520ff4fc419ec43039
LICENSE f2c3541b130a29abc5400732a573ba11a3a30a09435d3c1f15a83f77
```

## Limitations

You cannot calculate a digest from stdin.
