# description
MMV - mass move project. It takes path to input directory and moves files that match the given pattern to output direcory. Ouput files are named according to the pattern.

# run
You can run it with:
```
$ ./mmv 'path/to/some_*_filename.*' 'path2/to/changed_#1_filename.#2'
```

# examples
```
$ ./mmv 'path/to/some_*_filename.*' 'path2/to/changed_#1_filename.#2'
path/to/some_A_filename.bin -> path2/to/changed_A_filename.bin
path/to/some_A_filename.jpg -> path2/to/changed_A_filename.jpg
path/to/some_B_filename.bin -> path2/to/changed_B_filename.bin
path/to/some_B_filename.jpg -> path2/to/changed_B_filename.jpg
```

# regex

In pattern only `*` character is considered to be special. Other special characters like `?` `.` and other are NOT supported. 

# flags

You can run the program with two flags

`-h` `--help` - prints help information

`-f` `--force` - overrides existing files in output

# external crates

There are multiple crates that MMV uses, you can find the list of them in the `Cargo.toml` file. The primary crates are [**clap**](https://docs.rs/clap/latest/clap/) and [**serde**](https://docs.rs/serde/latest/serde/)

**clap** is used to conveniently work with command line arguments.

**serde**, in particular **serde_yaml** is used for **config.yaml** parsing.

# tests

MMV has both unit and integration testing. 

Integration tests are located in `./tests` directory

Unit tests are presented in the source files in special mod `test`

You can run tests by
```
$ cargo test
```


# implementation

Quick implementation overview for those who it may concern.

First of all arguments are parsed in `main`. If successfully they are passed to `run`.

There input and output paths are split into (path, file_name) because the pattern is only applied to filenames.

Then input filenames are firstly matched. The `MathcedFilename` struct consist of original filename and pattern fragments. For example string for pattern `*_aba_*` and name `a_aba_caba`, the `MatchedFilename` wille be (orignal: `a_aba_caba`, fragments: `["a", "caba"]`).

Then `MatchedFilenames` are passed to `modify_filenames` where according to pattern `MatchedFilename` is converted to `ModifiedFilename`. `ModifiedFilename` is basically original filename and new filename.

The `ModifiedFilenames` are then passed to `move_files` function that moves filename to new filename.