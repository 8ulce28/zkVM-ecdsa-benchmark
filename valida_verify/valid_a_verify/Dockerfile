#FROM ubuntu:24.04

# Install system dependencies
#RUN apt-get update && apt-get install -y pkg-config libssl-dev curl build-essential

# Install rustup nightly and set it for this directory
#RUN rustup install nightly && rustup override set nightly

# Set working directory
#WORKDIR /app

# Copy host source code
#COPY host /app/host

# Copy prebuilt guest binary
#COPY guest/target/valida-unknown-baremetal-gnu/debug/guest /app/guest_binary

# Set working dir to host and build with nightly
#WORKDIR /app/host
#RUN cargo build --release

# Run the host binary
#CMD ["./target/release/host"]


FROM ubuntu:24.04

RUN apt-get update && \
    apt-get install -y curl build-essential pkg-config libssl-dev \
    git ca-certificates

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y --default-toolchain nightly
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app

COPY host /app/host

COPY guest/target/valida-unknown-baremetal-gnu/debug/guest /app/guest_binary

WORKDIR /app/host
RUN cargo +nightly build --release

CMD ["./target/release/host"]


