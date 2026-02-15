# ==========================================
# Stage 1: Build the WASM app with Trunk
# ==========================================
FROM rust:latest AS builder

# Install build dependencies (clang, lld needed by ring crate)
RUN apt-get update && apt-get install -y clang lld && rm -rf /var/lib/apt/lists/*

# Install wasm target
RUN rustup target add wasm32-unknown-unknown

# Install trunk
RUN cargo install trunk --locked

# Enable reference-types (required by wasm-bindgen 0.2.106+)
# ENV RUSTFLAGS="-C target-feature=+reference-types"

# Set working directory
WORKDIR /app

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to pre-build dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --target wasm32-unknown-unknown --release 2>/dev/null || true
RUN rm -rf src

# Now copy the actual source code
COPY . .

# Build the project with Trunk
RUN trunk build --release

# ==========================================
# Stage 2: Serve with Nginx
# ==========================================
FROM nginx:alpine AS runner

# Copy custom nginx config for SPA routing
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Copy built files from builder stage
COPY --from=builder /app/dist /usr/share/nginx/html

# Expose port 80
EXPOSE 80

# Start nginx
CMD ["nginx", "-g", "daemon off;"]
