use log::info;
pub fn lesson_5_main() {
    info!("Lesson 5: start");
    type_cast_example();
    type_value_example();
    type_auto_example();
    type_alais_example();
    info!("Lesson 5: end");
}

fn type_cast_example() {
    let decimal = 65.4321_f32;

    // Error! No implicit conversion
    // let integer: i32 = decimal;

    // Explicit conversion
    let integer = decimal as i32;
    let character = integer as u8;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // When casting any value to an unsigned type, T,
    // std::T::MAX + 1 is added or subtracted until the value
    // fits into the new type

    // 1000 already fits in a u16
    println!("1000 as a u16 is: {}", 1000 as u16);
}

#[allow(dead_code, unused_variables, unused_assignments)]
fn type_value_example() {
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}

fn type_auto_example() {
    let elem = 5u8;

    let mut vec = Vec::new();
    vec.push(elem);

    println!("{:?}", vec);
}

fn type_alais_example() {
    type NanoSecond = u64;
    type Inch = u64;

    // Use an attribute to silence warning.
    #[allow(non_camel_case_types)]
    type u64_t = u64;
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}
