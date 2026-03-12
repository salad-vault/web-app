# ---- Stage 1 : Build WASM + Trunk ----
FROM rust:1.88-slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Cible WASM
RUN rustup target add wasm32-unknown-unknown

# Trunk pre-compile (même version que le projet)
RUN curl -L --proto '=https' --tlsv1.2 \
    "https://github.com/thedodd/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-gnu.tar.gz" \
    | tar xz -C /usr/local/bin

WORKDIR /app

# Copie de tout le source (styles.css est à la racine, pas dans src/)
COPY . .

# Build de production (génère dist/)
RUN trunk build --release

# ---- Stage 2 : Serveur nginx statique ----
FROM nginx:alpine AS runtime

# Utilisateur non-root pour limiter la surface d'attaque
RUN addgroup -S appgroup && adduser -S appuser -G appgroup \
    && chown -R appuser:appgroup /var/cache/nginx /var/log/nginx \
    && touch /var/run/nginx.pid && chown appuser:appgroup /var/run/nginx.pid \
    && sed -i 's/^user  nginx;/# user  nginx;/' /etc/nginx/nginx.conf

COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /app/dist/ /usr/share/nginx/html/

USER appuser

EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:3000/ || exit 1

CMD ["nginx", "-g", "daemon off;"]
