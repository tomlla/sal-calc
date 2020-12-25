.DEFAULT_GOAL := help
RUST_BACKTRACE:=1

help:
	@grep -E '.*:' Makefile |grep -v ":=" |grep -P -v "^\t" | grep -v "^#"

serde_ex:
	cargo run --bin serde_ex

serde_ex:
	cargo run --bin serde_ex

# # date_fmt_ex: export RUST_BACKTRACE=1
# date_fmt_ex:
# 	RUST_BACKTRACE=1 cargo run --bin date_fmt_ex

date_fmt_ex:
	RUST_BACKTRACE=$(RUST_BACKTRACE) cargo run --bin date_fmt_ex

doc/open:
	cargo doc --open
