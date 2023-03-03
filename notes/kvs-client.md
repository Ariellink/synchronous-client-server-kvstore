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

-[x] value behind `-` or `--` is *short or long flag*`-- flag` is a long flag. See `Arg::long`.  
-[x] value in the `[]` or `<>` is *Value name*


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

## follow-ups
⁉ `Usage: kvs-client get --addr <ipport> <KEY>`顺序不对？？


