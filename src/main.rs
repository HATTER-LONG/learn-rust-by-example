mod lesson_1;

use lesson_1::lesson_1_main;
use env_logger;
use std::env;
fn main() {
    env::set_var("RUST_LOG", "trace");
    env_logger::init();
    lesson_1_main();
}
