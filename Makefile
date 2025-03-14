.PHONY: check clean doc doc-std run test
DB=bikes.db

.env:
	cp deb/env .env
	sed 's/\/var\/lib\/bikes\///' deb/env > $@

include .env
export

check: .git/hooks/pre-commit
	. $<

clean:
	rm -rf $(DB) .env target

doc:
	cargo doc --open

doc-std:
	rustup doc --std

run: $(DB)
	cargo sqlx prepare
	cargo run

test:
	cargo test

$(DB):
	cargo sqlx database create && cargo sqlx migrate run

.git/hooks/pre-commit:
	curl -o $@ https://gist.githubusercontent.com/paasim/317a1fd91a6236ca36d1c1c00c2a02d5/raw/315eb5b4e242684d64deb07a0c1597057af29f90/rust-pre-commit.sh
	echo "" >> $@
	chmod +x $@
