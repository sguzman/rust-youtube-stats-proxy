FROM liuchong/rustup:nightly-musl AS base
WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml
ADD src src

RUN cargo build --package rust-youtube-stats-proxy --bin youtube --verbose --jobs 4 --release --target=x86_64-unknown-linux-musl --color always

FROM scratch
COPY --from=base /app/target/x86_64-unknown-linux-musl/release/youtube /main
ENTRYPOINT ["/main"]