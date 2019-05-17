FROM rust:1.34.2 AS base
RUN mkdir app
WORKDIR ./app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN rustup install nightly
RUN rustup default nightly
RUN cargo install cargo-build-deps --verbose --color always
RUN cargo build-deps --release

ADD src src

ARG name=youtube
RUN cargo build --package $name --bin $name --verbose --jobs 4 --all-features --release --target=nightly-x86_64-unknown-linux-gnu --color always

FROM scratch
COPY --from=base /root/app/target/nightly-x86_64-unknown-linux-gnu/release/youtube /main

ENTRYPOINT ["/main"]