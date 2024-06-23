# Find and Replace

Search & Replace some patterns within your files with the speed and robustness of Rust.

## Installation (Linux)

_You need to have `Rust` and `Cargo` installed on your machine to run this tool. Official installation steps [here.](https://www.rust-lang.org/tools/install)_

```bash
git clone https://github.com/Rayanworkout/Find-and-Replace.git
cd Find-and-Replace
cargo build --release

sudo mv target/release/fnr /usr/local/bin

```

You can now call the binary from anywhere in your terminal.


## Some examples

Find a pattern 'hello' in files of the current folder without replacing it.
    
```bash
fnr hello new
```

Find a pattern 'hello' in files of the current folder (recursively) and replace it with 'new'.

```bash
fnr hello new --write
```

For any lookup or replacement, you can activate verbose mode.

**Note the use of "_" to indicate that we only want to perform a lookup.**

```bash
fnr hello _ --verbose # or -v
```

You can also perform a case-insensitive search / replacement.

```bash
fnr hello new --ignore-case  # or -i
```


**Note that ignored path(s) should be absolute path(s), otherwise it won't be taken into account.**

Find a pattern 'hello' in files of the current folder, excluding the 'Desktop' folder and replace it with 'new'.

```bash
fnr hello new --omit ~/Desktop  # or -o ~/Desktop
```

You can also omit multiple folders.

```bash
fnr hello new --omit ~/Desktop/ ~/Another/
```

Including hidden files in your search.

```bash
fnr hello new --hidden --omit ~/Desktop/ ~/Desktop/foo
```

Only search for files with a specific extension (use glob patterns) inside the home directory.

```bash
fnr hello _ ~ --type *rs  # or -t *rs
```

You can also check for a specific pattern using one or 2 wildcards.
Here we search only in files with the name ending with "some.txt".
```bash
fnr old_pattern new_pattern ~/Desktop/ -t "*some.txt"
```

Wildcards can be used in many ways. Here we search within files starting with "d" and ending with "e.txt".
```bash
fnr old_pattern new_pattern ~/Desktop/ -t "d*e.txt"
```

Ignore files with a specific extension.

```bash
fnr hello new --type-not *rs  # or -T *rs
```

You can also search / ignore multiple file types or patterns.
Here, we allow only files with .rs and .toml extension.

```bash
fnr old new --type *rs *toml
```


As an example, the equivalent to the following `find` command
```bash
find ~/Desktop/ -type f -name "*txt" -exec cat {} \; | grep hello
```
would be
```bash
fnr hello _ ~/Desktop/ -t *txt
```

At any moment, feel free to hit 
```bash
fnr --help
```
to get a list of all available options.



## Usage

```bash
fnr [OPTIONS] <PATTERN> <NEW_PATTERN> [PATH]

# or if you prefer

fnr <PATTERN> <NEW_PATTERN> [PATH] [OPTIONS]

```

⚠️ Binaries and non UTF-8 files are skipped.

If no path is provided, the tool will search in the current folder.

All options have a short version, excluding `--write` and `--hidden` flags.


All options:

```bash
      --write
          Write changes to disk.
      --hidden
          Include hidden files in the search.
  -o, --omit [<OMIT>...]
          File or directory(ies) to exclude
  -v, --verbose
          Print additional information about files searched or errors.
  -i, --ignore-case
          Perform a case-insensitive search. Default is case-sensitive.
  -t, --type [<SELECTED_FILE_TYPES>...]
          Only search files matching <file_type> or glob pattern.
  -T, --type-not [<IGNORED_FILE_TYPES>...]
          Ignore files matching <file_type> or glob pattern.
```
