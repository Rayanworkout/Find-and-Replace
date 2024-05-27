#[cfg(test)]
mod tests {

    use assert_cmd::Command;

    // ARGS SUBMISSION TESTS

    #[test]
    fn test_no_arguments() {
        let mut cmd = Command::cargo_bin("fnr").unwrap();
        cmd.assert().failure();
    }

    #[test]
    fn test_omit() {
        let mut cmd = Command::cargo_bin("fnr").unwrap();
        cmd.args(&["old", ".", "--omit", "tests/"])
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
