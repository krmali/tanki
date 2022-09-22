# FROM rust:1.63-slim as build
#
# EXPOSE 3000/tcp
#
# COPY ./ /usr/src
# WORKDIR /usr/src
#
# RUN apt-get update && apt-get install -y \
#     git \
#     curl \
#     build-essential \
#     && rm -rf /var/lib/apt/lists/*
#
#
# RUN rustup target add wasm32-unknown-unknown
# RUN cargo install trunk
# RUN rm src/env.rs
# RUN mv src/env.rs.prod src/env.rs
# RUN cargo install --locked wasm-bindgen-cli
# RUN trunk build --release
# RUN mv german.json dist/german.json

# production environment
FROM nginx:stable-alpine
# COPY --from=build /usr/src /usr/share/nginx/html
COPY ./german.json /usr/share/nginx/html/german.json
ADD ./dist_prod /usr/share/nginx/html/
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
