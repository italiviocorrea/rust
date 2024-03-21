FROM rust as builder
RUN apt-get update && apt-get -y upgrade
RUN apt install openssl
RUN apt install libssl-dev
RUN apt-get install -y pkg-config libxml2-dev libxmlsec1-dev libxmlsec1-openssl

# copy app into Docker image
COPY . /app

# Set the workdirectory
WORKDIR /app

# build the app
RUN cargo build --release --bin couchdb-kafka-connector


FROM scratch
MAINTAINER Italivio Correa

COPY --from=builder /lib/x86_64-linux-gnu/libssl.so.3 /lib/x86_64-linux-gnu/libssl.so.3
COPY --from=builder /lib/x86_64-linux-gnu/libcrypto.so.3 /lib/x86_64-linux-gnu/libcrypto.so.3
COPY --from=builder /lib/x86_64-linux-gnu/libz.so.1 /lib/x86_64-linux-gnu/libz.so.1
COPY --from=builder /lib/x86_64-linux-gnu/libgcc_s.so.1 /lib/x86_64-linux-gnu/libgcc_s.so.1
COPY --from=builder /lib/x86_64-linux-gnu/libm.so.6 /lib/x86_64-linux-gnu/libm.so.6
COPY --from=builder /lib/x86_64-linux-gnu/libc.so.6 /lib/x86_64-linux-gnu/libc.so.6
COPY --from=builder /lib64/ld-linux-x86-64.so.2 /lib64/ld-linux-x86-64.so.2


COPY --from=builder /app/target/release/couchdb-kafka-connector /app/ckc

WORKDIR /app

CMD ["./ckc"]