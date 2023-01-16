# Rust Playground

### Documentation

1. Install [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. [Crates](https://crates.io/)

```bash
# dependencies to be added

$ cargo add rocket
$ cargo add rocket_codegen
$ cargo add serde
$ cargo add serde_json
$ cargo add serde_derive
$ cargo add rocket_contrib
$ cargo add diesel
$ cargo add dotenvy
```

### Setup Diesel
```bash
$ echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
$ echo DATABASE_URL=mysql://user:pass@localhost/heroes > .env

# cargo diesel_cli related
$ sudo apt update
$ sudo apt-get install libpq5=13.4-1
$ sudo apt-get install libpq-dev
$ sudo apt install libmysqlclient-dev
$ sudo apt-get install libsqlite3-dev
$ cargo install diesel_cli
$ diesel setup
$ diesel migration generate heroes
$ diesel migration run
$ diesel print-schema > src/schema.rs

# do this to test both up and down sql
$ diesel migration redo

$ cargo run
$ cargo watch -x run
```