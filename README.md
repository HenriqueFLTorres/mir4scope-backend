# Mir4scope Backend
🚀 **blazingly fast** 🚀 🦀 backend for https://www.mir4scope.com

## .env
`DATABASE_URL`: **postgres** database url
 - `postgres://<username>:<password>@<netloc>:<port>/mir4scope` 

## How to run
```bash
# needed to get tradable items
cargo run --package dump_trade_items
# instead of running the command above you can copy the file 
# `dump_trade_items/list.json` to the root of the project

# Then finally run!!!
cargo run --release # go birrrrr
# or with arguments:
cargo run --release -- -d -l -i 1 -f 5
```

### Dependencies
- [Rust](https://rustup.rs/) >= 1.74.1
- [Docker](https://www.docker.com/) or a running [postgres](https://www.postgresql.org/) database
- For docker check our [docker-compose.yml](docker-compose.yml), basically just run `docker-compose up -d`
```
Backend for https://www.mir4scope.com

Usage: mir4scope-backend.exe [OPTIONS]

Options:
  -i, --initial-page <INITIAL_PAGE>  Initial page to collect NFT [default: 1]
  -f, --final-page <FINAL_PAGE>      Final page to collect NFT [default: 5]
  -d, --drop                         If the backend should drop the database or not. [Default: false]
  -l, --local                        Local Development [default: false]
  -h, --help                         Print help
  -V, --version                      Print version
```