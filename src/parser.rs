/**
 This module defines command-line commands and their arguments.
*/
use std::ffi::OsString;

use clap::{Args, Parser, Subcommand};

/**
 * Parses the command line arguments.
 */
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

impl Cli {
    /**
     * Parses command line arguments.
     */
    pub(crate) fn parse(args: Vec<String>) -> Result<Cli, String> {
        Cli::try_parse_from(args).map_err(|e| e.to_string())
    }
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    #[command(about = "Runs the server")]
    Server,
    #[command(about = "Runs the client")]
    Client(ClientArgs),
}

#[derive(Debug, Args)]
pub(crate) struct ClientArgs {
    #[command(subcommand)]
    pub(crate) command: ClientSubCommands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum ClientSubCommands {
    #[command(about = "Plays an MP4 file.")]
    Play(PlayArgs),
    Status,
    Stop,
}

/**

*/
#[derive(Debug, Args)]
pub(crate) struct PlayArgs {
    /**
     A path to an MP4 file.
    */
    #[arg()]
    pub(crate) file: String,

    /**
    The name of a monitor to play a movie.
    */
    #[arg(long)]
    pub(crate) monitor: Option<String>,
}

/**
Defines the default path of the socket file.
*/
fn get_default_log_path() -> OsString {
    let mut p = std::env::temp_dir();
    p.push("sheila.socket");
    p.into_os_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default_socket_file_is_defined() {
        // arrange
        let args: Vec<String> = vec!["sheila", "server"]
            .into_iter()
            .map(String::from)
            .collect();

        // actual
        let actual = Cli::parse(args).unwrap();

        // assert
        let mut expected = std::env::temp_dir();
        expected.push("sheila.socket");

        assert_eq!(actual.socket, expected.to_str().unwrap());
    }

    #[test]
    fn test_server_subcommand_accepts_verbose_option() {
        // arrange
        let args: Vec<String> = vec!["sheila", "--verbose", "server"]
            .into_iter()
            .map(String::from)
            .collect();

        // actual
        let actual = Cli::parse(args).unwrap();
        // assert
        assert!(actual.verbose);
    }

    #[test]
    fn test_client_subcommand_accepts_verbose_option() {
        // arrange
        let args: Vec<String> = vec!["sheila", "--verbose", "client", "stop"]
            .into_iter()
            .map(String::from)
            .collect();

        // actual
        let actual = Cli::parse(args).unwrap();
        // assert
        assert!(actual.verbose);
    }

    #[test]
    fn test_client_has_display_subcommand() {
        let args: Vec<String> = vec!["sheila", "--verbose", "client", "display", "image.png"]
            .into_iter()
            .map(String::from)
            .collect();

        // actual
        let actual = Cli::parse(args).unwrap();

        match actual.command {
            Commands::Client(client_args) => match client_args.command {
                ClientSubCommands::Play(args) => {
                    assert_eq!(None, args.monitor);
                    assert_eq!("image.png", args.file);
                }
                _ => panic!("unexpected subcommand"),
            },
            _ => panic!("unexpected command"),
        }

        assert!(actual.verbose);
    }

    #[test]
    fn test_display_command_has_optional_monitor() {
        let args = arrange(vec![
            "sheila",
            "--verbose",
            "client",
            "display",
            "--monitor",
            "eDP-1",
            "image.png",
        ]);
        // actual
        let actual = parse(args).unwrap();

        match actual.command {
            Commands::Client(client_args) => match client_args.command {
                ClientSubCommands::Play(args) => {
                    assert_eq!(Some(String::from("eDP-1")), args.monitor);
                }
                _ => panic!("unexpected subcommand"),
            },
            _ => panic!("unexpected command"),
        }

        assert!(actual.verbose);
    }

    #[test]
    fn client_provides_stop_command() {
        // arrange
        let args = arrange(vec!["sheila", "client", "stop"]);
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

    #[test]
    fn client_has_status_command() {
        let args = arrange(vec!["sheila", "client", "status"]);

        let actual = parse(args);
        if let Commands::Client(ClientArgs {
            command: ClientSubCommands::Status,
        }) = actual.unwrap().command
        {
        } else {
            panic!("unexpected command");
        }
    }
    fn arrange(v: Vec<&str>) -> Vec<String> {
        v.into_iter().map(|s| s.to_string()).collect()
    }
}
