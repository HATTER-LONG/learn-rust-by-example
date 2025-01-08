use log::info;

pub fn lesson_12_main() {
    info!("Lesson 12: start");
    move_and_borrow_example();
    borrow_example();
    lifetime_example();
    info!("Lesson 12: end");
}

fn move_and_borrow_example() {
    fn destroy_box(c: Box<i32>) {
        println!("Destroying a box that contains {}", c);
    }

    let x = 5u32;
    let y = x;

    println!("x is {}, and y is {}", x, y);

    let a = Box::new(5i32);
    println!("a contains: {}", a);

    let b = a;
    // println!("a contains: {}", a);

    destroy_box(b);

    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // *immutable_box = 4;
    let mut mutable_box = immutable_box;
    *mutable_box = 4;

    //*immutable_box = 4;
    println!("mutable_box contains {}", mutable_box);

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("before person is {:?}", person);
    let Person { name, ref age } = person;

    println!("{} is {} years old.", name, age);

    //println!("after person is {:?}", person);
    println!("after person is {:?}", person.age);
}

fn borrow_example() {
    {
        fn eat_box_i32(boxed_i32: Box<i32>) {
            println!("Destroying box that contains {}", boxed_i32);
        }

        fn borrow_i32(borrowed_i32: &i32) {
            println!("This int is: {}", borrowed_i32);
        }

        let boxed_i32 = Box::new(5i32);
        let stacked_i32 = 6i32;

        borrow_i32(&boxed_i32);
        borrow_i32(&stacked_i32);

        {
            let _ref_to_i32: &i32 = &boxed_i32;

            //eat_box_i32(boxed_i32);

            borrow_i32(_ref_to_i32);
        }
        eat_box_i32(boxed_i32);
    }

    {
        #[allow(dead_code)]
        #[derive(Clone, Copy)]
        struct Book {
            author: &'static str,
            title: &'static str,
            year: u32,
        }

        fn borrow_book(book: &Book) {
            println!(
                "I immutably borrowed {} - {} edition",
                book.title, book.year
            );
        }

        fn new_edition(book: &mut Book) {
            book.year = 2014;
            println!("I mutably borrowed {} - {} edition", book.title, book.year);
        }

        let immutabook = Book {
            author: "Douglas Hofstadter",
            title: "Gödel, Escher, Bach",
            year: 1979,
        };

        let mut mutabook = immutabook;

        borrow_book(&immutabook);
        borrow_book(&mutabook);

        new_edition(&mut mutabook);
        //new_edition(&mut immutabook);
    }
    {
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let mut point = Point { x: 0, y: 0, z: 0 };

        let borrowed_point = &point;
        let another_borrow = &point;

        println!(
            "Point has coordinates: ({}, {}, {})",
            borrowed_point.x, another_borrow.y, point.z
        );

        //let mutable_borrow = &mut point;

        println!(
            "Point has coordinates: ({}, {}, {})",
            borrowed_point.x, another_borrow.y, point.z
        );

        let mutable_borrow = &mut point;

        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;

        // let y = &point.y;
        // println!("Point Z coordinate is {}", point.z);

        println!(
            "Point has coordinates: ({}, {}, {})",
            mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
        );
        let new_borrowed_point = &point;
        println!(
            "Point now has coordinates: ({}, {}, {})",
            new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
        );
    }
    {
        #[derive(Clone, Copy)]
        struct Point {
            x: i32,
            y: i32,
        }

        let c = 'Q';
        let ref rec_c1 = c;
        let ref_c2 = &c;

        println!("rec_c1 equals rec_c2: {}", *rec_c1 == *ref_c2);
        println!("rec_c1 equals rec_c2: {}", rec_c1 == ref_c2);

        let point = Point { x: 0, y: 0 };

        let _copy_of_x = {
            let Point {
                x: ref ref_to_x,
                y: _,
            } = point;
            *ref_to_x
        };

        let mut mutable_point = point;

        {
            let Point {
                x: _,
                y: ref mut mut_ref_to_y,
            } = mutable_point;

            *mut_ref_to_y = 1;
        }

        println!("point is {}", point.x);
        println!("mutable_point is {}", mutable_point.x);

        let mut mutable_tuple = (Box::new(5u32), 3u32);
        {
            let (_, ref mut last) = mutable_tuple;
            *last = 2u32;
        }
    }
    {
        fn longest(x: &str, y: &str) -> String {
            if x.len() > y.len() {
                x.to_string()
            } else {
                y.to_string()
            }
        }

        let string1 = longest("abcd", "bcde");
        println!("string1 address is {:p}", &string1);
    }
}

fn lifetime_example() {
    {
        fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
            println!("x is {} and y is {}", x, y);
        }

        fn failed_borrow<'a>() {
            let _x = 12;
            //let y: &'a i32 = &_x;
            //println!("x is {}", y);
        }

        fn success_borrow<'a>(x: &'a Box<i32>) -> &'a Box<i32> {
            x
        }

        let (four, nine) = (4, 9);

        print_refs(&four, &nine);
        failed_borrow();

        let x: &Box<i32>;
        {
            let y = Box::new(5);
            x = success_borrow(&y);
            println!("x is {}", x);
        }
        // println!("x is {}", x);
    }
    #[allow(unused)]
    {
        #[derive(Debug)]
        struct Borrowed<'a>(&'a i32);

        #[derive(Debug)]
        struct NameBorrowed<'a> {
            x: &'a i32,
            y: &'a i32,
        }

        #[derive(Debug)]
        enum Either<'a> {
            Num(i32),
            Ref(&'a i32),
        }

        let x = 18;
        let y = 15;

        let single = Borrowed(&x);
        let double = NameBorrowed { x: &x, y: &y };
        let reference = Either::Ref(&x);
        let number = Either::Num(y);

        println!("single is {:?}", single);
        println!("double is {:?}", double);
        println!("reference is {:?}", reference);
        println!("number is {:?}", number);
    }
    {
        #[derive(Debug)]
        #[allow(unused)]
        struct Borrowed<'a> {
            x: &'a i32,
        }

        impl<'a> Default for Borrowed<'a> {
            fn default() -> Self {
                Self { x: &10 }
            }
        }

        let b: Borrowed = Default::default();

        println!("b is {:?}", b);
    }
}
