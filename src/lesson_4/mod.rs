use log::info;

pub fn lesson_4_main() {
    info!("Lesson 4: start");
    ver_example();
    info!("Lesson 4: end");
}

#[allow(dead_code, unused_variables, unused_assignments)]
fn ver_example() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    let noisy_unused_variable = 2u32;
}
