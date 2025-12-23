# Library Management System

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
