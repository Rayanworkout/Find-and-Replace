#[derive(Debug)]
/// Settings applied for a DirectoryPatcher run
pub struct Settings {
    /// Control verbosity of console's output
    pub verbosity: bool,
    /// If true, ignore case in your search (default: false)
    pub ignore_case: bool,
    /// If true, search hidden files and directories (default: false)
    pub search_hidden: bool,
    /// Files and directories to omit from the results (default: None)
    pub omit_pattern: bool,
    /// List of file types to select (default: empty)
    pub selected_file_types: Vec<String>,
    /// List of file types to ignore (default: empty)
    pub ignored_file_types: Vec<String>,
}
