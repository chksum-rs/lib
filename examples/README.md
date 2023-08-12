# Examples

Simply run the following code:

```shell
cargo run --example md5 PATH/TO/FILE
```

All examples accept multiple arguments:

```shell
$ cargo run --example sha2_224 Cargo.toml LICENSE
Cargo.toml 96527686f8e021ece158227116caf53e071a30236dd0d62561ea41c9
LICENSE 99258bca0d23c69388dd53412f1009132753b89459359a401a6ed158
```

## Limitations

You cannot calculate a digest from stdin.
