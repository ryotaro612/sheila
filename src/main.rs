use log;
use std::process::{self};
mod client;
mod command;
mod logger;
mod parser;
mod server;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // https://docs.rs/clap/latest/clap/type.Error.html
    let cli = parser::parse(args).map_err(|err| err.exit()).unwrap();

    logger::init_log(cli.verbose);

    match cli.command {
        parser::Commands::Server => server::run(cli.socket.as_str()),
        parser::Commands::Client(client_args) => {
            client::run(cli.socket.as_str(), client_args.command)
        }
    }
    .unwrap_or_else(|e| {
        log::error!("{}", e);
        process::exit(1);
    });
}
