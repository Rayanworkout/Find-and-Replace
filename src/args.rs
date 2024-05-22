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
    pub pattern: String,

    // The new pattern to replace the old with
    // new_pattern: String,
    /// The path of the folder / file to read
    pub path: Option<PathBuf>,

    /// File or directory to exclude
    #[clap(long, short, alias = "exclude, ignore, skip", value_delimiter = ',')]
    pub omit: Option<Vec<PathBuf>>,

    /// Display informations about the file being read, the files that cannot be read ...
    #[clap(long, short)]
    pub verbose: bool,

    /// Match case when searching for content
    #[clap(long, short)]
    pub ignore_case: bool,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_args_are_parsed() {
        let cli = Cli {
            pattern: "old".to_string(),
            path: Some(PathBuf::from("path")),
            omit: Some(vec![PathBuf::from("omit")]),
            verbose: false,
            ignore_case: false,
        };

        assert_eq!(cli.pattern, "old");
        assert_eq!(cli.path, Some(PathBuf::from("path")));
        assert_eq!(cli.omit, Some(vec![PathBuf::from("omit")]));
        assert_eq!(cli.verbose, false);
    }
}
