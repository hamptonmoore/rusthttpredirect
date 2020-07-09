FROM rust:alpine as builder
COPY . /redirect
WORKDIR /redirect
RUN cargo build --release

FROM alpine
COPY --from=builder /redirect/target/release/redirect /bin/redirect
CMD ["/bin/redirect"]

EXPOSE 8084
