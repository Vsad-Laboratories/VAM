fn main() {
    if let Err(e) = vam::run() {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
