# dmenu-rs - dynamic menu
# See LICENSE file for copyright and license details.

include config.mk

ifeq ($(XINERAMA),true)
	XINERAMA_FLAGS = --features "Xinerama"
endif

ifeq ($(CC),)
	CC = cc
endif

export RUSTFLAGS
export PLUGINS
export VERSION

all:	options dmenu stest

options:
	@echo "dmenu ($(VERSION)) build options:"
	@echo "CFLAGS     = $(CFLAGS)"
	@echo "CC         = $(CC)"
	@echo "RUSTFLAGS  = $(RUSTFLAGS)"
	@echo "PLUGINS    = $(PLUGINS)"

config:	scaffold
	cd src/config && cargo run --bin config

dmenu:	m4
	cd src/build && cargo build --release $(XINERAMA_FLAGS)
	cp src/build/target/release/dmenu target

man:	config
	man target/dmenu.1

test:	all
	seq 1 100 | target/dmenu $(ARGS)

plugins:
	cd src/config && cargo run --bin list-plugins

stest:
	mkdir -p target
	$(CC) $(CFLAGS) -o target/stest.o src/stest/stest.c
	$(CC) -o target/stest target/stest.o
	rm -f target/stest.o
	cp src/man/src/stest.1 target

scaffold:
	mkdir -p target
	mkdir -p target/build
	touch target/build/deps.toml

m4:	config
	m4 src/build/CargoSource.toml > src/build/Cargo.toml

clean:	scaffold m4
	cd src/build && cargo clean
	cd src/config && cargo clean
	rm -f vgcore* massif* src/build/Cargo.toml
	rm -rf target
	rm -rf dmenu-* # distribution files

dist:	
	mkdir -p dmenu-$(VERSION)
	cp -r LICENSE README.md makefile config.mk src dmenu-$(VERSION)
	tar -cf dmenu-$(VERSION).tar dmenu-$(VERSION)
	gzip dmenu-$(VERSION).tar
	rm -rf dmenu-$(VERSION)

# may need sudo
install:	all
	mkdir -p $(DESTDIR)$(PREFIX)/bin
	cp -f target/dmenu src/sh/dmenu_path src/sh/dmenu_run target/stest $(DESTDIR)$(PREFIX)/bin/
	chmod 755 $(DESTDIR)$(PREFIX)/bin/dmenu
	chmod 755 $(DESTDIR)$(PREFIX)/bin/dmenu_path
	chmod 755 $(DESTDIR)$(PREFIX)/bin/dmenu_run
	chmod 755 $(DESTDIR)$(PREFIX)/bin/stest
	mkdir -p $(DESTDIR)$(MANPREFIX)/man1
	cp target/dmenu.1 $(DESTDIR)$(MANPREFIX)/man1/dmenu.1
	sed "s/VERSION/$(VERSION)/g" < target/stest.1 > $(DESTDIR)$(MANPREFIX)/man1/stest.1
	chmod 644 $(DESTDIR)$(MANPREFIX)/man1/dmenu.1
	chmod 644 $(DESTDIR)$(MANPREFIX)/man1/stest.1

uninstall:
	rm -f $(DESTDIR)$(PREFIX)/bin/dmenu\
		$(DESTDIR)$(PREFIX)/bin/dmenu_path\
		$(DESTDIR)$(PREFIX)/bin/dmenu_run\
		$(DESTDIR)$(PREFIX)/bin/stest\
		$(DESTDIR)$(MANPREFIX)/man1/dmenu.1\
		$(DESTDIR)$(MANPREFIX)/man1/stest.1
