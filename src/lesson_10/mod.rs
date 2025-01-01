use log::info;

pub fn lesson_10_main() {
    info!("Lesson 10: start");
    pub_struct_example();
    info!("Lesson 10: end");
}

fn pub_struct_example() {
    mod my {
        pub struct OpenBox<T> {
            pub contents: T,
        }

        #[allow(dead_code)]
        pub struct ClosedBox<T> {
            contents: T,
        }

        impl<T> ClosedBox<T> {
            pub fn new(contents: T) -> ClosedBox<T> {
                ClosedBox { contents }
            }
        }
    }

    let open_box = my::OpenBox {
        contents: "public information",
    };
    println!("The open box contains: {}", open_box.contents);

    let _closed_box = my::ClosedBox::new("classified information");
}
