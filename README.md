# RustDingTalk



#### Building for source
For build the binary just:
```sh
$ cargo build --release --verbose
```
To run as debug, just run this example:
```sh
$ cargo run -- 4.03
```
### Installation
Install simple typing:

```rustdingtalk
cargo install RustDingTalk
```

### Documentation
The documentation, for now, is the help return of tool:

```sh

USAGE:
    rustdingtalk [amount]

FLAGS:
    -h, --help       Prints help information
    -s, --silent     Silent information abount currency result
    -V, --version    Prints version information
    -v, --verbose    Verbose errors

ARGS:
    <amount>    Set amount to compare target [default: 1]
```
