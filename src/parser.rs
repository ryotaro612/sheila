use std::{ffi::OsString, path::PathBuf};

use clap::{
    builder::{EnumValueParser, OsStr},
    Args, Parser, Subcommand,
};

#[derive(Debug, Parser)]
#[command(name = "sheila")]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Commands,

    // https://stackoverflow.com/questions/76341332/clap-default-value-for-pathbuf
    #[arg(short, long, default_value = get_default_log_path())]
    socket: String,
    // https://poyo.hatenablog.jp/entry/2022/10/10/170000
     #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Run the server")]
    Server(ServerArgs),
    #[command(about = "Run the client")]
    Client(ClientArgs),
}

#[derive(Debug, Args)]
struct ServerArgs {
    // #[arg(short, long, default_value = get_default_log_path())]
    // socket: String,
}

#[derive(Debug, Args)]
struct ClientArgs {

}


pub(crate) fn parse(args: Vec<String>) -> Result<Cli, clap::error::Error> {
    //let args = Cli::try_parse_from(["example", "server", "--port", "8080"]);
    Cli::try_parse_from(args)
    // match args {
    //     Ok(cli) => {
    //         match cli.command {
    //             Commands::Server(server_args) => {
    //                 println!("Running server on port: {}", server_args.port);
    //             }
    //             Commands::Client(client_args) => {
    //                 println!("Connecting to socket: {}", client_args.socket);
    //             }
    //         }
    //     }
    //     Err(e) => {
    //         eprintln!("Error parsing arguments: {}", e);
    //     }
    // }
}
/**
 * Defines the default socket file path.
 */
fn get_default_log_path() -> OsString {
    let mut p = std::env::temp_dir();
    p.push("sheila.socket");
    p.into_os_string()
}

#[test]
fn test_default_socket_file_is_defined() {
    // arrange
    let args: Vec<String> = vec!["sheila", "server"]
        .into_iter()
        .map(String::from)
        .collect();

    // actual
    let actual = parse(args).unwrap();

    // assert
    let mut expected = std::env::temp_dir();
    expected.push("sheila.socket");

    assert_eq!(actual.socket, expected.to_str().unwrap());
}


#[test]
fn test_verbose_option_is_available() {
    // arrange
    let args: Vec<String> = vec!["sheila", "--verbose", "client"]
        .into_iter()
        .map(String::from)
        .collect();

    // actual
    let actual = parse(args).unwrap();

    assert!(actual.verbose);
}

