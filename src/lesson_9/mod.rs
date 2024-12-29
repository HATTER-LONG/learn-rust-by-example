use log::info;

pub fn lesson_9_main() {
    info!("Lesson 9: start");
    example_function();
    method_example();
    lambda_example();
    capture_example();
    closure_as_input_parameter();
    call_function_with_closure();
    iterator_any_example();
    higher_order_function();
    info!("Lesson 9: end");
}

fn example_function() {
    fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
        if rhs == 0 {
            return false;
        }
        lhs % rhs == 0
    }

    fn fizzbuzz(n: u32) -> () {
        if is_divisible_by(n, 15) {
            println!("fizzbuzz");
        } else if is_divisible_by(n, 3) {
            println!("fizz");
        } else if is_divisible_by(n, 5) {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    fn fizzbuzz_to(n: u32) {
        for i in 1..=n {
            fizzbuzz(i);
        }
    }

    fizzbuzz_to(10);
}

fn method_example() {
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }

        fn new(x: f64, y: f64) -> Point {
            Point { x, y }
        }
    }

    struct Rectangle {
        p1: Point,
        p2: Point,
    }

    impl Rectangle {
        fn area(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;
            ((x1 - x2) * (y1 - y2)).abs()
        }

        fn perimeter(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;
            2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
        }

        fn translate(&mut self, x: f64, y: f64) {
            self.p1.x += x;
            self.p2.x += x;
            self.p1.y += y;
            self.p2.y += y;
        }
    }

    struct Pair(Box<i32>, Box<i32>);
    impl Pair {
        fn destroy(self) {
            let Pair(first, second) = self;
            println!("Destroying Pair({}, {})", first, second);
            // first and second go out of scope and get freed
        }
    }

    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    square.translate(1.0, 1.0);
    println!("Square perimeter: {}", square.perimeter());

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
}

fn lambda_example() {
    let is_even = |x: u32| -> bool { x % 2 == 0 };
    println!("{}", is_even(5));
    println!("{}", is_even(6));

    let add_one = |x: u32| -> u32 { x + 1 };
    println!("{}", add_one(5));
    println!("{}", add_one(6));

    let add = |x, y| x + y;
    println!("{}", add(2, 3));
    println!("{}", add(4, 5));

    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;
    println!("Annotated closure: {}", closure_annotated(i));
    println!("Inferred closure: {}", closure_inferred(i));
}

fn capture_example() {
    use std::mem;

    let color = String::from("green");

    let print = || println!("`color`: {}", color);

    let _color_borrow = &color;
    print();
    let _color_moved = color;

    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();
    inc();
    println!("`count`: {}", count);

    let movable = Box::new(3);

    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    consume();
    //consume();
    //println!("`movable`: {:?}", movable);
}

fn closure_as_input_parameter() {
    fn apply<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }

    let x = 7;
    let print = || println!("{}", x);
    apply(print);

    fn apply_to_3<F>(f: F) -> i32
    where
        F: FnOnce(i32) -> i32,
    {
        f(3)
    }
    let mut data = "hello".to_owned();
    let double = |x| {
        data.push_str("world");
        println!("data: {}", data);
        std::mem::drop(data);
        x * 2
    };
    println!("3 doubled: {}", apply_to_3(double));
}

fn call_function_with_closure() {
    fn call_me<F: Fn()>(f: F) {
        f();
    }

    fn function() {
        println!("I'm a function!");
    }

    call_me(function);
}

fn iterator_any_example() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let vec2 = vec![4, 5, 6];
    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));
}

fn higher_order_function() {
    let upper = 1000;

    let mut acc = 0;
    for n in 0.. {
        let n_squared = n * n;

        if n_squared >= upper {
            break;
        } else if n_squared % 2 == 1 {
            acc += n_squared;
        }
    }
    println!("implemented: {}", acc);

    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n_squared| n_squared < upper)
        .filter(|&n_squared| n_squared % 2 == 1)
        .fold(0, |acc, n_squared| acc + n_squared);
    println!("functional: {}", sum_of_squared_odd_numbers);
}
