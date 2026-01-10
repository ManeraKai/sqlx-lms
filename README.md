# Library Management System

This repository is not very useful because, it's a university course project :)

I wrote the backend in Rust because, I wanted to try [sqlx](https://github.com/launchbadge/sqlx).

## Development

Install [Node.js](https://nodejs.org), [Rust](https://www.rust-lang.org/)

### Initialize Database
```sh
cargo install sqlx-cli
sqlx database create
sqlx migrate run
```

### Run backend
```sh
cargo run
```

### Run front-end
```sh
cd src/frontend/
npm install
npm run dev
```
