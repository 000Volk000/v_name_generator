FROM rust:1.19.1 AS v_name_generator

WORKDIR /app
COPY . /app

RUN cargo install

CMD ["v_name_generator", "ntfy"]
