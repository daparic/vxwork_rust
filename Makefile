all: libwrapper.a
	RUSTFLAGS="-L /home/fox_white/vxwork/compilers/rust-1.70.0.0/x86_64-unknown-linux-gnu/bin/../lib/rustlib/x86_64-wrs-vxworks/lib/ -L /home/fox_white/vxwork/hello/vxsdk/sysroot/krnl/h/public/ -l static=wrapper" cargo build --release

libwrapper.a: wrapper.c wrapper.h
	wr-cc -rtp -c wrapper.c -static -o wrapper.o
	ar rcs libwrapper.a wrapper.o
	cp *wrapper* /home/fox_white/vxwork/lib/rustlib/x86_64-wrs-vxworks/lib
	rm libwrapper.a wrapper.o