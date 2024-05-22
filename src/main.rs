use anyhow::Result;
use clap::Parser;
use fnr::{
    args::Cli,
    search::{search_in_file, walk_folders},
    utils::PathExt,
};

fn main() -> Result<()> {
    // Gather args passed with the program
    let args = Cli::parse();

    let current_folder = &std::env::current_dir()?;

    // If path is None, we assign the current directory
    let path = match &args.path {
        Some(path) => path,
        None => current_folder,
    };

    match path.is_directory() {
        Some(result) => match result {
            true => walk_folders(&path, &args.omit, &args.pattern)?,
            false => search_in_file(&path, &args.pattern)?,
        },
        None => eprintln!("Failed to read the following path: {:?}", args.path),
    }

    Ok(())
}
