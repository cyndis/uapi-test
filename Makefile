REMOTE=192.168.1.62
REMOTE_USER=root
REMOTE_PASSWORD=root
REMOTE_TARGET_64=aarch64-unknown-linux-musl
REMOTE_TARGET_32=armv7-unknown-linux-musleabi
LINKERS=CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-linux-gnu-gcc CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABI_LINKER=arm-linux-gnueabi-gcc

ARGS?=

CARGO_ENV=env $(LINKERS)
ENV_64=RUSTFLAGS="-C target-feature=+crt-static -C link-arg=-lgcc"

build-64:
	$(CARGO_ENV) $(ENV_64) cargo build --release --target $(REMOTE_TARGET_64)
	cp target/$(REMOTE_TARGET_64)/release/uapi-test ./uapi-test-64

build-32:
	$(CARGO_ENV) cargo build --release --target $(REMOTE_TARGET_32)
	cp target/$(REMOTE_TARGET_32)/release/uapi-test ./uapi-test-32

run-remote-64: build-64
	@sshpass -p '$(REMOTE_PASSWORD)' scp ./uapi-test-64 $(REMOTE_USER)@$(REMOTE):./
	@sshpass -p '$(REMOTE_PASSWORD)' ssh $(REMOTE_USER)@$(REMOTE) env RUST_LIB_BACKTRACE=1 ./uapi-test-64 $(ARGS)

run-remote-32: build-32
	@sshpass -p '$(REMOTE_PASSWORD)' scp ./uapi-test-32 $(REMOTE_USER)@$(REMOTE):./
	@sshpass -p '$(REMOTE_PASSWORD)' ssh $(REMOTE_USER)@$(REMOTE) ./uapi-test-32 $(ARGS)

.PHONY: build-64 build-32 run-remote-64 run-remote-32
