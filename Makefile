SDKENV:=$(shell echo $$WIND_SDK_HOME)
ifndef SDKENV
    $(error Advise: source /path/to/sdkenv.sh # Wind River SDK)
endif
VXWORK_PATH=$${WIND_SDK_HOME}
RUSTLIBOUTDIR=${VXWORK_PATH}/lib/rustlib/x86_64-wrs-vxworks/lib

all: libwrapper.a
	RUSTFLAGS="-L ${VXWORK_PATH}/compilers/rust-1.70.0.0/x86_64-unknown-linux-gnu/bin/../lib/rustlib/x86_64-wrs-vxworks/lib/ -l static=wrapper" cargo build --release

libwrapper.a: wrapper.c
	wr-cc -rtp -c wrapper.c -o wrapper.o
	ar rcs libwrapper.a wrapper.o
	@if [ ! -d ${RUSTLIBOUTDIR} ]; then \
        mkdir -p ${RUSTLIBOUTDIR}; \
    fi
	cp *wrapper* ${RUSTLIBOUTDIR}
	rm libwrapper.a wrapper.o

clean:
	@rm -rf *.o target

.PHONY: clean
