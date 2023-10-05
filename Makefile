.PHONY: dist

# Compile the binaries for all targets
build: build-x86_64-unknown-linux-musl

build-x86_64-unknown-linux-musl:
	cross build --target x86_64-unknown-linux-musl --release

# Build distributable binaries for all targets
dist: dist-x86_64-unknown-linux-musl

dist-x86_64-unknown-linux-musl: build-x86_64-unknown-linux-musl package-x86_64-unknown-linux-musl

# Package the compiled binaries
package-x86_64-unknown-linux-musl:
	$(eval PKG_VERSION := $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version'))
	mkdir -p dist

	# .tar.gz
	tar -czvf dist/chirpstack-integration-pulsar_$(PKG_VERSION)_amd64.tar.gz -C target/x86_64-unknown-linux-musl/release chirpstack-integration-pulsar

# Update the version
version:
	test -n "$(VERSION)"
	sed -i 's/^version.*/version = "$(VERSION)"/g' ./Cargo.toml
	make test
	git add .
	git commit -v -m "Bump version to $(VERSION)"
	git tag -a v$(VERSION) -m "v$(VERSION)"

# Cleanup dist.
clean:
	cargo clean
	rm -rf dist

# Run tests.
test:
	cargo clippy
	cargo test

# Enter the devshell.
devshell:
	nix-shell
