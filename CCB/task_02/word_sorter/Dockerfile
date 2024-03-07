FROM rust:1.72 AS builder
COPY . .
RUN cargo build --release

FROM rockylinux:9
COPY --from=builder ./target/release/word_sorter ./target/release/word_sorter
ENTRYPOINT ["/target/release/word_sorter"]