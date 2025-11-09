fn main() {
    let name = env!("CARGO_PKG_NAME");
    let author = env!("CARGO_PKG_AUTHORS");
    let version = env!("CARGO_PKG_VERSION");

    println!(
        "{} v{} by {}",
        name,
        version,
        author
    )
}
