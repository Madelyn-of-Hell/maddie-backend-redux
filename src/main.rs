mod generator;

use generator::{compile_facade, compile_real_site};
use std::fs;

fn main() {
    fs::write("facade_progress.html", compile_facade()).unwrap();
    fs::write("real_progress.html", compile_real_site()).unwrap();
}
