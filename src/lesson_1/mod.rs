use log::info;
use std::fmt;
// This is a line comment

/*
* This is a block comment that can span multiple lines
*/

// If you want to use this module in main.rs or other files,
// you need to add the following lines in main.rs:
//
// pub mod lesson_1;
// use lesson_1::lesson_1_main;
//
// 1. `pub` means it's a public module,
//    so it can be used in other files, not only in main.rs.
// 2. `mod` means it's a module.
// 3. `lesson_1` is the name of the module.
// 4. `use` means you want to use a function from a module.
// 5. `lesson_1::lesson_1_main` means you want to use the `lesson_1_main`
//    function from the `lesson_1` module.
// 6. If you want to use this function in other.rs file,
//    you need to add these lines too.

// `pub` means this function is public and can be used in other files
pub fn lesson_1_main() {
    info!("Lesson 1: start");
    hello_world();
    format_print();
    debug_format_print();
    display_format_print();
    display_vec_format_print();
    info!("Lesson 1: end");
}

// Create a function to print "Hello, World!" to the console
fn hello_world() {
    println!("Hello, World!");
}

fn format_print() {
    // {} will be replaced by the value of the variable
    println!("{} days", 31);

    // You can use positional arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // You can use named arguments
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // You can use special formatting after a `:`
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );

    // You can right-align text with a specified width
    println!("number: {number:>width$}", number = 1, width = 6);
    // You can pad numbers with extra zeroes
    println!("number: {number:>0width$}", number = 1, width = 6);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Create a structure named `Structure` which contains an `i32`.
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated handling.
    // This will not work.
    //println!("This struct `{}` won't print...", Structure(3));
}

fn debug_format_print() {
    // `Structure` is a structure which contains an `i32`.
    #[derive(Debug)]
    struct Structure(i32);

    // Put a `Structure` inside of the structure `Deep`.
    #[derive(Debug)]
    struct Deep(Structure);

    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how the results look.
    // What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

    // Pretty print
    println!("{:#?}", Deep(Structure(7)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
}

fn display_format_print() {
    // Define a structure named `Structure` which contains an `i32`.
    struct Structure(i32);

    // Implement `Display` for `Structure`.
    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Write strictly the first element into the supplied output
            // stream: `f`. Returns `fmt::Result` which indicates whether the
            // operation succeeded or failed. Note that `write!` uses syntax which
            // is very similar to `println!`.
            write!(f, "{}", self.0)
        }
    }

    println!("This struct `{}` will print...", Structure(3));

    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Display: {}", point);
    println!("Debug: {:?}", point);
}

fn display_vec_format_print() {
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;
            for (count, v) in vec.iter().enumerate() {
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", v)?;
            }
            write!(f, "]")
        }
    }

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
