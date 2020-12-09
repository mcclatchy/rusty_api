# rusty_api
A graphql based api that responds to cgi requests and uses sqlite3 for storage

## Setup Tools, Database and Compile Binary
1. Make sure Rust is installed on your machine. If not, follow the instructions on [the Rust website](https://www.rust-lang.org/tools/install).
2. Setup the database using diesel. [Here is a nice walkthrough](http://diesel.rs/guides/getting-started/).
    1. Make sure diesel command line tools are installed: `cargo install diesel_cli`.
    2. Run `diesel setup` to generate the schema and folder structure. 
    3. Create database migration: `diesel migration generate rusty_api`.
    4. Migrate the database: `diesel migration run`.
3. To compile the binary, run `cargo build --release`.

After setting everything up and compiling the code, the binary will be located at `target/release/rusty_api`.

_Caution: The current query uses a hard coded index key, so it won't work unless you create a new record and swap the index key out before compiling._ 

### Compiling Issues
Although the static media server is running on CentOS 7, the web content server is running CentOS 6. 
Therefore, Rust needs to be compiled with specifications for CentOS 6. 

The appropriate compiling toolchain is `x86_64-unknown-linux-musl`. 
_This toolchain specification is used by a multitude of languages, such as C, so it's not unique to Rust._

An unfortunate situation with the CentOS 6 server is that some of the necessary libraries for compiling may not be installed. 
To get around that, it's often possible to compile on a different OS by targetting specific toolchains.

Another way to get around the problems of compiling for other OS is to _virtualize_ that OS, and what better way than to use Docker. :) 

If you have the patience, it's not difficult (but not trivial) to build a Docker container from scratch to replicate the specific build environment. 
However, in many cases you may have find a suitable Docker container already exists with what you need.

#### tl;dr
[This Github repo](https://github.com/emk/rust-musl-builder) contains a suitable Docker environment for compiling to CentOS 6. 
Run these two commands from within your code directory:
```commandline
alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder'
rust-musl-builder cargo build --release
```

