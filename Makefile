VXWORK_PATH=/home/fox_white/vxwork

all: libwrapper.a
	RUSTFLAGS="-L ${VXWORK_PATH}/compilers/rust-1.70.0.0/x86_64-unknown-linux-gnu/bin/../lib/rustlib/x86_64-wrs-vxworks/lib/ -l static=wrapper" cargo build --release

libwrapper.a: wrapper.c
	wr-cc -rtp -c wrapper.c -o wrapper.o
	ar rcs libwrapper.a wrapper.o
	cp *wrapper* ${VXWORK_PATH}/lib/rustlib/x86_64-wrs-vxworks/lib
	rm libwrapper.a wrapper.o