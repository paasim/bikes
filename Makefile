ifneq (,$(wildcard ./.env))
	include .env
	export
endif

.env:
	cp deb/env .env

bikes.db:
	sqlx database create
	sqlx migrate run
	cargo sqlx prepare --sqlite-create-db-wal false

clean:
	rm -rf target

dev:
	cargo run

test:
	cargo test
