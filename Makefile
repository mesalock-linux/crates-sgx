all:
	@cargo-vendor vendor --relative-path > config
	@grep -E "checksum.*mesalock|checksum.*crates" Cargo.lock | cut -d ' ' -f2,3 | column -t > README.txt
test:
	@cargo-vendor vendor > config
	@grep -E "checksum.*mesalock|checksum.*crates" Cargo.lock | cut -d ' ' -f2,3 > README.txt
clean:
	@cargo-vendor clean
