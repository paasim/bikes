# bikes
[![build](https://github.com/paasim/bikes/workflows/build/badge.svg)](https://github.com/paasim/bikes/actions)

A simple webapp that shows nearby citybike stations. In addition, a few preset groups can be added to the database.

## install

The [release builds](https://github.com/paasim/bikes/releases) contain a debian package. `man bikes` may or may not be helpful.

## development

Dev server can be started with `make run`. It initializes the database, relevant environment variables and starts the server.

For development purposes, [`sqlx-cli`](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md), which is needed for db and query metadata initialization. It can be installed with `cargo binstall sqlx-cli`.
