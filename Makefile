PHP ?= php
CARGO ?= cargo

UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
EXT := target/release/libammonia.dylib
else ifneq ($(findstring MINGW,$(UNAME_S)),)
EXT := target/release/ammonia.dll
CARGO := cargo +nightly
else ifneq ($(findstring MSYS,$(UNAME_S)),)
EXT := target/release/ammonia.dll
CARGO := cargo +nightly
else
EXT := target/release/libammonia.so
endif

.PHONY: test bench
test:
	$(CARGO) test
	$(CARGO) build --release
	$(PHP) -d extension=$(CURDIR)/$(EXT) -d display_errors=1 tests/clean.php

bench:
	$(CARGO) build --release
	composer install --no-interaction --working-dir=bench
	$(PHP) -d extension=$(CURDIR)/$(EXT) -d display_errors=1 bench/bench.php
