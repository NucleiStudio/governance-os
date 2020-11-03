FROM ubuntu as builder

LABEL maintainer="eliott@nuclei.studio"

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    clang \
    cmake \
    curl \
    git \
    libssl-dev \
    pkg-config

WORKDIR /building
ADD . .

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    export PATH=$PATH:$HOME/.cargo/bin && \
    ./scripts/init.sh && \
    cargo build -p governance-os-node --release && \
    cp target/release/governance-os-node /node

# ===== SECOND STAGE ======

FROM ubuntu

COPY --from=builder /node /usr/local/bin

RUN useradd --create-home runner

USER runner
EXPOSE 30333 9933 9944

ENTRYPOINT ["node"]
