# Club Backend

[![Build Status](https://travis-ci.com/ufosc/club-backend.svg?branch=dev)](https://travis-ci.com/ufosc/club-backend)

A RESTful API for club information (events, projects, sign-ins, etc) that is used for the club website, bots, and any future projects related to administration.

A list of use cases can be found [here](https://github.com/ufosc/club-backend/issues/2).

## Getting Started

We will use Docker to develop and deploy the code.

### Installing

Install [Docker](https://docs.docker.com/install/linux/docker-ce/ubuntu/) and [Docker compose](https://docs.docker.com/compose/install/).

### Running

To run the app

```bash
docker-compose up
```

> Note: The first time, you might have to add the `--build` flag to the end of the previous command

And go to [localhost:3001](http://localhost:3001/).

To run updated code, you can press "Ctrl-C" in the terminal or type

```bash
docker-compose down
```

### Testing

To run the unit test, you'll need to get into the Docker container

```bash
docker-compose exec backend bash
```

then run tests single threaded (needed to make sure to keep the database in the right state)

```bash
cargo test -- --test-threads=1
```

<!-- ## Deployment

**Additional steps to deploy and run the project** -->

## Built With

- [Rust](https://www.rust-lang.org/en-US/) - a (wonderful) systems programming language
- [Rocket Framework](https://rocket.rs/) - a web framework for Rust
- [PostgreSQL](https://www.postgresql.org/) - an open source relational database
- [Diesel](http://diesel.rs/) - An extensible ORM and Query Builder for Rust (to interact with PostgreSQL)

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for how to work on the project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
