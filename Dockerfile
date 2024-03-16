FROM python:3.11-slim as dev
# Get update system packages and install essentials to build rust
RUN apt-get -qq update && \
    apt-get install -y -q \
    build-essential \
    curl && curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /app/gladoid-bevy
RUN cargo install cargo-watch && pip install poetry --no-cache-dir && poetry config virtualenvs.create false && poetry install --without dev --no-interaction --no-cache
COPY . .

FROM python:3.11-slim as builder
RUN apt-get -qq update && \
    apt-get install -y -q \
    build-essential \
    curl && curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /build

COPY . .
RUN cargo build --release

FROM python:3.11-slim as release

ENV PYTHONDONTWRITEBYTECODE=1
ENV PYTHONUNBUFFERED=1

RUN pip install poetry --no-cache-dir && poetry config virtualenvs.create false &&  poetry install --without dev --no-interaction --no-cache

WORKDIR /app
# Copy the compiled gladoid-bevy
COPY --from=builder /build/target/release/gladoid-bevy /app/gladoid-bevy
CMD ["./gladoid-bevy"]
