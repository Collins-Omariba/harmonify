FROM rust:1.80 as builder

WORKDIR /app

# Install necessary build tools and libraries
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# For development via docker-compose, we just run the app using `cargo run`.
# The docker-compose mounts the local folder into `/app`.

CMD ["cargo", "run"]
