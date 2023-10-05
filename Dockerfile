# Copy binary stage
FROM --platform=$BUILDPLATFORM alpine:3.18.0 as binary

ARG TARGETPLATFORM

COPY target/x86_64-unknown-linux-musl/release/chirpstack-pulsar-integration /usr/bin/chirpstack-pulsar-integration-x86_64


RUN case "$TARGETPLATFORM" in \
	"linux/amd64") \
		cp /usr/bin/chirpstack-pulsar-integration-x86_64 /usr/bin/chirpstack-pulsar-integration; \
		;; \
	"linux/arm/v7") \
		cp /usr/bin/chirpstack-pulsar-integration-armv7hf /usr/bin/chirpstack-pulsar-integration; \
		;; \
	"linux/arm64") \
		cp /usr/bin/chirpstack-pulsar-integration-aarch64 /usr/bin/chirpstack-pulsar-integration; \
		;; \
	esac;

# Final stage
FROM alpine:3.18.0

RUN apk --no-cache add \
    ca-certificates

COPY --from=binary /usr/bin/chirpstack-pulsar-integration /usr/bin/chirpstack-pulsar-integration
USER nobody:nogroup
ENTRYPOINT ["/usr/bin/chirpstack-pulsar-integration"]
