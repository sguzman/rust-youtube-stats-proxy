FROM rust:1.34.2 AS base
RUN mkdir app
WORKDIR ./app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
ADD src src

RUN rustup install nightly
RUN rustup default nightly

ARG name=youtube
RUN cargo build --package rust-youtube-stats-proxy --bin $name --verbose --jobs 4 --all-features --release --target=x86_64-unknown-linux-gnu --color always

FROM scratch
COPY --from=base /app/target/x86_64-unknown-linux-gnu/release/youtube /main

RUN chmod +x /main
ENTRYPOINT ["/main"]