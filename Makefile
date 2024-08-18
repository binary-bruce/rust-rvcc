build: clean
	cargo build; cp target/debug/rvcc rvcc

test: build
	RISCV=/root/riscv ./test.sh

clean:
	cargo clean; rm rvcc *.s | true

.PHONY: build test clean
