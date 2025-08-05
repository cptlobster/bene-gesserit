# syntax: docker/dockerfile
# Rust builder image
FROM rust:alpine AS rs_builder

# Install all dependencies for building
RUN apk add --no-cache openssl-dev openssl pkgconf musl-dev make perl

# Copy all necessary source files
WORKDIR /usr/src/bene-gesserit
COPY src ./src
COPY Cargo.toml ./Cargo.toml
COPY Cargo.lock ./Cargo.lock

# Compile the generator executable
RUN cargo install --path .

# Curler image (downloads binaries for component programs)
FROM alpine:3.22 AS curler
# Install curl
RUN apk add --no-cache curl zstd

ARG IOCAINE_VERSION="2.5.0"
ARG ANUBIS_VERSION="1.21.3"

RUN curl -L -o iocaine.zst https://git.madhouse-project.org/api/packages/iocaine/generic/iocaine-binaries/${IOCAINE_VERSION}/iocaine-${IOCAINE_VERSION}.x86_64-linux.zst \
    && unzstd ./iocaine.zst \
    && chmod +x ./iocaine

RUN curl -L -o anubis.tar.gz https://github.com/TecharoHQ/anubis/releases/download/v${ANUBIS_VERSION}/anubis-${ANUBIS_VERSION}-linux-amd64.tar.gz \
    && tar -xzf ./anubis.tar.gz \
    && mv anubis-${ANUBIS_VERSION}-linux-amd64 anubis

# Final image, contains all built dependencies
FROM openresty/openresty:alpine AS final

WORKDIR /etc/bene_gesserit

RUN apk add --no-cache supervisor

# Copy compiled generator executable from builder image
COPY --from=rs_builder /usr/local/cargo/bin/bene_gesserit /usr/local/bin/bene_gesserit
COPY --from=curler ./iocaine /usr/local/bin/iocaine
COPY --from=curler ./anubis/bin/anubis /usr/local/bin/anubis

# Copy template/include files
COPY static static
COPY templates templates
COPY overseer.sh overseer.sh

# Use overseer script as entrypoint
ENTRYPOINT ./overseer.sh