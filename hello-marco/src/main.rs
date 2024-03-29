use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Doug Copeland", about = "Marco Polo game")]

struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Play {
        #[clap(short, long)]
        name: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Play { name }) => {
            let result = hello_marco::marco_polo(&name);
            println!("{}", result);
        }
        None => {
            println!("No command was used");
        }
    }
}
