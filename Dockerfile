FROM liuchong/rustup:nightly-musl AS base
RUN mkdir app
WORKDIR ./app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
ADD src src

RUN LIB_LDFLAGS=-L/usr/lib/x86_64-linux-gnu CFLAGS=-I/usr/local/musl/include CC=musl-gcc cargo build --package rust-youtube-stats-proxy --bin youtube --verbose --jobs 4 --all-features --release --target=x86_64-unknown-linux-musl --color always

FROM scratch
COPY --from=base ./target/x86_64-unknown-linux-musl/release/youtube /main
ENTRYPOINT ["/main"]