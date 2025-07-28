mod engine;

fn main() {
    let engine = engine::Engine::new("RustyCraft".to_string());
    engine.run().unwrap_or_else(|e| {
        eprintln!("Error running the engine: {}", e);
        std::process::exit(1);
    });
}