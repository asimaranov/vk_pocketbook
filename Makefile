LINUX_SDK_V4_ROOT=""
LINUX_SDK_V4_SYSROOT=$(SDK_V4_ROOT)/arm-obreey-linux-gnueabi/sysroot
LINUX_SDK_V4_LINKER=$(SDK_V4_ROOT)/bin/arm-obreey-linux-gnueabi-gcc

LINUX_SDK_V6_ROOT=""
LINUX_SDK_V6_SYSROOT=$(LINUX_SDK_V6_ROOT)/usr/arm-obreey-linux-gnueabi/sysroot
LINUX_SDK_V6_LINKER=$(LINUX_SDK_V6_ROOT)/usr/bin/arm-obreey-linux-gnueabi-clang

MACOS_SDK_V6_ROOT = "/Users/andrey/Downloads/arm-unknown-linux-gnueabi"
MACOS_SDK_V6_SYSROOT=$(MACOS_SDK_V6_ROOT)/arm-unknown-linux-gnueabi/sysroot
MACOS_SDK_V6_LINKER=$(MACOS_SDK_V6_ROOT)/bin/arm-unknown-linux-gnueabi-gcc


linux_sdk_v4:
	RUSTFLAGS=" -C linker=$(LINUX_SDK_V4_LINKER) -linkview" \
	BINDGEN_EXTRA_CLANG_ARGS="--sysroot=$(LINUX_SDK_V4_SYSROOT)/ -I$(LINUX_SDK_V4_SYSROOT)/usr/include/freetype2" \
	cargo build --target arm-unknown-linux-gnueabi --features inkview-sys/sdk_v4 $(ARGS)


linux_sdk_v6:
	RUSTFLAGS=" -C linker=$(LINUX_SDK_V6_LINKER)  -linkview" \
	BINDGEN_EXTRA_CLANG_ARGS="--sysroot=$(LINUX_SDK_V6_SYSROOT)/ -I$(LINUX_SDK_V6_SYSROOT)/usr/include/freetype2 -I$(LINUX_SDK_V6_SYSROOT)/usr/include/" \
	cargo build --target arm-unknown-linux-gnueabi --features inkview-sys/sdk_v6 $(ARGS)


macos_v6:
	RUST_BACKTRACE=1 \
	RUSTFLAGS=" -C linker=${MACOS_SDK_V6_LINKER} -linkview -lssl" \
	CC=${MACOS_SDK_V6_LINKER} \
	BINDGEN_EXTRA_CLANG_ARGS="--sysroot=$(MACOS_SDK_V6_SYSROOT)/ -I$(MACOS_SDK_V6_SYSROOT)/usr/include/freetype2 -I$(MACOS_SDK_V6_SYSROOT)/usr/include/ -I$(MACOS_SDK_V6_SYSROOT)/usr/include/openssl/" \
	cargo build --target arm-unknown-linux-gnueabi --release $(ARGS)

macos_v6_debug:
	RUSTFLAGS=" -C linker=${MACOS_SDK_V6_LINKER} -linkview" \
	BINDGEN_EXTRA_CLANG_ARGS="--sysroot=$(MACOS_SDK_V6_SYSROOT)/ -I$(MACOS_SDK_V6_SYSROOT)/usr/include/freetype2 -I$(MACOS_SDK_V6_SYSROOT)/usr/include/" \
	cargo build --target arm-unknown-linux-gnueabi $(ARGS)

macos_v4:
	RUST_BACKTRACE=1 \
	RUSTFLAGS=" -C linker=${MACOS_SDK_V4_LINKER} -linkview" \
	BINDGEN_EXTRA_CLANG_ARGS="--sysroot=$(MACOS_SDK_V4_SYSROOT)/ -I$(MACOS_SDK_V4_SYSROOT)/usr/include/freetype2 -I$(MACOS_SDK_V4_SYSROOT)/usr/include/" \
	cargo build --target arm-unknown-linux-gnueabi --release $(ARGS)



cp_app:
	cp target/arm-unknown-linux-gnueabi/release/pocketbook_vk my_app.app
	$(LINUX_SDK_V6_ROOT)/usr/arm-obreey-linux-gnueabi/bin/strip my_app.app

cp_app_macos:
	cp target/arm-unknown-linux-gnueabi/release/pocketbook_vk my_app.app

	$(MACOS_SDK_V6_ROOT)/bin/arm-unknown-linux-gnueabi-strip my_app.app


cp_app_macos_debug:
	cp target/arm-unknown-linux-gnueabi/debug/egui_template my_app.app

	$(MACOS_SDK_V6_ROOT)/bin/arm-unknown-linux-gnueabi-strip my_app.app


ftp_transfer:
	./upload_ftp.sh

