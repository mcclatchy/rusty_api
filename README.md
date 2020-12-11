# Rusty API
A graphql based api that responds to cgi requests and uses sqlite3 for storage.

This code is intended to be used as boilerplate for projects that require the use of a database. 
The static media server supports the execution of CGI, which is suitable for relatively low volumes of traffic.

A use case might be as a non-ephemeral storage solution for newsroom tools. 
Or, perhaps for writing data to the server and then having the results of that data copied 
to another location on the server that can be accessible by readers.

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

## Settings
Rusty API requires some settings to be enabled for it to be used.
The settings file is named `settings.toml` and is placed alongside the binary.

Here is an example `settings.toml` file:
```toml
api_key = "189rjfadoisfj8923fjio"
database_url = "rusty_api.sqlite"
debug = false
```

`api_key` is reserved for future use. This is a random string of chracters that 
can be used for allowing incoming requests from outside sources. 

`database_url` is required. This is the name of the database and is
expected to be placed alongside the binary.

`debug` is reserved for future use and is intended to be used for switching 
between a debug and production environment.

## Examples
Although this program has been working very well for local development, there have been
some occassional unexpected / odd behaviors when deployed to the static media server. 

Here are three commands to test the current GraphQL functionality. 
Some substitution will likely be necessary to reference the correct record or server location. 

### Return all Posts
`curl -X POST -H "Content-Type: application/json" --data '{ allPosts{id title body published} }' http://localhost:8000/cgi-bin/test.cgi -v`

### Return a single Post by ID
`curl -X POST -H "Content-Type: application/json" --data '{ getPost(postId: "659cc6d7-b74d-4fff-bfae-c06aedb905f1"){id title body} }' http://localhost:8000/cgi-bin/test.cgi -v`

### Create a new Post
`curl -X POST -H "Content-Type: application/json" --data 'mutation { createPost(postTitle: "Today was a good day" postBody: "I didn't even have to use my AK" postPublished: true){id title body published} }' http://localhost:8000/cgi-bin/test.cgi -v`

## Testing
If Python is available on your system, then it's very easy to setup a local server for testing. 
This shell script will compile the code, copy it into a local path named `cgi-bin`, and then run a web server.
The `curl` statements above can be used for interacting with the server to test the code.

```shell
#!/usr/bin/env bash
set -euxo pipefail
clear
echo Starting Build
cargo build --release
echo Copying build to ./cgi-bin
cp target/release/rusty_api ./cgi-bin/test.cgi
echo Starting server
python3 -m http.server --bind localhost --cgi 8000
```
After you copy & paste the above code into a shell script file, such as `test.sh`, place the script alongside the binary.
Make sure to run `chmod +x ./test.sh` so that your system will treat it as an executable. 

To run the script, type `./test.sh` and open point your web browser to `localhost:8000/cgi-bin/test.cgi`.

## Security
Because CGI runs in a seperate thread created by the web server and as a standalone executable
on the server, it is susceptible to denial of service attacks. 

Therefore, it is strongly recommended that the location of the binary is obfuscated, as well 
as placed behind a username and password defined in a local `.htaccess` file.

The web server communicates to the CGI executable through a combination of environment variables
and STDIN, providing some additional security for the executable. 

Additionally, because this is written in Rust, there is a reduced threat from common attacks, though 
this code has not been thoroughly vetted, meaning percautions and best practices should always be adhered to. 