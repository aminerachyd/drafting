install:
	cargo build --release
	sudo mv target/release/drafting /usr/local/bin
