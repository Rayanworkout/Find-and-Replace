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
    
    $ fnr old .

    Find a pattern 'old' in files of the current folder, excluding the 'target' folder

    $ fnr old . --omit target // or -o target

    You can omit multiple folders by separating them with a comma

    $ fnr old . --omit target,build
"
)]
pub struct Cli {
    /// The pattern to search for
    pub old_pattern: String,

    // The new pattern to replace the old with
    // new_pattern: String,
    /// The path of the folder / file to read
    pub path: PathBuf,

    /// File or directory to exclude
    #[clap(long, short, alias = "exclude, ignore, skip", value_delimiter = ',')]
    pub omit: Option<Vec<PathBuf>>,

    /// Display informations about the file being read, the files that cannot be read ...
    #[clap(long, short)]
    pub verbose: bool,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_args_are_parsed() {
        let cli = Cli {
            old_pattern: "old".to_string(),
            path: PathBuf::from("path"),
            omit: Some(vec![PathBuf::from("omit")]),
            verbose: false,
        };

        assert_eq!(cli.old_pattern, "old");
        assert_eq!(cli.path, PathBuf::from("path"));
        assert_eq!(cli.omit, Some(vec![PathBuf::from("omit")]));
        assert_eq!(cli.verbose, false);
    }
}
