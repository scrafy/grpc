FROM rust as build

# create a new empty shell project
RUN USER=root cargo new --bin grpc_test
WORKDIR /grpc_test

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/grpc_test*
RUN cargo build --release

# our final base
FROM rust

# copy the build artifact from the build stage
COPY --from=build /grpc_test/target/release/grpc_test .

# set the startup command to run your binary
CMD ["./grpc_test"]