// A command-line tool to search wikipedia and summarize the content both via subcommands
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Yilun Wu")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Yilun Wu")]
    Wiki {
        #[clap(short, long)]
        page: String,
    },
    #[clap(version = "1.0", author = "Yilun Wu")]
    Sumwiki {
        #[clap(short, long)]
        page: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Wiki { page }) => {
            let content = project3::get_wiki_content(&page);
            println!("{}", content);
        }
        Some(Commands::Sumwiki { page }) => {
            let content = project3::get_wiki_content(&page);
            let summary = project3::summarize_content(&content);
            println!("{}", summary);
        }
        None => println!("No subcommand was used"),
    }
}
