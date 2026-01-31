FROM rust:1.91.1 AS v-name-generator

WORKDIR /app
COPY . /app

RUN cargo install --path /app --features="app"

CMD ["sleep", "infinity"]
