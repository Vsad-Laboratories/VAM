fn main() {
    if let Err(e) = vam::run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
