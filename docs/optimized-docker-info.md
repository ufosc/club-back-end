# Optimized Docker Info

The [optimized.Dockerfile](optimized.Dockerfile) is designed to cache the dependencies before building the project itself. Because Docker caches steps, after the first build, it won't take the time to rebuild dependencies, just the app itself.

Another step could be to use Docker's [multi-stage builds](https://docs.docker.com/develop/develop-images/multistage-build/) with this [example](https://whitfin.io/speeding-up-rust-docker-builds/) to reduce the size of the Docker image.

Unfortunately, how this Dockerfile works (pre-compiling) it does not work well with the existing docker-compose file. If the docker-compose file could use a specific stage of the multistage build (specifically after the dependency building part), they might work well together.
