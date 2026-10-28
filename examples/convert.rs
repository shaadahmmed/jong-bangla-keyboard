use jong_bangla_core::transliterate;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 1 {
        eprintln!("Usage: cargo run --example convert -- 'phonetic input'");
        std::process::exit(2);
    }
    println!("{}", transliterate(&args[0]));
}
