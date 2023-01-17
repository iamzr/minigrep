# Minigrep

This is a clone of the common command line utility `grep` written in rust.

## Usage

You can run the program easily using the following command:

```sh
cargo run -- [query] [path_to_file]
```

where:

- `[query]` is the query string you want to search for
- `[path_to_file]` is the path to the file you want to search

If you want to output the results to a txt file use:

```
cargo run -- [query] [path_to_file] > [output.txt]
```

where you can replace `[output.txt]` to your desired output path.

You can enable logging using:

```
RUST_LOG=[log_level] cargo run -- [query] [path_to_file]
```

(Note: this command is for Linux)

where `[log_level]` is the desired log-level.
