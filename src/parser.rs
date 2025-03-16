use std::{ffi::OsString, path::PathBuf};

use clap::{builder::{EnumValueParser, OsStr}, Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "sheila")]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Commands,

    // #[arg(global=true)]  // <-- here
    // file: Option<String>
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
    // https://stackoverflow.com/questions/76341332/clap-default-value-for-pathbuf
    #[arg(short, long, default_value = get_default_log_path())]
    socket: String,
}

#[derive(Debug, Args)]
struct ClientArgs {
    #[arg(short, long)]
    socket: String,
}

fn get_default_log_path() -> OsString {
    let mut  a = std::env::temp_dir();
    a.push("sheila.socket");
    a.into_os_string()
}

pub(crate) fn parse(args: Vec<String>) -> Result<Cli, clap::error::Error>{
    //let args = Cli::try_parse_from(["example", "server", "--port", "8080"]);
    let args = Cli::try_parse_from(args);
    return args;
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

#[test]
fn test_default_socket_file_is_defined() {
    // arrange
    let args: Vec<String> = vec!["sheila", "server"].into_iter().map(String::from).collect();

    // actual
    let actual = parse(args);

    // assert 
    match actual {
        Ok(c) => {
            match c.command {
                Commands::Server(server_args) => {
                    let mut dir = std::env::temp_dir();
                    dir.push("sheila.socket");
                    assert_eq!(server_args.socket, dir.to_str().unwrap());
                }
                _ => {
                    assert!(false, "the expected subcommand is server.");
                }
            }
        }
        Err(e) => {
            assert!(false, "Error parsing arguments: {}", e);
        }
    }
}
// use clap::{Args, Parser, Subcommand};

// #[derive(Debug, Parser)]
// struct Cli {
//     #[command(subcommand)]
//     command: Commands,
// }

// #[derive(Debug, Subcommand)]
// enum Commands {
//     #[command(about = "help for hoge")]
//     Server(ClientArgs),
//     // #[command(about = "help for hoge")]
//     // Hoge {
//     //     #[arg(short, long)]
//     //     opt: String,
//     // },
//     // #[command(about = "help for fuga")]
//     // Fuga(FugaArgs),
// }

// #[derive(Debug, Args)]
// struct ClientArgs {
//     #[arg(short, long)]
//     socket: String,
// }

// #[derive(Debug, Args)]
// struct FugaArgs {
//     #[arg(short, long)]
//     opt: String,
// }

// /**
//  * 
//  */
// pub(crate) fn parse() {
//     let b = String::from("あ");
//     let x: String = b.into();
//     let a = Cli::try_parse_from([ "server", "help"]);

//     println!("{:?}", a);
//     //println!("{a}");
// }
// // sub command https://poyo.hatenablog.jp/entry/2022/10/10/170000#%E3%82%B5%E3%83%96%E3%82%B3%E3%83%9E%E3%83%B3%E3%83%89

// #[test]
// fn math_works() {
//     parse();
// //assert!(x.is_positive());
// //assert_eq!(x + 1, 2);
// }