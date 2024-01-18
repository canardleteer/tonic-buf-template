# `{{project-name}}` Service Repository

## `cargo generate` notes

- You will need `protoc` installed to build.

Make sure this repo is in good shape after generation, if it isn't please
submit an issue or PR for [this template]:

```shell
cargo check
cargo fmt --check
cargo clippy
buf lint proto
buf format -d proto

# clean up before building an image
cargo clean
```

## Getting Started

It's worth checking out the `tonic` [upstream documentation on configuring your IDE](https://github.com/hyperium/tonic#getting-started).

## Build the Docker image

```shell
docker build -t {{project-name}}:latest -f ./docker/Dockerfile.{{project-name}} .
```

## Run the Docker image

```shell
docker network create {{project-name}}-network
docker run --rm -it -d \
    --name {{project-name}}-server \
    --net {{project-name}}-network \
    -p 50051:50051 \
    {{project-name}}:latest

docker run --rm -it \
    --net {{project-name}}-network \
    -e USE_CLIENT_BINARY=true \
    {{project-name}}:latest -a {{project-name}}-server

# cleanup
docker rm -f {{project-name}}-server
docker network remove {{project-name}}-network
```
