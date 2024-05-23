## Find and Replace

A simple Rust CLI tool to efficiently search a text pattern inside your folders and replace it. (currently support search only).

### Installation (Linux)

_You need to have `Rust` and `Cargo` installed on your machine to run this tool. Official installation steps [here](https://www.rust-lang.org/tools/install)_

```bash
git clone https://github.com/Rayanworkout/Find-and-Replace.git
cd Find-and-Replace
cargo build --release

sudo mv target/release/fnr /usr/local/bin

```

You can now call the tool from anywhere in your terminal.

### Usage

```bash
fnr <pattern> [path] [options]
```

Binaries and non UTF-8 files are skipped.

### Some examples

Find a pattern 'old' in files of the current folder.
If no path is provided, the tool will search in the current folder.
    
```bash
$ fnr old
```

For any lookup, you can activate verbose mode.

```bash
$ fnr old --verbose // or -v
```

You can also perform a case-insensitive search.

```bash
$ fnr old --ignore-case // or -i
```

Find a pattern 'old' in files of the current folder, excluding the 'Desktop' folder.
**Note that the path should be an absolute path, otherwise it won't be taken into account.**

```bash
$ fnr old --omit ~/Desktop // or -o ~/Desktop
```

You can also omit multiple folders.

```bash
$ fnr old --omit ~/Desktop/ ~/Desktop/foo
```

Including hidden files in your search.

```bash
$ fnr old --hidden --omit ~/Desktop/ ~/Desktop/foo
```

Only search for files with a specific extension (use glob patterns) inside the home directory.

```bash
$ fnr old ~ --type *rs // or -t *rs
```

You can also check for a specific pattern using one or 2 wildcards.
Here we search only in files with the name ending with "some.txt".
```bash
fnr pattern ~/Desktop/ -t "*some.txt"
```

Wildcards can be used in many ways. Here we search within files starting with "d" and ending with "e.txt".
```bash
fnr pattern ~/Desktop/ -t "d*e.txt"
```

Ignore files with a specific extension.

```bash
$ fnr old --type-not *rs // or -T *rs
```

You can also search / ignore multiple file types or patterns.
Here, we search for files with .rs and .toml extension, but ignore .txt and .md files.

```bash
$ fnr old --type *rs *toml --type-not *txt *md
```

As an example, the equivalent to the following command
```bash
find ~/Desktop/ -type f -name "*txt" -exec cat {} \; | grep hello
```
would be
```bash
$ fnr hello ~/Desktop/ -t *txt
```

At any moment, feel free to hit 
```bash
$ fnr --help
```
to get a list of all available options.