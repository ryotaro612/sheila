use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "example")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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
    #[arg(short, long)]
    port: u16,
}

#[derive(Debug, Args)]
struct ClientArgs {
    #[arg(short, long)]
    socket: String,
}

pub(crate) fn parse() {
    let args = Cli::try_parse_from(["example", "server", "--port", "8080"]);
    match args {
        Ok(cli) => {
            match cli.command {
                Commands::Server(server_args) => {
                    println!("Running server on port: {}", server_args.port);
                }
                Commands::Client(client_args) => {
                    println!("Connecting to socket: {}", client_args.socket);
                }
            }
        }
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
        }
    }
}

#[test]
fn test_parse() {
    parse();
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