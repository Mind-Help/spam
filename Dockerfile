FROM rust:1.64.0 as builder
ARG DATABASE_URL
ARG SMTP_USERNAME
ARG OUR_EMAIL
ARG SMTP_PASSWD
ARG SMTP_HOST
ARG SMTP_PORT
ARG DOMAIN
ARG PORT
WORKDIR /opt/spam
COPY . .
RUN cargo build --release
CMD ["cargo", "run", "--release"]

# FROM alpine:latest
# RUN apk update && apk upgrade
# RUN apk add bash dos2unix
# COPY --from=builder /opt/spam/target/release/spam ./
# COPY --from=builder /opt/spam/run.sh ./
# ENTRYPOINT ["bash", "./run.sh"]