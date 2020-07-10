FROM rust:buster as builder
COPY . /redirect
WORKDIR /redirect
RUN cargo build --release

FROM debian:buster
COPY --from=builder /redirect/target/release/redirect /bin/redirect
CMD ["/bin/redirect"]

EXPOSE 8084
