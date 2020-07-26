

all:
	cargo build --release

install:
	cp target/release/zc /usr/bin

clean:
	rm -rf target

uninstall:
	rm /usr/bin/zc
