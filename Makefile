.PHONY: debug_nostd debug_std release_nostd release_default
# bellow targets application crash or runs indefinately
debug_nostd:
	cargo build --no-default-features
	./target/debug/check_decaf

release_nostd:
	cargo build --release --no-default-features
	./target/release/check_decaf

# Below compilation modes(release) application works just fine

debug_std:
	cargo build
	./target/debug/check_decaf


release_std:
	cargo build --release
	./target/release/check_decaf
