# -*- mode: makefile-gmake -*-

TARGETS := access sharded ycsb

X86_64_DIR := x86_64
AARCH64_DIR := aarch64

CC_X86_64 := $(shell pwd)/cc_x86_64
CC_AARCH64 := $(shell pwd)/cc_aarch64


FLAGS :=
TARPATH := debug
ifdef RELEASE
	FLAGS += --release
	TARPATH := release
endif

ifdef COLLECT
	FLAGS += --features collect
endif

ifdef HUGEPAGE
	FLAGS += --features hugepage
endif

.PHONY: all
all: $(addsuffix _x86_64,$(TARGETS)) $(addsuffix _aarch64,$(TARGETS))
x86_64: $(addsuffix _x86_64,$(TARGETS))
aarch64: $(addsuffix _aarch64,$(TARGETS))

.PHONY: $(TARGETS)
$(TARGETS): %: %_x86_64 %_aarch64

.PHONY: %_x86_64
%_x86_64:
	${HOME}/.cargo/bin/cross build --target x86_64-unknown-linux-gnu --bench $(patsubst %_x86_64,%,$@) $(FLAGS)

.PHONY: %_aarch64
%_aarch64:
	${HOME}/.cargo/bin/cross build --target aarch64-unknown-linux-gnu --bench $(patsubst %_aarch64,%,$@) $(FLAGS)
