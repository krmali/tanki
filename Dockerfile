FROM rust:1.63-slim as build

EXPOSE 3000/tcp

COPY ./ /usr/src
WORKDIR /usr/src

RUN apt-get update && apt-get install -y \
    git \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*


RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN rm env.rs
RUN mv env.rs.prod env.rs
RUN trunk build --release

# production environment
FROM nginx:stable-alpine
COPY --from=build /usr/src /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
