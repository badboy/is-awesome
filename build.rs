use std::env;

use glean_build::Builder;

fn main() {
    if env::var("CARGO_FEATURE_GECKO").is_err() {
        Builder::default()
            .file("metrics.yaml")
            .generate()
            .expect("Error generating Glean Rust bindings");
    }
}
