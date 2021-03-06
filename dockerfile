FROM tim/rustup

ENV ROCKET_ADDRESS=0.0.0.0

ENV ROCKET_PORT=8080

ADD src /app

WORKDIR /app

RUN rustup default nightly

RUN cargo build

CMD ["cargo", "run"]
