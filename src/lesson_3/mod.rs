use log::info;

pub fn lesson_3_main() {
    info!("Lesson 3: start");
    struct_enum_example();
    enum_example();
    list_example();
    const_example();
    info!("Lesson 3: end");
}

#[allow(dead_code, unused_variables, unused_assignments)]
fn struct_enum_example() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    // A unit struct
    struct Unit;

    // A tuple struct
    struct Pair(i32, f32);

    // A struct with two fields
    #[derive(Debug)]
    struct Point {
        x: f32,
        y: f32,
    }

    // Structs can be reused as fields of another struct
    #[allow(dead_code)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Destructure the point using a `let` binding
    let bottom_right = Point { x: 5.2, ..point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point { x: my_x, y: my_y } = point;
    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: my_x, y: my_y },
        bottom_right: point,
    };
}

#[allow(dead_code, unused_variables, unused_assignments)]
fn enum_example() {
    enum WebEvent {
        // An `enum` may either be `unit-like`
        PageLoad,
        PageUnload,
        // like tuple structs
        KeyPress(char),
        Paste(String),
        // or c-like structures
        Click { x: i64, y: i64 },
    }

    // A function which takes a `WebEvent` enum as an argument and returns nothing
    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("page loaded"),
            WebEvent::PageUnload => println!("page unloaded"),
            // Destructure `c` from inside the `enum`.
            WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
            WebEvent::Paste(s) => println!("pasted \"{}\".", s),
            // Destructure `Click` into `x` and `y`.
            WebEvent::Click { x, y } => {
                println!("clicked at x={}, y={}.", x, y);
            }
        }
    }

    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }

    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

    impl Operations {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::Add => x + y,
                Self::Subtract => x - y,
            }
        }
    }
}

fn list_example() {
    enum List {
        Cons(u32, Box<List>),
        Nil,
    }

    impl List {
        fn new() -> List {
            Self::Nil
        }

        fn prepend(self, elem: u32) -> List {
            Self::Cons(elem, Box::new(self))
        }

        fn len(&self) -> u32 {
            match self {
                Self::Cons(_, tail) => 1 + tail.len(),
                Self::Nil => 0,
            }
        }

        fn stringify(&self) -> String {
            match self {
                Self::Cons(head, tail) => {
                    format!("{}, {}", head, tail.stringify())
                }
                Self::Nil => {
                    format!("Nil")
                }
            }
        }
    }

    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

fn const_example() {
    static LANGUAGE: &'static str = "Rust";
    const THRESHOLD: i32 = 10;

    fn is_big(n: i32) -> bool {
        // Access constant in some function
        n > THRESHOLD
    }

    let n = 16;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    //THRESHOLD = 5;
}
