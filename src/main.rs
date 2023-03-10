use cds::download_pdfs;
//Searches a path for duplicate files
use clap::Parser;

#[derive(Parser)]
//add extended help
#[clap(
    version = "1.0",
    author = "Selina Liu",
    about = "Downloads large amount of common data set pdfs from different hosts",
    // after_help = "Example: rdedupe search --path . --pattern .txt"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Download { url: String, folder: String },
    // Dedupe {
    //     #[clap(long, default_value = ".")]
    //     path: String,
    //     #[clap(long, default_value = "")]
    //     pattern: String,
    // },
    // //create count with path and pattern defaults for both
    // Count {
    //     #[clap(long, default_value = ".")]
    //     path: String,
    //     #[clap(long, default_value = "")]
    //     pattern: String,
    // },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Download { url, folder }) => {
            //download files from a url
            println!("Downloading files from {} to {}", url, folder);
            let _result = download_pdfs(&url, &folder);
            println!("Downloading complete");
        }
        // Some(Commands::Dedupe { path, pattern }) => {
        //     //dedupe files matching a pattern
        //     //display the progress bar using indicatif
        //     println!("Deduping files in {} matching {}", path, pattern);
        //     let result = cds::run(&path, &pattern);
        //     match result {
        //         Ok(_) => println!("Deduping complete"),
        //         Err(e) => println!("Error: {}", e),
        //     }
        // }
        // Some(Commands::Count { path, pattern }) => {
        //     //count files matching a pattern
        //     println!("Counting files in {} matching {}", path, pattern);
        //     let files = cds::walk(&path).unwrap();
        //     let files = cds::find(files, &pattern);
        //     println!("Found {} files matching {}", files.len(), pattern);
        // }
        None => {
            println!("No command given");
        }
    }
}
