FROM rust as build
RUN apt-get update && apt-get -y upgrade
RUN apt-get install libssl-dev
RUN apt-get -y install pkg-config musl musl-dev musl-tools
RUN rustup target add x86_64-unknown-linux-musl

# copy app into Docker image
COPY . /app

# Set the workdirectory
WORKDIR /app

# build the app
RUN cargo build --target x86_64-unknown-linux-musl --release --bin api-sdt


#FROM gcr.io/distroless/cc-debian11
#FROM alpine:3.16.0 AS runtime
FROM scratch

COPY --from=build /app/target/x86_64-unknown-linux-musl/release/api-sdt /app/api-sdt

WORKDIR /app

COPY ./api ./api
COPY ./config ./config

EXPOSE 8080

CMD ["./api-sdt"]
