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

RUN curl -o iocaine.zst https://git.madhouse-project.org/api/packages/iocaine/generic/iocaine-binaries/latest/iocaine-latest.x86_64-linux.zst \
    && unzstd ./iocaine.zst

# Final image, contains all built dependencies
FROM alpine:3.22 AS final

WORKDIR /etc/bene_gesserit

# Copy compiled generator executable from builder image
COPY --from=rs_builder /usr/local/cargo/bin/bene_gesserit /usr/local/bin/bene_gesserit
COPY --from=curler ./iocaine /usr/local/bin/iocaine

# Copy template/include files
COPY docker_include docker_include
COPY templates templates
COPY overseer.sh overseer.sh

# Use overseer script as entrypoint
ENTRYPOINT ./overseer.sh