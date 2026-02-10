# Find and Replace

Fast recursive search and replace for files, built with Rust.

## Installation (Linux)

_You need `Rust` and `Cargo` installed. Official installation steps are available [here](https://www.rust-lang.org/tools/install)._

```bash
git clone https://github.com/Rayanworkout/Find-and-Replace.git
cd Find-and-Replace
cargo build --release
sudo mv target/release/fnr /usr/local/bin
```

You can now call `fnr` from anywhere in your terminal.

## Usage

```bash
fnr [OPTIONS] <PATTERN> <NEW_PATTERN> [PATH]

# or

fnr <PATTERN> <NEW_PATTERN> [PATH] [OPTIONS]
```

`[PATH]` is optional. If omitted, `fnr` searches from the current directory.

## Quick Notes

- `fnr` always expects both `<PATTERN>` and `<NEW_PATTERN>`.
- Use `_` as a placeholder for `<NEW_PATTERN>` when you only want lookup output.
- `--write` applies replacements to files on disk.
- Without `--write`, `fnr` only previews matches and suggested replacements.
- Binaries and non-UTF-8 files are skipped.

## Examples

Find a pattern `hello` in files of the current folder without writing changes:

```bash
fnr hello new
```

Find `hello` recursively and replace it with `new` on disk:

```bash
fnr hello new --write
```

Enable verbose mode for lookup or replacement:

```bash
fnr hello _ --verbose # or -v
```

Case-insensitive matching:

```bash
fnr hello new --ignore-case  # or -i
```

Exclude `Desktop` and replace `hello` with `new`:

```bash
fnr hello new --omit ~/Desktop  # or -o ~/Desktop
```

Omit multiple folders:

```bash
fnr hello new --omit ~/Desktop/ ~/Another/
```

Include hidden files in the search:

```bash
fnr hello new --hidden --omit ~/Desktop/ ~/Desktop/foo
```

Search only files matching a glob pattern inside home:

```bash
fnr hello _ ~ --type *rs  # or -t *rs
```

Search using wildcard patterns. Match files ending with `some.txt`:

```bash
fnr old_pattern new_pattern ~/Desktop/ -t "*some.txt"
```

Match files starting with `d` and ending with `e.txt`:

```bash
fnr old_pattern new_pattern ~/Desktop/ -t "d*e.txt"
```

Ignore files matching a specific extension/pattern:

```bash
fnr hello new --type-not *rs  # or -T *rs
```

Search only specific file types/patterns:

```bash
fnr old new --type *rs *toml
```

Equivalent of this `find` + `grep` flow:

```bash
find ~/Desktop/ -type f -name "*txt" -exec cat {} \; | grep hello
```

with `fnr`:

```bash
fnr hello _ ~/Desktop/ -t *txt
```

Show help at any time:

```bash
fnr --help
```

## Options

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
