mod parse_rc;
use parse_rc::LshConfig;
fn main() {
    let config = LshConfig::load_or_default();

    println!("Input directory: {}", config.input);
    println!("Output directory: {}", config.output);
    println!("Include types: {}", config.include_types);
    println!("Mode: {}", config.mode);

    if config.include_types {
        println!("🔎 Including type generation...");
    };
}