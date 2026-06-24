################################################################################
# Create a stage for building the application.

FROM --platform=$BUILDPLATFORM rust:1.88.0-alpine3.21@sha256:9c6a4baf58661f99a5441b15e3ad8295dabf35e849c4935e77ad35d9809be1d2 AS build
WORKDIR /app

# renovate: datasource=repology depName=alpine_3_21/zig versioning=loose
ENV ZIG_VERSION="0.13.0-r1"
# renovate: datasource=repology depName=alpine_3_21/musl-dev versioning=loose
ENV MUSL_DEV_VERSION="1.2.5-r9"
# renovate: datasource=crate depName=cargo-zigbuild versioning=semver
ENV ZIGBUILD_VERSION="0.20.1"

# Install deps
RUN apk add --no-cache \
        musl-dev=${MUSL_DEV_VERSION} \
        zig=${ZIG_VERSION} \
    && \
    cargo install --locked cargo-zigbuild@${ZIGBUILD_VERSION} && \
    rustup target add \
        x86_64-unknown-linux-musl \
        aarch64-unknown-linux-musl \
        riscv64gc-unknown-linux-musl

# Build the application.
# Leverage a cache mount to /usr/local/cargo/registry/
# for downloaded dependencies, a cache mount to /usr/local/cargo/git/db
# for git repository dependencies, and a cache mount to /app/target/ for
# compiled dependencies which will speed up subsequent builds.
# Leverage a bind mount to the src directory to avoid having to copy the
# source code into the container. Once built, copy the executable to an
# output directory before the cache mounted /app/target is unmounted.
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo zigbuild --release \
       --target x86_64-unknown-linux-musl \
       --target aarch64-unknown-linux-musl \
       --target riscv64gc-unknown-linux-musl && \
    mkdir /app/linux && \
    cp target/aarch64-unknown-linux-musl/release/arr-backup /app/linux/arm64 && \
    cp target/x86_64-unknown-linux-musl/release/arr-backup /app/linux/amd64 && \
    cp target/riscv64gc-unknown-linux-musl/release/arr-backup /app/linux/riscv64

################################################################################
FROM alpine:3.22@sha256:14358309a308569c32bdc37e2e0e9694be33a9d99e68afb0f5ff33cc1f695dce AS final
ARG TARGETPLATFORM

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/go/dockerfile-user-best-practices/
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Copy the executable from the "build" stage.

COPY --from=build /app/${TARGETPLATFORM} /bin/arr-backup

CMD [ "/bin/arr-backup" ]
