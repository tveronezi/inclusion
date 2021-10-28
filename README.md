# inclusion

## Dependencies

* Check your cargo version with `cargo version`. The version at the writing of this document was
  `cargo 1.56.0 (4ed5d137b 2021-10-04)`.
* On ubuntu, you install `sudo apt-get install libpq-dev`.
* Install `diesel-cli` with `cargo install diesel_cli --no-default-features --features "postgres"`.
* Install `cargo-watch` with `cargo install cargo-watch`
* Check your docker version. `Docker version 20.10.10, build b485636`
* Check your docker-compose version. `docker-compose version 1.27.4, build 40524192`
* Install `trunk` and `wasm-bindgen-cli` with `cargo install trunk wasm-bindgen-cli`
* install new rust target with `rustup target add wasm32-unknown-unknown`

## How to test it?

```shell
cargo test
```

## How to read the API documentation?

```shell
cargo doc --open
```

## How to use it?

```shell
docker-compose up -d
cargo run --bin inclusion-service
```

The API will be available under [http://127.0.0.1:4000/](http://127.0.0.1:4000/).

You can change the variables defined in via environment variables. For example:

```shell
BIND=127.0.0.1:8888 cargo run --bin inclusion-service
```

or

```shell
ARTICLES_DB_URL=postgres://rusty:rusty@localhost:5432/articlesdb cargo run --bin inclusion-service
```
