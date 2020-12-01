# Hello Rust with Nix

## Rust Application:

A Rocket Application, exposing the following endpoints:

- "/" - for a Simple Hello World
- "/hostinfo" - for getting the host information the binary is running on, it gets the following information in JSON.
    - Hostname.
    - Process ID.
    - Uptime.

## Building the Application:

### Docker / Container:

We have a `Dockerfile` available to build the project locally.

#### How it works:

- It uses, `rustlang/rust:nightly-slim` to build the project.
- It then uses, `ubuntu:18.04` as a runtime for the built binary.
- It initializes an environment variable as `ROCKET_PORT` and sets it to `8000` to serve the project on port `8000`.

#### Building it:

```shell
$ docker build -t nasirhm/hello-nix-rust .
```

#### Running the built container image:

```shell
$ docker run --rm -it -p 8000:8000 nasirhm/hello-nix-rust
```

It'll run the application in a Docker container and map port 8000 of host to the port 8000 of the container.

### Nix:

#### How it works:

- With `nix`, `lorri`, `direnv` installed and `lorri daemon` running in the backgroud, when you'll `cd .` into the directory, It'll create a nix-shell with Rust available in it.
- It'll fetch rust from, `mozilla/nixpkgs-mozilla` from `nightly` channel and version `2020-11-01` overlaid into it.
- There are 2 nix configs for building the binary.
    - One being: `helloworld.nix`, which will do the following things:
        - Download naerksk
        - Download every Rust crate our HTTP service depends on into the Nix store
        - Run your program's tests
        - Build your dependencies into a Nix package
        - Build your program with those dependencies
        - Place a link to the result at `./result`
        - The resultant binary will be in: `./result/bin/helloworld`
    - The other being: `default.nix`, which will do the following things:
        - Do the stuff, done by `helloworld.nix`.
        - nixpkgs provides `dockerTools` which we will be using to create docker image out of Nix package.
        - It'll create a docker image with `nasirhm/hello-nix-rust` as name and `latest` as a tag.
        - It'll also set `ROCKET_PORT` to 5000 to let Rocket know to run application on port 5000.

#### Building the Nix Package

To build the Nix Package.

```shell
$ nix-build helloworld.nix
```

To run:

```shell
$ ./result/bin/helloworld
```

#### Building the Container image from dockerTools

To build the Container image:

```shell
$ nix-build default.nix
```

It'll create a tarball containing the docker image information as the result of the Nix build.

To run the image with Docker, we first have to load it:

```shell
$ docker load -i result
```

and then run it using `docker run`:

```shell
$ docker run --rm -itp 8000:5000 nasirhm/hello-nix-rust
```

It'll map port 8000 of the host with 5000 of the container.

**NOTE:** We can also use `podman` instead of `docker` as a container engine / runtime.

### To Test:

To test the application, we can use `cargo` for it:

```shell
$ cargo test
```

It has the following testcases:

- For the `/` endpoint:
   - It makes sure, HTTP reponse Status Ok 200 is returned.
   - and, ensures, it returns `Hello, World!` as a response.
- For the `/hostinfo` endpoint:
   - It makes sure, HTTP response Status Ok 200 is returned.
   - and, ensures, we get the correct / current hostname, process id and uptime (in secs).
   - in JSON schema.

**Thank You,** [Xe](https://github.com/xe) for writing the following awesome articles from which I created this short project for learning more about Rust and Nix:

- [How I Start: Nix](https://christine.website/blog/how-i-start-nix-2020-03-08)
- [How I Start: Rust](https://christine.website/blog/how-i-start-rust-2020-03-15)
