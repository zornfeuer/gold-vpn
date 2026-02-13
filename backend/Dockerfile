# ══════════════════════════════════════════════════════════════════════════════
# СТУПЕНЬ 1: Сборка приложения
# ══════════════════════════════════════════════════════════════════════════════
FROM docker.io/library/rust:1.93-slim-bookworm AS builder

# Устанавливаем необходимые зависимости для сборки
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        pkg-config \
        libssl-dev \
        libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Создаём рабочую директорию
WORKDIR /app

# Копируем файлы зависимостей для кэширования
COPY Cargo.toml Cargo.lock ./

# Создаём пустой src/lib.rs для предварительной загрузки зависимостей
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo fetch --locked

# Копируем исходный код
COPY src/ ./src/

# Собираем приложение в режиме релиза с оптимизациями
RUN cargo build --release --locked && \
    strip /app/target/release/backend

# ══════════════════════════════════════════════════════════════════════════════
# СТУПЕНЬ 2: Финальный образ
# ══════════════════════════════════════════════════════════════════════════════
FROM docker.io/library/debian:bookworm-slim

# Устанавливаем только необходимые runtime зависимости
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl3 \
        libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Копируем бинарник из стадии сборки
COPY --from=builder /app/target/release/backend /usr/local/bin/backend

# Создаём директорию для конфигов (если понадобится)
RUN mkdir -p /etc/backend

USER root
WORKDIR /root

# Экспортируем порт
EXPOSE 8080

# Запуск приложения
ENTRYPOINT ["/usr/local/bin/backend"]
