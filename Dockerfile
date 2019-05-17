FROM liuchong/rustup:musl AS base
RUN mkdir app
WORKDIR ./app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN rustup target add x86_64-unknown-linux-musl
RUN rustup install nightly
RUN cargo install cargo-build-deps --verbose --color always
RUN cargo build-deps --release

ADD src src

ARG name=youtube
RUN cargo build --package $name --bin $name --verbose --jobs 4 --all-features --release --target=x86_64-unknown-linux-musl --color always

FROM scratch
COPY --from=base /root/app/target/x86_64-unknown-linux-musl/release/youtube /main

ENTRYPOINT ["/main"]