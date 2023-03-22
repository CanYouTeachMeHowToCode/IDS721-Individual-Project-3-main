// A command-line tool to search on wikipedia, summarize the found content and perform question answering based on the content
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
    #[clap(version = "1.0", author = "Yilun Wu")]
    Answerquestion {
        #[clap(short, long)]
        question: String,
        page: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Wiki { page }) => {
            let content = project3::get_wiki_content(&page);
            println!("{content}");
        }
        Some(Commands::Sumwiki { page }) => {
            let content = project3::get_wiki_content(&page);
            let summary = project3::summarize_content(&content);
            println!("{summary}");
        }
        Some(Commands::Answerquestion { question, page }) => {
            let answer = project3::question_answering(&question, &page);
            println!("Answer: {answer}");
        }
        None => println!("No Command Given."),
    }
}
