build:
	cargo build --release --bin WRECK;
	@cp ./target/release/WRECK ./WRECK;
	@chmod +x ./WRECK

clean:
	cargo clean
	rm *.m
	rm *.cmptt
	rm *.tt
	rm *.dat
	rm test_out.txt