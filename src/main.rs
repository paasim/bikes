mod conf;
mod db;
mod err;
mod html_template;
mod page;
mod server;
mod station;
mod tile;

fn main() {
    conf::get_conf()
        .and_then(|(c, dtc)| server::run(c, dtc))
        .unwrap()
}
