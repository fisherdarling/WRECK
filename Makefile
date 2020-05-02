build-debug:
	cargo build --bin WRECK;
	@cp ./target/debug/WRECK ./WRECK;
	@chmod +x ./WRECK
build:
	cargo build --release --bin WRECK;
	@cp ./target/release/WRECK ./WRECK;
	@chmod +x ./WRECK

clean_out:
	rm -f *.svg
	rm -f *.dot
	rm -f *.nfa
	rm -f test_out.txt
	rm -f *.tt
	rm -f *.lis
	rm -f *.err

clean: clean_out
	cargo clean
	rm -f WRECK
