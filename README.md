# Club Back-End

[![Build Status](https://travis-ci.com/ufosc/club-backend.svg?branch=dev)](https://travis-ci.com/ufosc/club-backend)

A RESTful API for club information (events, projects, sign-ins, etc) that is used for the club website, bots, and any future projects related to administration.

A list of use cases can be found [here](https://github.com/ufosc/club-back-end/issues/2).

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

<!-- ## Deployment

**Additional steps to deploy and run the project** -->

## Built With

- [Rust](https://github.com/ufosc/club-resources/tree/master/rust) - a (wonderful) systems programming language
- [Rocket Framework](https://rocket.rs/) - a web framework for Rust
- [PostgreSQL](https://www.postgresql.org/) - an open source relational database
- [Diesel](http://diesel.rs/) - An extensible ORM and Query Builder for Rust (to interact with PostgreSQL)

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for how to work on the project and the [DESIGN.md](docs/DESIGN.md) file to understand more about the technologies we use and how it all fits together.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
