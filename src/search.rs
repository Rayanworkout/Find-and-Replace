use crate::utils::PathExt;
use anyhow::{Context, Result};
use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use crate::args::Cli;

/// Function to get the matches in a file
/// We enumerate over the lines of the file
/// and filter the lines that contain the pattern
/// We return a vector of tuples containing the line number
/// and the content of the line
fn get_matches(file: BufReader<File>, pattern: &str) -> Vec<(usize, String)> {
    file.lines()
        .enumerate() // Add line numbers starting from 0
        .filter_map(|(line_number, content)| {
            match content {
                Ok(content) if content.contains(pattern) => Some((line_number + 1, content)), // Line numbers start from 1
                _ => None,
            }
        })
        .collect()
}

pub fn search_in_file(args: &Cli) -> Result<()> {
    let file =
        File::open(&args.path).with_context(|| format!("could not read file `{:?}`", args.path))?;

    let reader = BufReader::new(file);

    let matches = get_matches(reader, &args.old_pattern);

    if !matches.is_empty() {
        println!("\n{:?}", &args.path);
        for (line_number, content) in matches {
            println!("\nLine {}: {}", line_number, content);
        }
    }

    Ok(())
}

pub fn search_in_folder(args: &Cli) -> Result<()> {
    // Here, we collect all the paths in the folder
    // with a Ok() result
    let mut folder: Vec<PathBuf> = read_dir(&args.path)
        .with_context(|| format!("Failed to read the following folder: {:?}", &args.path))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();

    // Then we keep only those who aren't omitted
    if let Some(omit) = &args.omit {
        folder.retain(|path| !path.should_omit(omit));
    }

    for path in folder {
        let cli = Cli {
            path: path.clone(),
            old_pattern: args.old_pattern.clone(),
            omit: args.omit.clone(),
            verbose: args.verbose,
        };

        match path.is_directory() {
            Some(result) => match result {
                true => search_in_folder(&cli)?,
                false => search_in_file(&cli)?,
            },
            None => eprintln!("Failed to read the following path: {:?}", path),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_search_in_file() {
        let args = Cli {
            old_pattern: "old".to_string(),
            path: PathBuf::from("tests/assets/classic.txt"),
            omit: None,
            verbose: false,
        };

        assert!(search_in_file(&args).is_ok());
    }

    #[test]
    fn test_search_in_folder() {
        let args = Cli {
            old_pattern: "old".to_string(),
            path: PathBuf::from("tests/assets"),
            omit: None,
            verbose: false,
        };

        assert!(search_in_folder(&args).is_ok());
    }

    #[test]
    fn test_get_matches_classic() {
        let file = BufReader::new(File::open("tests/assets/classic.txt").unwrap());
        let matches = get_matches(file, "more");

        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn test_get_matches_empty() {
        let file = BufReader::new(File::open("tests/assets/empty.txt").unwrap());
        let matches = get_matches(file, "more");

        assert!(matches.is_empty());
    }

    #[test]
    fn test_get_matches_multiple() {
        let file = BufReader::new(File::open("tests/assets/classic.txt").unwrap());
        let matches = get_matches(file, "world");

        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_get_matches_no_match() {
        let file = BufReader::new(File::open("tests/assets/classic.txt").unwrap());
        let matches = get_matches(file, "foo");

        assert!(matches.is_empty());
    }
}
