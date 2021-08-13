fn main() {
    if let Err(e) = ctloader::get_args().and_then(ctloader::run) {
        println!("{}", e);
        std::process::exit(1);
    }
}
