SDKENV:=$(shell echo $$WIND_SDK_HOME)
ifndef SDKENV
    $(error Advise: source /path/to/sdkenv.sh # Wind River SDK)
endif
VXWORK_PATH=$${WIND_SDK_HOME}
RUST_LIB_DIR=${VXWORK_PATH}/compilers/rust-1.70.0.0/x86_64-unknown-linux-gnu/lib/rustlib/x86_64-wrs-vxworks/lib/

all: libwrapper.a
	RUSTFLAGS="-L ${RUST_LIB_DIR} -l static=wrapper" cargo build --release

libwrapper.a: wrapper.c
	wr-cc -rtp -c wrapper.c -o wrapper.o
	ar rcs libwrapper.a wrapper.o
	cp libwrapper.a ${RUST_LIB_DIR}

clean:
	@rm -rf wrapper.o libwrapper.a
	@cargo clean

.PHONY: clean
