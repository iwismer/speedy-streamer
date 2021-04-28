SOURCES = rfid_reader.cpp
OUT_NAME = rfid_reader
LIB_DIR = /usr/bin
MPNJ_LIB_DIR = lib
HEADER_DIR = include

ETK_DEFAULT_INSTALL_DIR = /home/etk/impinj_etk
INSTALL_TOOL_HELP = \
"Failed to find the arm-none-linux-gnueabi-g++ compiler." \
"Please make sure that /home/etk/impinj_etk/arm-toolchain/bin is in your PATH."
INSTALL_ETK_HELP = "Please follow the ETK install instructions and make sure that" \
"/home/etk/impinj_etk is in your PATH, or that the ETK_INSTALL_DIR is defined."

# The cap_gen tool may be in the PATH, the install dir, or in the current dir.
CAP_GEN_EXE = cap_gen.sh
CAP_GEN_IN_PATH=$(shell which $(CAP_GEN_EXE))
CAP_GEN_CWD=$(shell ls ./$(CAP_GEN_EXE) 2>/dev/null)
CAP_GEN_DEFAULT=$(shell ls $(ETK_DEFAULT_INSTALL_DIR)/$(CAP_GEN_EXE) 2>/dev/null)
ifneq (,$(CAP_GEN_IN_PATH))
	CAP_GEN=$(CAP_GEN_IN_PATH)
else
	ifneq (,$(ETK_INSTALL_DIR))
		CAP_GEN=$(ETK_INSTALL_DIR)/$(CAP_GEN_EXE)
	else
		ifneq (,$(CAP_GEN_DEFAULT))
			CAP_GEN=$(CAP_GEN_DEFAULT)
		else
			ifneq (,$(CAP_GEN_CWD))
				CAP_GEN=$(CAP_GEN_CWD)
			endif
		endif
	endif
endif


all: x86 arm streamer_arm streamer

help:
	@echo Example use:
	@echo  ‘make arm’ to build the sample for on-reader use
	@echo  ‘make x86’ to build the sample for the (x86) host
	@echo  ‘make cap’ to build a CAP upgrade file

bin/rfid_reader_x86: $(SOURCES)
	mkdir -p ./bin
	g++ \
		-m32 -Wno-write-strings \
		-I$(HEADER_DIR) \
		$(SOURCES) \
		-L$(MPNJ_LIB_DIR) -lltkcpp_x86 -lltkcppimpinj_x86 -lxml2_x86 \
		-L$(LIB_DIR) -ldl -lssl -lcrypto \
		-o bin/rfid_reader_x86

x86: bin/rfid_reader_x86

bin/rfid_reader_arm: check_env $(SOURCES)
	mkdir -p ./bin
	arm-none-linux-gnueabi-g++ \
		-Wno-write-strings \
		-I$(HEADER_DIR) \
		$(SOURCES) \
		-L$(MPNJ_LIB_DIR) \
		-static -lltkcpp_atmel -lltkcppimpinj_atmel -lxml2_atmel \
		-lssl_atmel -lcrypto_atmel -ldl_atmel \
		-o bin/rfid_reader_arm
	arm-none-linux-gnueabi-strip bin/rfid_reader_arm

arm: bin/rfid_reader_arm

target/armv5te-unknown-linux-gnueabi/release/streamer:
	cargo build --target=armv5te-unknown-linux-gnueabi --release
	arm-none-linux-gnueabi-strip target/armv5te-unknown-linux-gnueabi/release/streamer

streamer_arm: target/armv5te-unknown-linux-gnueabi/release/streamer

cap: arm check_env streamer_arm
	cp bin/rfid_reader_arm cap/rfid_reader
	cp target/armv5te-unknown-linux-gnueabi/release/streamer cap/streamer

	$(CAP_GEN) -d cap_description.in -o rfid_streamer.upg

clean:
	rm -rf bin/*
	rm -rf *.upg
	cargo clean

.PHONY: check_env
check_env:
	@if ! which $(CAP_GEN_EXE) > /dev/null && \
		[ ! -f $(ETK_INSTALL_DIR)/$(CAP_GEN_EXE) > /dev/null ] && \
		[ ! -f ./$(CAP_GEN_EXE) > /dev/null ]; then \
		echo "Failed to find $(CAP_GEN_EXE)."; \
		echo $(INSTALL_ETK_HELP); \
		exit 1; \
	fi
	@if ! which arm-none-linux-gnueabi-g++ > /dev/null; then \
		echo $(INSTALL_TOOL_HELP); \
		exit 1; \
	fi
