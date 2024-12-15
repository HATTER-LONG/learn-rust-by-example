mod lesson_1;
mod lesson_2;
mod lesson_3;

use env_logger;
use lesson_1::lesson_1_main;
use lesson_2::lesson_2_main;
use lesson_3::lesson_3_main;
use std::env;
fn main() {
    env::set_var("RUST_LOG", "trace");
    env_logger::init();
    lesson_1_main();
    lesson_2_main();
    lesson_3_main();
}
