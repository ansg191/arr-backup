FROM --platform=$BUILDPLATFORM tonistiigi/xx:1.5.0 AS xx

FROM --platform=$BUILDPLATFORM rust:1.82-alpine AS build

COPY --from=xx / /
ARG TARGETPLATFORM

WORKDIR /app

RUN xx-info env && \
    apk add musl-dev clang lld && \
    xx-apk add musl-dev clang lld

# Download and build deps
RUN --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    xx-cargo build --locked --release

# Build the application.
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
#    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
xx-cargo build --locked --release && \
xx-verify --static ./target/$(xx-cargo --print-target-triple)/release/arr-backup && \
cp ./target/$(xx-cargo --print-target-triple)/release/arr-backup /bin/arr-backup

################################################################################
FROM alpine:3.20.3 AS final

# Create a non-privileged user that the app will run under.
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
COPY --from=build /bin/arr-backup /bin/

# What the container should run when it is started.
CMD ["/bin/arr-backup"]
