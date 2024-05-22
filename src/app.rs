use clap::Parser;
use std::path::PathBuf;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, Parser)]
#[command(
    name = "fnr (find & replace)",
    version,
    after_help = "
Examples:
    Find a pattern 'old' in files of the current folder
    
    $ fnr old

    Find a pattern 'old' in files of the current folder, excluding the 'target' folder

    $ fnr old . --omit target // or -o target

    You can also omit multiple folders

    $ fnr old . --omit target/ build/

    Including hidden files

    $ fnr old . --hidden --omit target/ build/

    Only search for files with a specific extension

    $ fnr old . --type .rs

    Ignore files with a specific extension

    $ fnr old . --type-not .rs

    You can also search / ignore multiple file types
    Here, we search for files with .rs and .toml extension, but ignore .txt and .md files

    $ fnr old . --type .rs .toml --type-not .txt .md
"
)]
pub struct Options {
    /// The pattern to search for
    pub pattern: String,

    /// The path of the folder / file to read
    /// Default is the current directory
    pub path: Option<PathBuf>,

    #[arg(long, help = "Include hidden files in the search")]
    hidden: bool,

    /// File or directory(ies) to exclude
    #[clap(long, short, alias = "exclude, ignore, skip", num_args= 0..,)]
    pub omit: Vec<PathBuf>,

    #[clap(
        long,
        short,
        help = "Print additional information about files searched or errors"
    )]
    pub verbose: bool,

    /// Match case when searching for content
    #[clap(long, short)]
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




pub fn run() {
    let args = Options::parse();
    println!("{:?}", args);
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
        cmd.args(&["old", ".", "-t", "rs", "-T", "json"])
            .assert()
            .success();
    }
}
