all:
	@cargo vendor --relative-path > config
	@grep "checksum.*mesalock" Cargo.lock | cut -d ' ' -f2,3 > README.txt
clean:
	@cargo clean
