use anyhow::Result;
use clap::Parser;
use fnr::{
    args::Cli,
    search::{search_in_file, search_in_folder},
    utils::PathExt,
};

fn main() -> Result<()> {
    // Gather args passed with the program
    let args = Cli::parse();

    match args.path.is_directory()? {
        true => search_in_folder(&args)?,
        false => search_in_file(&args)?,
    }

    Ok(())
}
