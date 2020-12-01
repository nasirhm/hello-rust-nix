# Use the minimal image
FROM rustlang/rust:nightly-slim AS build

# Where we will build the program
WORKDIR /src/helloworld

# Copy source code into the container
COPY . .

# Build the program in release mode
RUN cargo build --release

# Create the runtime image
FROM ubuntu:18.04

# Set PORT to 8000 for Rocket
ENV ROCKET_PORT=8000

# Copy the compiled service binary
COPY --from=build /src/helloworld/target/release/helloworld /usr/local/bin/helloworld

# Start the helloworld service on container boot
CMD ["usr/local/bin/helloworld"]
