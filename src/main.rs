mod conf;
mod err;
mod page;
mod server;
mod station;
mod tile;

fn main() {
    if let Err(e) = conf::get_conf().and_then(|(c, dtc)| server::run(c, dtc)) {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
