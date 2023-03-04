use std::{env, process};
use clap::{arg, command, Command};
use kvs::KvStore;
use kvs::{Result};

fn main() -> Result<()> {
    let matches = command!() // requires `cargo` feature
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
                Command::new("get")
                .about("get a vaule from a key: get <key>")
                .arg(arg!(<KEY>).help("A String key").required(true))
                .arg(arg!(-a --addr <ipport> "example: 127.0.0.1:4000").required(true).default_value("127.0.0.1:4000"))
        )
        .subcommand(
            Command::new("set")
                .about("set a key/vaule pair: set <key> <vaule>")
                .arg(arg!(<KEY>).help("A String key").required(true))
                .arg(arg!(<VALUE>).help("A String vaule").required(true))
                .arg(arg!(-a --addr <ipport> "example: 127.0.0.1:4000").required(true).default_value("127.0.0.1:4000"))
        )
        .subcommand(
            Command::new("rm")
                .about("remove the a key/vaule pair: rm <key>")
                .arg(arg!(<KEY>).help("A String key").required(true))
                .arg(arg!(-a --addr <ipport> "example: 127.0.0.1:4000").required(true).default_value("127.0.0.1:4000"))
        )
        .get_matches();
    
    //errors to stderr to stderr

    //fn send_request()
    match matches.subcommand() {
        Some(("get", _matches)) => {
            let key = _matches.get_one::<String>("KEY").unwrap();
            let addr = _matches.get_one::<String>("addr").unwrap();
            // match store.get(key.to_owned()) {
            //     //handle Option<String> ~value
            //     Ok(Some(val)) => println!("{}", val),
            //     Ok(None) => println!("Key not found"),
            //     Err(e) => println!("{:?}", e),
            // }
        },
        Some(("set", _matches)) => {
            let key = _matches.get_one::<String>("KEY").unwrap();
            let value = _matches.get_one::<String>("VALUE").unwrap();
            let addr = _matches.get_one::<String>("addr").unwrap();
            // if let Err(e) = store.set(key.to_owned(), value.to_owned()) {
            //     println!("{:?}",e);
            //     process::exit(-1);
            // }

        },
        Some(("rm", _matches)) => {
            let key = _matches.get_one::<String>("KEY").unwrap();
            let addr = _matches.get_one::<String>("addr").unwrap();
            // if let Err(_e) = store.remove(key.to_owned()) {
            //     println!("Key not found");
            //     process::exit(-1);
            // }
        },
        _ => process::exit(-1),
    }
    Ok(())
}