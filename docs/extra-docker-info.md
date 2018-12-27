# Extra Docker Info

## Background

With some insight from this [Rust Web Starter project](https://github.com/ghotiphud/rust-web-starter) the Dockerfile and docker-compose were edited to connect to the database properly, to not re-install cargo packages each time docker-compose is run, to automatically re-compile code changes, and to use the Diesel CLI.

## Docker Compose

If the Dockerfile or docker-compose.yml files are modified, it might be necessary to rebuild before running. To do so, run

```bash
docker-compose up --rebuild
```

Sometimes docker-compose leaves behind ports, volumes, etc. This can interfere with re-running docker-compose on this or other docker projects. To totally remove these artifacts, run

```bash
docker-compose down -v
```

## Rust Tools

Rust has a code formatting tool called Rustfmt, which allows us to define a style and have it easily enforced. All you need to do is run `cargo fmt` (as described below) and all the source code in this project will be updated to match our style guidelines!

Clippy is a smart linter for Rust that can be run as an extra check on the code in the project. By default it checks all the dependencies and the source code, before only checking changes. It can be run by using `cargo clippy` (as described below).

## Additional Commands

The [Diesel CLI](http://diesel.rs/) is designed to help create databases, change the databases properties (called migrations), and undo migrations if there is an issue (all while keeping the data safe).

So to run these commands first start the instance with

```bash
docker-compose up
```

and then in a new terminal jump into that containers shell by

```bash
docker-compose exec backend bash
```

From there all normal Diesel or Rust maintenance commands can be run. Please go to the official documentation for details, a list of commonly used commands are below.
- `cargo fmt`
- `cargo test -- --test-threads=1`
- `cargo upgrade`
- `cargo clippy`
- `diesel setup`
- `diesel migration generate {name}`
- `diesel migration run`
- `diesel migration redo`
