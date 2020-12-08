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