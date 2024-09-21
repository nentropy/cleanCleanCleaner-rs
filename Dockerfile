FROM rust:1.68

WORKDIR /usr/src/blackhut

COPY . .

RUN cargo build --release

CMD ["./target/release/blackhut"]

# docker build -t blackhut .
# docker run -it --rm blackhut