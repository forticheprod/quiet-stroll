FROM rust:latest
RUN cargo install --git https://github.com/doubleailes/quiet-stroll.git
COPY Rocket.toml ./
HEALTHCHECK --interval=10m --timeout=2s --retries=3 CMD curl --fail http://localhost || exit 1
CMD ["quiet-stroll"]