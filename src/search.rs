use anyhow::{Context, Result};
use ignore::Walk;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

/// Function to get the matches in a file
/// We enumerate over the lines of the file
/// and filter the lines that contain the pattern
/// We return a vector of tuples containing the line number
/// and the content of the line
fn find_matches(file: BufReader<File>, pattern: &str) -> Vec<(usize, String)> {
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

pub fn search_in_file(path: &PathBuf, pattern: &str) -> Result<()> {
    let file = File::open(&path).with_context(|| format!("could not read file `{:?}`", &path))?;

    let reader = BufReader::new(file);

    let matches = find_matches(reader, &pattern);

    if !matches.is_empty() {
        println!("\n{:?}", path);
        for (line_number, content) in matches {
            println!("Line {}: {}", line_number, content);
        }
    }

    Ok(())
}

pub fn walk_folders(path: &PathBuf, omit: &Option<Vec<PathBuf>>, pattern: &str) -> Result<()> {
    for entry in Walk::new(&path) {
        match entry {
            Ok(data) => {
                let entry_path = data.path().to_path_buf();

                // Check if the path should be omitted
                if let Some(to_omit) = omit {
                    if to_omit
                        .iter()
                        .any(|omit_path| entry_path.starts_with(omit_path))
                    {
                        continue;
                    }
                }

                if entry_path.is_file() {
                    search_in_file(&entry_path, pattern)?;
                }
            }
            Err(error) => eprintln!("Failed to read the following path: {:?}", error),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_search_in_file() {
        let pattern = "old".to_string();
        let path = PathBuf::from("tests/assets/classic.txt");

        assert!(search_in_file(&path, &pattern).is_ok());
    }

    #[test]
    fn test_walk_folders() {
        let pattern = "old".to_string();
        let path = PathBuf::from("tests/assets");

        assert!(walk_folders(&path, &None, &pattern).is_ok());
    }

    #[test]
    fn test_find_matches_classic() {
        let file = BufReader::new(File::open("tests/assets/classic.txt").unwrap());
        let matches = find_matches(file, "more");

        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn test_find_matches_empty() {
        let file = BufReader::new(File::open("tests/assets/empty.txt").unwrap());
        let matches = find_matches(file, "more");

        assert!(matches.is_empty());
    }

    #[test]
    fn test_find_matches_multiple() {
        let file = BufReader::new(File::open("tests/assets/classic.txt").unwrap());
        let matches = find_matches(file, "world");

        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_find_matches_no_match() {
        let file = BufReader::new(File::open("tests/assets/classic.txt").unwrap());
        let matches = find_matches(file, "foo");

        assert!(matches.is_empty());
    }
}
