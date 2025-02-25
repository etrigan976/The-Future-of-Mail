git-push:
	git add .
	git commit -m "$(msg) - $(shell date '+%Y-%m-%d %H:%M:%S')"
	git push
test:
	cargo test
run:
	cargo run --release
doc:
	cargo doc
	cargo doc --open