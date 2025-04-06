mod conf;
mod err;
mod page;
mod server;
mod station;
mod tile;

fn main() {
    if let Err(e) = conf::AppConf::from_env().and_then(server::run) {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
