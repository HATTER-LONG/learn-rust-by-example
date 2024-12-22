use std::option;

use log::info;

pub fn lesson_8_main() {
    info!("Lesson 8: start");
    example_function();
    guard_example();
    if_let_example();
    info!("Lesson 8: end");
}

fn example_function() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }
    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;
    println!("a = {}, b = {}, y = {}", a, b, y);

    let Foo { x, .. } = foo;
    println!("x = {:?}", x);

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {}, y = {}", b, y),
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
        Foo { y, .. } => println!("y = {}", y),
    }
}

fn guard_example() {
    let pair = (2, -2);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }

    match 5 {
        e @ 1..=5 => println!("got a range element {}", e),
        _ => println!("anything"),
    }
}

fn if_let_example() {
    let option: Option<i32> = None;
    match option {
        Some(i) => {
            println!("This is a really long string and `i` is {}", i);
        }
        _ => {}
    }

    if let Some(i) = option {
        println!("This is a really long string and `i` is {}", i);
    } else {
        println!("This is a really long string and there is no `i`");
    }

    enum Foo {
        Bar,
        Baz,
        Qux(u32),
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);
    if let Foo::Bar = a {
        println!("a is foobar");
    }
    if let Foo::Bar = b {
        println!("b is foobar");
    }
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }
}
