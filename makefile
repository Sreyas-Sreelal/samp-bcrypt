ifdef OS
	TOOLCHAIN = +stable-i686-pc-windows-msvc
	BINARYNAME = samp_bcrypt.dll
	OUPUTNAME = samp_bcrypt.dll
	CP_RELEASE = copy .\target\release\$(BINARYNAME) .\plugins\$(OUPUTNAME)
	CP_DEBUG = copy .\target\debug\$(BINARYNAME) .\plugins\$(OUPUTNAME)
else
	ifeq ($(shell uname), Linux)
		TOOLCHAIN = +stable-i686-unknown-linux-gnu
		BINARYNAME = libsamp_bcrypt.so
		OUPUTNAME = samp_bcrypt.so
		CP_RELEASE = cp target/release/$(BINARYNAME) plugins/$(OUPUTNAME)
		CP_DEBUG = cp target/debug/$(BINARYNAME) plugins/$(OUPUTNAME)
	endif
endif

release:
	cargo $(TOOLCHAIN) build --release
	$(CP_RELEASE)

debug:
	cargo $(TOOLCHAIN) build
	$(CP_DEBUG)

setup:
	sampctl package ensure
	sampctl package build

ensure:
	sampctl package ensure
	
run:
	sampctl package build
	sampctl package run
	
clean:
	cargo clean
