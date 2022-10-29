# moonstone

Your #1 way to test algorithm problems on Windows, macOS and Linux!

### Table of Contents

1. [Usage](#usage)
2. [Installation](#installation)
3. [Template File Structure](#file-structure)
4. [License](#license)

## Usage

```
$ mst new <name> - Creates a new project with the specified name
$ mst init - Initializes a new project in the current working directory
$ mst generate <name> <amount> [-t <time_limit (in ms)> (default: 5000)] - Generates a test package with a specified number of tests and a time limit
$ mst test <name> - Tests the main file with the specified test package
$ mst reset-cache - Resets the template cache
```

Config files are located in `$HOME_DIR/.mst/` Please **do not** edit the cache
files. Those are for internal use only.

## Installation

In order to install moonstone, you have to build it from scratch. Don't worry,
it's an incredibly easy process.

1. Install rustup from [rustup.rs](https://rustup.rs/)
2. Install gcc
3. Clone the repository
   > `git clone https://github.com/peonii/moonstone.git`
4. Run `cargo build --release`
5. Copy `mst` from `target/release/` to a folder with your binaries

## File Structure

```
📄 main.alg - Main algorithm
📄 brute.alg - Brute-force algorithm (used for generating output)
📄 gen.alg - Input generator
```

*Why do I have to use .alg files?*
This is for comfort, to not have to paste the same lines in every single program you write.
Why not use default files instead? Because some programs may require something extra.
This also cleans the code up and allows for nicer, more readable algorithms.

*Can I disable the replacer?*
You will be able to disable it in the next update.

## License

This project is licensed under the MIT License.

> Made with ❤️ by peony
