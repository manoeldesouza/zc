

all:
	cargo build --release

run:
	target/release/zc

install:
	cp target/release/zc /usr/bin

clean:
	rm -rf target

uninstall:
	rm /usr/bin/zc
