all: check clippy test

check:
	cargo check

clippy:
	cargo clippy --all --all-targets -- -Dwarnings -Drust-2018-idioms

watch:
	cargo watch -x check -x 'clippy --all --all-targets -- -Dwarnings -Drust-2018-idioms' -x test
