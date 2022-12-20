FROM clux/muslrust:stable as build
COPY ./ ./

RUN cargo install --path .

FROM alpine
COPY --from=build /root/.cargo/bin/temperature-exporter /
CMD ["/temperature-exporter"]