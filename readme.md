to run this use should use the command 'trunk serve'



FROM rust:latest
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
VOLUME /code
WORKDIR /code
EXPOSE 8081
CMD trunk serve
