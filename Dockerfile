FROM rust:1.69 AS builder

WORKDIR /opt

COPY . .
RUN ["cargo", "build", "--release", "-F", "graphql-playground"]

FROM ubuntu:23.10 AS runner

RUN apt-get update
RUN apt-get install -y libpq5

WORKDIR /srv
COPY --from=builder /opt/config/development.yaml config/development.yaml
COPY --from=builder /opt/target/release/glints-api glints-api

CMD ["./glints-api"]
