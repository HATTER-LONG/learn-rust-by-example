use log::info;

pub fn lesson_11_main() {
    info!("Lesson 11: start");
    template_example();
    template_impl_example();
    template_trait_example();
    bound_example();
    multi_bound_example();
    where_bound_example();
    newtype_example();
    contains_trait_example();
    info!("Lesson 11: end");
}

fn template_example() {
    struct A;
    struct S(A);
    struct SGen<T>(T);

    fn reg_fn(_s: S) {}

    fn gen_spec_t(_s: SGen<A>) {}

    fn gen_spec_i32(_s: SGen<i32>) {}

    fn generic<T>(_s: SGen<T>) {}

    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));
    generic(SGen('c'));
}

#[allow(dead_code)]
fn template_impl_example() {
    struct S;
    struct GenericVal<T>(T);

    impl GenericVal<f32> {}
    impl GenericVal<S> {}

    impl<T> GenericVal<T> {}

    struct Val {
        val: f64,
    }

    struct GenVal<T> {
        gen_val: T,
    }

    impl Val {
        fn value(&self) -> &f64 {
            &self.val
        }
    }

    impl<T> GenVal<T> {
        fn value(&self) -> &T {
            &self.gen_val
        }
    }

    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("{}, {}", x.value(), y.value());
}

fn template_trait_example() {
    struct Empty;
    struct Null;

    trait DoubleDrop<T> {
        fn double_drop(self, _: T);
    }

    impl<T, U> DoubleDrop<T> for U {
        fn double_drop(self, _: T) {}
    }

    let empty = Empty;
    let null = Null;

    empty.double_drop(null);
}

#[allow(dead_code)]
fn bound_example() {
    use std::fmt::Display;
    fn printer<T: Display>(t: T) {
        println!("{}", t);
    }

    struct S<T: Display>(T);

    // let s = S(vec![1]);

    trait HasArea {
        fn area(&self) -> f64;
    }
    #[derive(Debug)]
    struct Rectangle {
        length: f64,
        height: f64,
    }

    struct Triangle {
        length: f64,
        height: f64,
    }

    impl HasArea for Rectangle {
        fn area(&self) -> f64 {
            self.length * self.height
        }
    }

    // impl HasArea for Triangle {
    //     fn area(&self) -> f64 {
    //         self.length * self.height / 2.0
    //     }
    // }

    fn print_area<T: HasArea>(t: &T) {
        println!("This shape has an area of {}", t.area());
    }

    use std::fmt::Debug;
    fn print_debug<T: Debug>(t: &T) {
        println!("{:?}", t);
    }

    {
        let rectangle = Rectangle {
            length: 3.0,
            height: 4.0,
        };
        let _triangle = Triangle {
            length: 3.0,
            height: 4.0,
        };

        print_area(&rectangle);
        // print_area(_triangle);
        print_debug(&rectangle);
        //print_debug(_triangle);
    }

    struct Cardinal;
    struct BlueJay;
    struct Turkey;

    trait Red {}
    trait Blue {}

    impl Red for Cardinal {}
    impl Blue for BlueJay {}

    fn red<T: Red>(_: &T) -> &'static str {
        "red"
    }

    fn blue<T: Blue>(_: &T) -> &'static str {
        "blue"
    }

    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;
    println!("{}", red(&cardinal));
    println!("{}", blue(&blue_jay));
    // println!("{}", red(&_turkey));
}

fn multi_bound_example() {
    use std::fmt::{Debug, Display};

    fn compare_prints<T: Debug + Display>(t: &T) {
        println!("Debug: `{:?}`", t);
        println!("Display: `{}`", t);
    }

    fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
        println!("t: `{:?}`", t);
        println!("u: `{:?}`", u);
    }

    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    //compare_prints(&array);
    compare_types(&array, &vec);
}

fn where_bound_example() {
    use std::fmt::Debug;

    trait PrintInOption {
        fn print_in_option(self);
    }

    impl<T> PrintInOption for T
    where
        Option<T>: Debug,
    {
        fn print_in_option(self) {
            println!("{:?}", Some(self));
        }
    }

    let vec = vec![1, 2, 3];
    vec.print_in_option();
}

fn newtype_example() {
    struct Years(i64);
    struct Days(i64);

    impl Years {
        pub fn to_days(&self) -> Days {
            Days(self.0 * 365)
        }
    }

    impl Days {
        pub fn to_years(&self) -> Years {
            Years(self.0 / 365)
        }
    }

    fn old_enough(age: &Years) -> bool {
        age.0 >= 18
    }

    let age = Years(5);
    let age_days = age.to_days();

    println!("Old enough: {}", old_enough(&age));
    println!("Old enough: {}", old_enough(&age_days.to_years()));
}

fn contains_trait_example() {
    struct Container(i32, i32);

    trait Contains<A, B> {
        fn contains(&self, _: &A, _: &B) -> bool;
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }

    impl Contains<i32, i32> for Container {
        fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
            (&self.0 == number_1) && (&self.1 == number_2)
        }

        fn first(&self) -> i32 {
            self.0
        }

        fn last(&self) -> i32 {
            self.1
        }
    }

    fn difference<A, B, C>(container: &C) -> i32
    where
        C: Contains<A, B>,
    {
        container.last() - container.first()
    }

    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} and {}: {}",
        &number_1,
        &number_2,
        container.contains(&number_1, &number_2)
    );
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}
