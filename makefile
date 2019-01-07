ifdef OS
	TOOLCHAIN = +nightly-i686-pc-windows-msvc
	BINARYNAME = samp_bcrypt.dll
	OUPUTNAME = samp_bcrypt.dll
else
	ifeq ($(shell uname), Linux)
		TOOLCHAIN = +nightly-i686-unknown-linux-gnu
		BINARYNAME = libsamp_bcrypt.so
		OUPUTNAME = samp_bcrypt.so
	endif
endif

release:
	cargo $(TOOLCHAIN) build --release
	cp target/release/$(BINARYNAME) test/plugins/$(OUPUTNAME)

debug:
	cargo $(TOOLCHAIN) build
	cp target/debug/$(BINARYNAME) test/plugins/$(OUPUTNAME)

setup:
	cd test && mkdir plugins
	cd test && mkdir gamemodes

ensure:
	cd test && sampctl server ensure
	sampctl package ensure

run:
	sampctl package build
	cd test && sampctl server run
	
clean:
	cargo clean
	rm -rf test
