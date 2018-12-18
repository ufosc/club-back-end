# Extra Docker Info

## Background

With some insight from this [Rust Web Starter project](https://github.com/ghotiphud/rust-web-starter) the Dockerfile and docker-compose were edited to connect to the database properly, to not re-install cargo packages each time docker-compose is run, to automatically re-compile code changes, and to use the diesel CLI.

## Docker Compose

If the Dockerfile or docker-compose.yml files are modified, it might be necessary to rebuild before running. To do so, run

```bash
docker-compose up --rebuild
```

Sometimes docker-compose leaves behind ports, volumes, etc. This can interfere with re-running docker-compose on this or other docker projects. To totally remove these artifacts, run

```bash
docker-compose down -v
```

## Additional Commands

The [diesel CLI](http://diesel.rs/) is designed to help create databases, change the databases properties (called migrations), and undo migrations if there is an issue (all while keeping the data safe).

So to run these commands first start the instance with

```bash
docker-compose up
```

and then in a new terminal jump into that containers shell by

```bash
docker-compose exec backend bash
```

From there all normal diesel or rust maintenance commands can be run. Please go to the official documentation for details, a list of commonly used commands are below.
- `cargo upgrade`
- `diesel setup`
- `diesel migration generate {name}`
- `diesel migration run`
- `diesel migration redo`
