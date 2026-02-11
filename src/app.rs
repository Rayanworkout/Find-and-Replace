use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use crate::{Settings, Walker};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, Parser)]
#[command(
    name = "fnr (find & replace)",
    version,
    after_help = "
Examples:
    Find a pattern 'old' in files of the current folder
    
    $ fnr old

    To perform a simple lookup, you can activate lookup mode

    $ fnr old --lookup // or -l

    You can also choose to perform a case-insensitive search

    $ fnr old --ignore-case // or -i

    Find a pattern 'old' in files of the current folder, excluding the 'target' folder

    $ fnr old . --omit target // or -o target

    You can also omit multiple folders

    $ fnr old . --omit target/ build/

    Including hidden files in your search

    $ fnr old . --hidden --omit target/ build/

    Only search for files with a specific extension (use glob patterns)

    $ fnr old . --type *rs // or -t *rs

    Ignore files with a specific extension

    $ fnr old . --type-not *rs // or -T *rs

    You can also search / ignore multiple file types
    Here, we search for files with .rs and .toml extension, but ignore .txt and .md files

    $ fnr old . --type *rs *toml --type-not *txt *md

    If a .fnrignore file exists in the search tree, matching paths are skipped automatically
"
)]
pub struct Options {
    #[arg(help = "The pattern to search for.", required = true)]
    pub pattern: String,

    #[arg(help = "The new pattern to replace the old pattern.", required = true)]
    pub new_pattern: String,

    /// The path of the folder / file to read.
    /// Default is the current directory.
    pub path: Option<PathBuf>,

    #[arg(long, help = "Write changes to disk.")]
    write: bool,

    #[arg(long, help = "Include hidden files in the search.")]
    hidden: bool,

    /// File or directory(ies) to exclude.
    #[clap(long, short, alias = "exclude, ignore, skip", num_args= 0..,)]
    pub omit: Vec<PathBuf>,

    #[clap(
        long,
        short,
        help = "Print additional information about files searched or errors."
    )]
    pub verbose: bool,

    #[clap(
        long,
        short,
        help = "Perform a case-insensitive search. Default is case-sensitive."
    )]
    pub ignore_case: bool,

    #[clap(
        long,
        short,
        help = "Only perform a lookup instead of replacement, defaults to false."
    )]
    pub lookup: bool,

    #[arg(
        short = 't',
        long = "type",
        help = "Only search files matching <file_type> or glob pattern.",
        num_args= 0..,
    )]
    selected_file_types: Vec<String>,

    #[arg(
        short = 'T',
        long = "type-not",
        help = "Ignore files matching <file_type> or glob pattern.",
        num_args = 0..,
    )]
    ignored_file_types: Vec<String>,
}

pub fn run() -> Result<()> {
    let args = Options::parse();

    // Destructure the Options struct
    // So we can use the variables directly
    let Options {
        pattern,
        new_pattern,
        path,
        hidden,
        omit,
        verbose,
        ignore_case,
        lookup,
        selected_file_types,
        ignored_file_types,
        write,
    } = args;

    let settings = Settings {
        verbose,
        omit_pattern: omit,
        search_hidden: hidden,
        ignore_case,
        lookup,
        selected_file_types,
        ignored_file_types,
        write,
    };

    // If no path is provided, use the current directory
    let path = path.unwrap_or_else(|| PathBuf::from("."));

    let pattern = match ignore_case {
        true => pattern.to_lowercase(),
        false => pattern,
    };

    let walker = Walker::new(pattern, new_pattern, path, settings);

    walker.run()
}
