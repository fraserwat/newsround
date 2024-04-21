# Build stage
FROM rust:latest as builder
WORKDIR /usr/src/newsround
COPY . .
RUN cargo build --release

# Runtime stage using Ubuntu for newer GLIBC
FROM ubuntu:latest
# Download and compile OpenSSL 3.0
RUN apt-get update && apt-get install -y wget build-essential
RUN wget https://www.openssl.org/source/openssl-3.0.0.tar.gz && \
    tar -xzvf openssl-3.0.0.tar.gz && \
    cd openssl-3.0.0 && \
    ./config && \
    make && \
    make install && \
    cd .. && rm -rf openssl-3.0.0 openssl-3.0.0.tar.gz && \
    apt-get remove --purge -y wget build-essential && \
    apt-get autoremove -y && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/newsround/target/release/newsround /usr/local/bin/newsround
CMD ["newsround"]
