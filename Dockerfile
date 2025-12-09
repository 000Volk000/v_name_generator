FROM rust:1.91.1 AS v_name_generator

WORKDIR /app
COPY . /app

RUN cargo install --path /app
