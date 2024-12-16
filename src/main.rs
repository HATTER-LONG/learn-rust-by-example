mod lesson_1;
mod lesson_2;
mod lesson_3;
mod lesson_4;
mod lesson_5;
mod lesson_6;

use env_logger;
use lesson_1::lesson_1_main;
use lesson_2::lesson_2_main;
use lesson_3::lesson_3_main;
use lesson_4::lesson_4_main;
use lesson_5::lesson_5_main;
use lesson_6::lesson_6_main;
use std::env;
fn main() {
    env::set_var("RUST_LOG", "trace");
    env_logger::init();
    lesson_1_main();
    lesson_2_main();
    lesson_3_main();
    lesson_4_main();
    lesson_5_main();
    lesson_6_main();
}
