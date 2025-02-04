use log::info;

pub fn lesson_14_main() {
    info!("Lesson 14: start");
    macro_example();
    info!("Lesson 14: end");
}

fn macro_example() {
    macro_rules! create_function {
        ($func_name:ident) => {
            fn $func_name() {
                println!("You called {:?}()", stringify!($func_name));
            }
        };
    }

    create_function!(foo);
    create_function!(bar);

    macro_rules! print_result {
        ($expression:expr) => {
            println!("{:?} = {:?}", stringify!($expression), $expression);
        };
    }

    foo();
    bar();

    print_result!(1u32 + 1);

    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });

    macro_rules! test {
        ($left:expr; and $right:expr) => {
            println!(
                "{:?} and {:?} is {:?}",
                stringify!($left),
                stringify!($right),
                $left && $right
            )
        };
        ($left:expr; or $right:expr) => {
            println!(
                "{:?} or {:?} is {:?}",
                stringify!($left),
                stringify!($right),
                $left || $right
            )
        };
    }

    test!(1 == 1; and 2 == 2);
    test!(1 == 1; or 2 == 2);

    macro_rules! find_min {
        ($x:expr) => ($x);
        ($x:expr, $($y:expr),+) => (
            std::cmp::min($x, find_min!($($y),+))
        );
    }

    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2, 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
}
