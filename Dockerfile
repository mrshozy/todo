FROM rust:latest as builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

RUN mv ./target/release/todo /usr/local/bin/todo

EXPOSE 3000

CMD ["todo"]
