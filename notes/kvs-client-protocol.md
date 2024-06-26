# kvs-client 

`kvs-client` is a command-line key-value store client

`kvs-Client` implements the functionality required for kvs-client to speak to kvs-server

```rust
./kvs-client --help
a command-line key-value store client
Usage: kvs-client [COMMAND] [--addr IP-Port]
Commands:
  get   get a vaule from a key: get [key]
  set   set a key/vaule pair: set [key] [vaule]
  rm    remove the a key/vaule pair: rm [key]
  help  Print this message or the help of the given subcommand(s)
Options:
  -h, --help     Print help information
  -V, --version  Print version information

```
kvs-client 需要在projet2 的基础上加上 `[--addr IP-Port]`   
--addr： 如果不指定，那就指定`127.0.0.1:4000`

- `kvs-client set <key> <value> [--addr IP-Port] `  
- `kvs-client get <KEY> [--addr IP-PORT]`  
- `kvs-client rm <KEY> [--addr IP-PORT]`  
> Print an error and return a `non-zero exit code` on server error, or if `IP-PORT` does not parse as an address.   

> for `rm` command: A "key not found" is also treated as an error in the "rm" command.
- `kvs-client -V`  Print the version

## Implement with clap crate
`Struct clap::Command`下设subcommand方法。
`get`, `set`,`rm` 都是subcommand中嵌套的Command实例。

### 如何增加一个参数 `--addr IP-Port`
需要参考[`marco arg!()`](https://docs.rs/clap/latest/clap/macro.arg.html#)的接口  

#### Macro clap::arg Syntax
[explicit name] [short] [long] [value names] [...] [help string]

- [x] value behind `-` or `--` is *short or long flag*`-- flag` is a long flag. See `Arg::long`.  
- [x] value in the `[]` or `<>` is *Value name*


####  如何实现 default value with `127.0.0.1:4000`?  
> `clap::subcommand` 提供 `default_value()`  
(https://docs.rs/clap/latest/clap/_tutorial/index.html#defaults)   
**We’ve previously showed that arguments can be `required` or `optional`. When `optional`, you work with a Option and can `unwrap_or`. Alternatively, you can set `Arg::default_value`.**

> orignal:
``` rust
command!()
.subcommand(
        Command::new("get")
        .about("get a vaule from a key: get [key]")
        .arg(arg!([KEY]).help("A String key").required(true)),
        
)
```
> new:
``` rust
command!()
.subcommand(
        Command::new("get")
        .about("get a vaule from a key: get [key]")
        //[value name]
        .arg(arg!([KEY]).help("A String key").required(true))
        // short flag: -a
        // long flag: --addr
        .arg(arg!(-a --addr <ipport> "example: 127.0.0.1:4000").required(true).default_value("127.0.0.1:4000")),   
)

```
> Termianl output
```shell
➜  synchronous-client-server-kvstore git:(main) ✗ target/debug/kvs-client -help
A key-vaule store

Usage: kvs-client <COMMAND>

Commands:
  get   get a vaule from a key: get <key>
  set   set a key/vaule pair: set <key> <vaule>
  rm    remove the a key/vaule pair: rm <key>
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

```shell
➜  synchronous-client-server-kvstore git:(main) ✗ target/debug/kvs-client get -help
get a vaule from a key: get <key>

Usage: kvs-client get --addr <ipport> <KEY>

Arguments:
  <KEY>  A String key

Options:
  -a, --addr <ipport>  example: 127.0.0.1:4000 [default: 127.0.0.1:4000]
  -h, --help           Print help information
  -V, --version        Print version information
```
### 如何实现 All error messages should be printed to stderr.

```rust 
if let Err(err: KVStoreErr) = send_request(matches) {
  eprintln!("{:?}", err);
  process::exit(-1);
}
```

### addr 解析
Struct clap::ArgMatches
`pub fn get_one<T: Any + Clone + Send + Sync + 'static>(&self, id: &str) -> Option<&T>`  
> get_one 接受一个（id）, `arg!(id)` , See `Arg::id`.  

`Macro clap::arg` 有explict name, 应该是主要用于`get_one()`的id参数, 但是这个选项是optional，如果被省略（omitted），那么arg的名称会从按以下优先顺先确定｀id｀：  

    1. Explicit Name
    2. Long
    3. Value Name

#### get_one() Usage example: 
```rust
let cmd = Command::new("prog")
    .args(&[
        arg!(--config <FILE> "a required file for the configuration and no short"),
        arg!(-d --debug ... "turns on debugging information and allows multiples"),
        arg!([input] "an optional input file to use")
    ]);

let m = cmd.try_get_matches_from(["prog", "--config", "file.toml"]).unwrap();
assert_eq!(m.get_one::<String>("config").unwrap(), "file.toml");
assert_eq!(*m.get_one::<u8>("debug").unwrap(), 0);
assert_eq!(m.get_one::<String>("input"), None);
```
#### In kvs_client command arg matches:
```rust
Some(("set", _matches)) => {
            let key = _matches.get_one::<String>("KEY").unwrap();
            let value = _matches.get_one::<String>("VALUE").unwrap();
            let addr = _matches.get_one::<String>("addr").unwrap();
        },
```

## follow-ups
⁉ `Usage: kvs-client get --addr <ipport> <KEY>`顺序不对？？


## send_request()
`fn send_request(matches:ArgMatches) -> Result<()>`
这个函数在main()中被调用，接受的参数就是`get_matches()`返回的结果`ArgMatches`.

### ArgMatches
`ArgMatches`就是将argments用vector存储为类成员的类。
他还有一些其他的成员是subcommands等等。
```rust
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ArgMatches {
    #[cfg(debug_assertions)]
    pub(crate) valid_args: Vec<Id>,
    #[cfg(debug_assertions)]
    pub(crate) valid_subcommands: Vec<Str>,
    pub(crate) args: FlatMap<Id, MatchedArg>,
    pub(crate) subcommand: Option<Box<SubCommand>>,
}
```
用于在程序runtime时，get the information about 由用户指定的arguments。要获得一个这个struct的新实例，你需要使用对command struct使用'.get_matches()'。
实际上因为`app struct`在clap latest version中被改写为`clap::builder::Command` struct.
并为clap struct实现了get_matches()方法。
```rust
    #[inline]
    pub fn get_matches(self) -> ArgMatches {
        self.get_matches_from(env::args_os())
    }
```
*example*
```rust
     # use clap::{Command, Arg};
     let matches = Command::new("myprog")
         // Args and options go here...
         .get_matches();
```
#### impl ArgMatches
.get_one()
`id`就是`ArgMatches`的成员valid_args的vector的元素。
` pub fn get_one<T: Any + Clone + Send + Sync + 'static>(&self, id: &str) -> Option<&T> {
        MatchesError::unwrap(id, self.try_get_one(id))
    }`

*example* 
```rust
let m = Command::new("myapp")
    .arg(Arg::new("port")
        .value_parser(value_parser!(usize))
        .action(ArgAction::Set)
        .required(true))
    .get_matches_from(vec!["myapp", "2020"]);

let port: usize = *m
    .get_one("port") //拿到port对应的arg
    .expect("`port`is required");
assert_eq!(port, 2020);
```
要指定get_one()返回值可以`.get_one::<String>("id")`
```rust
fn send_request(matches:ArgMatches) -> Result<()> {
        match matches.subcommand() {
            Some(("get", _matches)) => {
                let key = _matches.get_one::<String>("KEY").unwrap();
                let addr = _matches.get_one::<String>("addr").unwrap();
```
##

