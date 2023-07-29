FROM node:16.18.1-alpine3.16 AS JS_BUILD
COPY webapp /webapp
WORKDIR /webapp
RUN npm install && npm run build

FROM rust:latest AS RUST_BUILD
COPY server /server
WORKDIR /server
RUN cargo build --release

FROM alpine:3.16.3
COPY --from=JS_BUILD /webapp/build* ./webapp/
COPY --from=RUST_BUILD /server/target/release/server ./
CMD ./server
