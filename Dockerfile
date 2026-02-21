# Use nightly to support the newest editions
FROM rustlang/rust:nightly

# Install the WebAssembly target
RUN rustup target add wasm32v1-none

# Install the missing Linux system dependency
RUN apt-get update && apt-get install -y libdbus-1-3

# Install the Stellar CLI
RUN curl -fsSL https://github.com/stellar/stellar-cli/raw/main/install.sh | sh

WORKDIR /workspace