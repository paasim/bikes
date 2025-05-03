fn main() {
    if let Err(e) = bikes::AppConf::from_env().and_then(bikes::run) {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
