build: clean
	cargo build; cp target/debug/rvcc rvcc

test: build
	RISCV=/home/codespace/riscv ./test.sh

clean:
	cargo clean; rm rvcc tmp *.s | true

.PHONY: build test clean
