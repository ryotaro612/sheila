use std::ffi::OsString;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "sheila")]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,

    // https://stackoverflow.com/questions/76341332/clap-default-value-for-pathbuf
    #[arg(short, long, default_value = get_default_log_path())]
    pub(crate) socket: String,
    // https://poyo.hatenablog.jp/entry/2022/10/10/170000
    #[arg(short, long)]
    pub(crate) verbose: bool,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    #[command(about = "Run the server")]
    Server,
    #[command(about = "Run the client")]
    Client(ClientArgs),
}

#[derive(Debug, Subcommand)]
pub(crate) enum ClientSubCommands {
    #[command(about = "Display")]
    Display(DisplayArgs),
    Stop,
}

#[derive(Debug, Args)]
pub(crate) struct ClientArgs {
    #[command(subcommand)]
    pub(crate) command: ClientSubCommands,
}

#[derive(Debug, Args)]
pub(crate) struct DisplayArgs {
    #[arg()]
    pub(crate) file: String,
}

/**
 * Parses the command line arguments.
 */
pub(crate) fn parse(args: Vec<String>) -> Result<Cli, clap::error::Error> {
    Cli::try_parse_from(args)
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
    let args: Vec<String> = vec!["sheila", "--verbose", "server"]
        .into_iter()
        .map(String::from)
        .collect();

    // actual
    let actual = parse(args).unwrap();

    assert!(actual.verbose);
}

#[test]
fn test_client_has_display_subcommand() {
    let args: Vec<String> = vec!["sheila", "--verbose", "client", "display", "image.png"]
        .into_iter()
        .map(String::from)
        .collect();

    // actual
    let actual = parse(args).unwrap();

    match actual.command {
        Commands::Client(client_args) => match client_args.command {
            ClientSubCommands::Display(args) => {
                assert_eq!("image.png", args.file);
            }
            _ => panic!("unexpected subcommand"),
        },
        _ => panic!("unexpected command"),
    }

    assert!(actual.verbose);
}

#[test]
fn test_client_provides_stop_command() {
    // arrange
    let args: Vec<String> = vec!["sheila", "client", "stop"]
        .into_iter()
        .map(String::from)
        .collect();

    // actual
    let actual = parse(args);
    // assert
    match actual.unwrap().command {
        Commands::Client(client_args) => match client_args.command {
            ClientSubCommands::Stop => {
                // nop
            }
            _ => panic!("unexpected subcommand"),
        },
        _ => panic!("unexpected command"),
    }
}
