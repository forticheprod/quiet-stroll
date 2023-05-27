FROM rust:latest
RUN cargo install --git https://github.com/doubleailes/quiet-stroll.git
COPY Rocket.toml ./
CMD ["quiet-stroll"]