use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use crate::{console::Console, Settings, Walker};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, Parser)]
#[command(
    name = "fnr (find & replace)",
    version,
    after_help = "
Examples:
    Find a pattern 'old' in files of the current folder
    
    $ fnr old

    For any lookup, you can activate verbose mode

    $ fnr old --verbose // or -v

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
"
)]
pub struct Options {
    /// The pattern to search for.
    pub pattern: String,

    /// The path of the folder / file to read.
    /// Default is the current directory.
    pub path: Option<PathBuf>,

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

    #[clap(long, short, help = "Perform a case-insensitive search. Default is case-sensitive.")]
    pub ignore_case: bool,

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

#[allow(unused_variables)]
pub fn run() -> Result<()> {
    let args = Options::parse();

    // Desctructure the Options struct
    // So we can use the variables directly
    let Options {
        pattern,
        path,
        hidden,
        omit,
        verbose,
        ignore_case,
        selected_file_types,
        ignored_file_types,
    } = args;

    let console = Console::new(verbose);

    let settings = Settings {
        verbose,
        omit_pattern: omit,
        search_hidden: hidden,
        ignore_case,
        selected_file_types,
        ignored_file_types,
    };

    // If no path is provided, use the current directory
    let path = path.unwrap_or_else(|| PathBuf::from("."));

    let walker = Walker::new(pattern, path, settings, console);

    walker.run()
}

#[cfg(test)]
mod tests {

    // use super::*;
    use assert_cmd::Command;

    #[test]
    fn test_no_arguments() {
        let mut cmd = Command::cargo_bin("fnr").unwrap();
        cmd.assert().failure();
    }

    #[test]
    fn test_basic_search() {
        let mut cmd = Command::cargo_bin("fnr").unwrap();
        cmd.arg("old").assert().success();
    }

    #[test]
    fn test_omit() {
        let mut cmd = Command::cargo_bin("fnr").unwrap();
        cmd.args(&["old", ".", "--omit", "target"])
            .assert()
            .success();
    }

    #[test]
    fn test_hidden_verbose() {
        let mut cmd = Command::cargo_bin("fnr").unwrap();
        cmd.args(&["old", ".", "--hidden", "--verbose"])
            .assert()
            .success();
    }

    #[test]
    fn test_file_types() {
        let mut cmd = Command::cargo_bin("fnr").unwrap();
        cmd.args(&["old", ".", "-t", "*rs", "-T", "*json"])
            .assert()
            .success();
    }
}
