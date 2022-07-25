use std::env::current_dir;
use std::fs;
use std::net::SocketAddr;

use log::LevelFilter;
use log::{error, info, warn};

use clap::arg_enum;
use structopt::StructOpt;



use rustdb::engines;

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
const DEFAULT_ENGINE: Engine = Engine::kvs;

#[derive(StructOpt, Debug)]
#[structopt(name = "kvs-server")]
struct Opt {
    #[structopt(
    long,
    help = "Sets the listening address",
    value_name = "IP:PORT",
    raw(default_value = "DEFAULT_LISTENING_ADDRESS"),
    parse(try_from_str)
    )]
    addr: SocketAddr,
    #[structopt(
    long,
    help = "Sets the storage engine",
    value_name = "ENGINE-NAME",
    raw(possible_values = "&Engine::variants()")
    )]
    engine: Option<Engine>,
}

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    enum Engine {
        kvs,
        sled,
    }
}

fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let mut opt = Opt::from_args();
    let res = current_engine().and_then(move | current_engine()| {

    })
}

fn current_engine() -> Result<Option<Engine>> {
    let engine = current_dir()?.join("engine");
    if !engine .exists() {
        return Ok(None);
    }

    match fs::read_to_string(engine)?.parse() {
        Ok(engine) => Ok(Some(engine)),
        Err(e) => {
            warn!("The content of engine file is invalid: {}", e);
            Ok(None)
        }
    }

}