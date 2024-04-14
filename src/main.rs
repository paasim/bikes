mod conf;
mod db;
mod err;
mod page;
mod server;
mod station;
mod tile;

fn main() {
    conf::get_conf()
        .and_then(|(c, dtc)| server::run(c, dtc))
        .unwrap()
}
