# tree_gen

`tree_gen` is a command-line tool designed to generate visual representations of directory structures in both ASCII and JSON formats. This tool provides an easy way to visualize and explore file hierarchies directly in the terminal. It also supports comparing two directories, filtering files, and customizing the output to suit various needs.

## Features

- Generate tree views of directory structures in ASCII or JSON formats.
- Compare two directories to show differences.
- Customize output depth and filter files by extension, size, or date.
- Write output to a file or display in the terminal.
- Show file sizes, icons, and even preview lines of file content.

## Installation

### Install Globally

To install `tree_gen` globally, you can use `cargo`. Just run `cargo install tree_gen` in your favorite terminal.

```bash
cargo install tree_gen
```

### Install local

To use `tree_gen`, clone the repository and build the binary using Rust's package manager, Cargo:

```bash
cargo build --release
```

Then you can run `tree_gen` from the `target/release` directory or add it to your PATH for easy access.

## Usage

```bash
tree_gen <PATH> [OPTIONS]
```

### Positional Argument

- `PATH`: The path of the directory you wish to display or compare.

### Options

- `--compare <COMPARE_PATH>`: Compare the specified directory (`<PATH>`) with another directory at `<COMPARE_PATH>`.
- `-d, --depth <DEPTH>`: Set the maximum depth of the directory tree (default is 10).
- `-e, --extension <EXT>`: Filter output to show only files with the specified file extension.
- `-o, --output <FILE>`: Write the output to a specified file instead of printing to the terminal.
- `--branch-style <STYLE>`: Set the branch style for the tree structure. Options: `ascii`, `unicode` (default: `unicode`).
- `--preview-lines <LINES>`: Display a limited number of preview lines for each file.
- `--date-filter <DATE_FILTER>`: Filter files by date. Format: `<before|after|between> <date1>[,<date2>]`.
- `--size-min <SIZE_MIN>`: Set the minimum file size (in bytes) for filtering.
- `--size-max <SIZE_MAX>`: Set the maximum file size (in bytes) for filtering.
- `--include <INCLUDE>`: Include files matching a specific pattern (wildcard or regex).
- `--exclude <EXCLUDE>`: Exclude files matching a specific pattern (wildcard or regex).
- `-i, --ignore-hidden`: Ignore hidden files and folders.
- `--icons`: Display file icons alongside file names.
- `-s, --show-sizes`: Show file sizes next to file names.
- `-j, --json`: Output the directory structure in JSON format.

### Examples

#### Basic Usage

To display the directory structure of the current directory:

```bash
tree_gen .
```

#### Set Depth

To display a directory tree structure with a maximum depth of 2 levels:

```bash
tree_gen . --depth 2
```

#### Filter by File Extension

To show only `.rs` files in the directory structure:

```bash
tree_gen . --extension rs
```

#### Save Output to File

To save the directory structure to a file named `output.txt`:

```bash
tree_gen . --output output.txt
```

#### Branch Style

To set the branch style to ASCII instead of the default Unicode:

```bash
tree_gen . --branch-style ascii
```

#### Display JSON Format

To output the directory structure in JSON format:

```bash
tree_gen . --json
```

#### Compare Two Directories

To compare two directories (`lib` and `lib_copy`) and show the differences:

```bash
tree_gen lib --compare lib_copy
```

#### Filter by File Size

To show only files larger than 1 KB and smaller than 1 MB:

```bash
tree_gen . --size-min 1024 --size-max 1048576
```

#### Filter by Date

To show files created after January 1, 2023:

```bash
tree_gen . --date-filter "after 2023-01-01"
```

#### Exclude or Include Files by Pattern

To exclude files matching the pattern `*.tmp`:

```bash
tree_gen . --exclude "*.tmp"
```

Or to include only files matching a specific pattern:

```bash
tree_gen . --include "*.rs"
```

#### Additional Options

- `-i, --ignore-hidden`: Ignore hidden files and folders.
- `--icons`: Display file icons next to file names.
- `-s, --show-sizes`: Display file sizes alongside file names.
- `--preview-lines <LINES>`: Limit the preview lines for each file to `<LINES>` lines.

## TODO List for `tree_gen` for futures improvements

- [ ] Support a configuration file (e.g., `.treegenrc`) for default values for flags like `depth`, `ignore_hidden`, `branch_style`, etc.
- [ ] Provide an interactive mode (`--interactive`) to allow directory expansion and collapse in the terminal.
- [ ] Optimize performance for large directories, possibly with multi-threading.
- [ ] Refactor output formats (ASCII, JSON, XML) to use traits, simplifying future additions.
- [ ] Create release binaries for multiple platforms (Windows, macOS, Linux)

## Contributing

Contributions are welcome! If you’d like to improve this tool or add new features, please feel free to submit a pull request. (TODO lists for futures improvement already mention above. You can implement TODO lists features or your can create your own new features.).

## License

`tree_gen` is open-source software licensed under the [MIT License](/LICENSE).
