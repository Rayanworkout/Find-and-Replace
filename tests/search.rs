#[cfg(test)]
mod tests {

    use anyhow::Result;
    use assert_cmd::Command;
    use std::str;

    #[test]
    fn test_basic_search_empty() -> Result<()> {
        let mut cmd = Command::cargo_bin("fnr")?;
        let output = cmd
            .arg("nothing")
            .arg("new")
            .arg("tests/assets/")
            .output()
            .expect("Failed to execute command");

        let stdout = str::from_utf8(&output.stdout)?;
        let stderr = str::from_utf8(&output.stderr)?;

        assert!(output.status.success());
        assert_eq!(stdout, "\nNo match found.\n");
        assert_eq!(stderr, "");

        Ok(())
    }

    #[test]
    fn test_basic_search() -> Result<()> {
        let mut cmd = Command::cargo_bin("fnr")?;
        let output = cmd
            .arg("old")
            .arg("new")
            .arg("tests/assets/")
            .output()
            .expect("Failed to execute command");

        let stdout = str::from_utf8(&output.stdout)?;
        let stderr = str::from_utf8(&output.stderr)?;

        assert!(output.status.success());
        assert_eq!(
            stdout,
            "\ntests/assets/classic.txt\n-- old\n++ new\n\n1 match found.\nRe-run the command with --write to write changes to disk.\n");
        assert_eq!(stderr, "");

        Ok(())
    }

    #[test]
    fn test_basic_search_two_matches_same_file() -> Result<()> {
        let mut cmd = Command::cargo_bin("fnr")?;
        let output = cmd
            .arg("here")
            .arg("new")
            .arg("tests/assets/")
            .output()
            .expect("Failed to execute command");

        let stdout = str::from_utf8(&output.stdout)?;
        let stderr = str::from_utf8(&output.stderr)?;

        assert!(output.status.success());
        assert_eq!(
            stdout,
            "\ntests/assets/classic.txt\n-- I appear here\n++ I appear new\n\ntests/assets/classic.txt\n-- and here\n++ and new\n\n2 matches found.\nRe-run the command with --write to write changes to disk.\n");
        assert_eq!(stderr, "");

        Ok(())
    }

    #[test]
    fn test_basic_search_two_matches_different_files() -> Result<()> {
        let mut cmd = Command::cargo_bin("fnr")?;
        let output = cmd
            .arg("world")
            .arg("new")
            .arg("tests/assets/")
            .output()
            .expect("Failed to execute command");

        let stdout = str::from_utf8(&output.stdout)?;
        let stderr = str::from_utf8(&output.stderr)?;

        assert!(output.status.success());
        assert_eq!(
            stdout,
            "\ntests/assets/classic.txt\n-- hello world\n++ hello new\n\ntests/assets/some_python.py\n-- print(\"hello world\")\n++ print(\"hello new\")\n\n2 matches found.\nRe-run the command with --write to write changes to disk.\n");
        assert_eq!(stderr, "");

        Ok(())
    }

    // Starting using args
    #[test]
    fn test_basic_search_three_matches_with_hidden_file() -> Result<()> {
        let mut cmd = Command::cargo_bin("fnr")?;
        let output = cmd
            .arg("world")
            .arg("new")
            .arg("tests/assets/")
            // Adding the --hidden flag
            .arg("--hidden")
            .output()
            .expect("Failed to execute command");

        let stdout = str::from_utf8(&output.stdout)?;
        let stderr = str::from_utf8(&output.stderr)?;

        assert!(output.status.success());

        assert_eq!(
            stdout,
            "\ntests/assets/classic.txt\n-- hello world\n++ hello new\n\ntests/assets/some_python.py\n-- print(\"hello world\")\n++ print(\"hello new\")\n\ntests/assets/.hidden\n-- hello world\n++ hello new\n\n3 matches found.\nRe-run the command with --write to write changes to disk.\n");
        assert_eq!(stderr, "");

        Ok(())
    }

    #[test]
    fn test_basic_search_three_matches_with_hidden_file_verbose() -> Result<()> {
        let mut cmd = Command::cargo_bin("fnr")?;
        let output = cmd
            .arg("world")
            .arg("new")
            .arg("tests/assets/")
            // Adding the --hidden flag
            .arg("--hidden")
            // Adding the --verbose flag
            .arg("--verbose")
            .output()
            .expect("Failed to execute command");

        let stdout = str::from_utf8(&output.stdout)?;
        let stderr = str::from_utf8(&output.stderr)?;

        assert!(output.status.success());

        assert_eq!(
            stdout,
            "\ntests/assets/classic.txt\n-- hello world\n++ hello new\n\ntests/assets/some_python.py\n-- print(\"hello world\")\n++ print(\"hello new\")\n\ntests/assets/.hidden\n-- hello world\n++ hello new\n\n3 matches found.\nRe-run the command with --write to write changes to disk.\n");
        assert_eq!(stderr, "");

        Ok(())
    }

    #[test]
    fn test_basic_search_case_insensitive() -> Result<()> {
        let mut cmd = Command::cargo_bin("fnr")?;
        let output = cmd
            .arg("WORLD")
            .arg("new")
            .arg("tests/assets/")
            // Adding the --case-insensitive flag
            .arg("--ignore-case")
            .output()
            .expect("Failed to execute command");

        let stdout = str::from_utf8(&output.stdout)?;
        let stderr = str::from_utf8(&output.stderr)?;

        assert!(output.status.success());

        assert_eq!(
            stdout,
            "\ntests/assets/classic.txt\n-- hello world\n++ hello new\n\ntests/assets/some_python.py\n-- print(\"hello world\")\n++ print(\"hello new\")\n\ntests/assets/some_python.py\n-- print(\"hello world\")\n++ print(\"hello new\")\n\n3 matches found.\nRe-run the command with --write to write changes to disk.\n");
        assert_eq!(stderr, "");

        Ok(())
    }
}
