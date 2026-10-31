all: build-modules

build-modules: rust-build

rust-build:
	mkdir -p modules
	$(CARGO) build --release --manifest-path "$(top_srcdir)/../Cargo.toml" --target-dir "$(top_srcdir)/../target"
	@os=`uname -s`; \
	if [ "$$os" = Darwin ]; then src="$(top_srcdir)/../target/release/libammonia.dylib"; \
	else src="$(top_srcdir)/../target/release/libammonia.so"; fi; \
	test -f "$$src"; \
	cp -f "$$src" modules/ammonia.so

install-modules:
	$(mkinstalldirs) $(INSTALL_ROOT)$(EXTENSION_DIR)
	$(INSTALL) modules/ammonia.so $(INSTALL_ROOT)$(EXTENSION_DIR)/ammonia.so

install: install-modules
