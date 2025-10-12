# Anirudh Konidala: dockerfile for running rust server microservices!

FROM rust:1.90-bookworm

RUN apt update && apt install -y \
    python3 \
    python3-pip \
    python3-venv \
    curl \
    && rm -rf /var/lib/apt/lists/*

# use uv because its faster and written in rust lol lmao (rust is just better lmao)
RUN curl -LsSf https://astral.sh/uv/install.sh | sh
ENV PATH="/root/.cargo/bin:${PATH}"
RUN uv tool install maturin

WORKDIR /app

COPY . .

RUN maturin develop

CMD ["python3", "test_game.py"]
