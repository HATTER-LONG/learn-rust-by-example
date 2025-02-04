use log::info;

pub fn lesson_15_main() {
    info!("Lesson 15: start");
    option_example();
    result_example();
    info!("Lesson 15: end");
}

fn option_example() {
    {
        fn give_commoner(gift: Option<&str>) {
            match gift {
                Some("snake") => println!("Yuck! I'm throwing that snake in a fire."),
                Some(inner) => println!("{}? How nice.", inner),
                None => println!("No gift? Oh well."),
            }
        }

        fn give_princess(gift: Option<&str>) {
            let inside = gift.unwrap();
            if inside == "snake" {
                panic!("AAAaaaaa!!!!");
            }

            println!("I love {}s!!!!!", inside);
        }

        let food = Some("cabbage");
        let snake = Some("snake");
        let void = None;

        give_commoner(food);
        give_commoner(snake);
        give_commoner(void);

        let bird = Some("robin");
        // let nothing = None;

        give_princess(bird);
        // give_princess(nothing);
    }
    {
        struct Person {
            job: Option<Job>,
        }

        #[derive(Clone, Copy)]
        struct Job {
            phone_number: Option<PhoneNumber>,
        }

        #[derive(Clone, Copy)]
        struct PhoneNumber {
            area_code: Option<u8>,
            number: u32,
        }

        impl Person {
            fn work_phone_area_code(&self) -> Option<u8> {
                self.job?.phone_number?.area_code
            }
        }

        let p = Person {
            job: Some(Job {
                phone_number: Some(PhoneNumber {
                    area_code: Some(61),
                    number: 439222222,
                }),
            }),
        };

        assert_eq!(p.work_phone_area_code(), Some(61));
    }
    {
        #![allow(dead_code)]
        #[derive(Debug)]
        enum Food {
            Apple,
            Carrot,
            Potato,
        }

        #[derive(Debug)]
        struct Peeled(Food);
        #[derive(Debug)]
        struct Chopped(Food);
        #[derive(Debug)]
        struct Cooked(Food);

        fn peel(food: Option<Food>) -> Option<Peeled> {
            match food {
                Some(f) => Some(Peeled(f)),
                None => None,
            }
        }

        fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
            match peeled {
                Some(Peeled(f)) => Some(Chopped(f)),
                None => None,
            }
        }

        fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
            chopped.map(|Chopped(f)| Cooked(f))
        }

        fn process(food: Option<Food>) -> Option<Cooked> {
            food.map(|f| Peeled(f))
                .map(|Peeled(f)| Chopped(f))
                .map(|Chopped(f)| Cooked(f))
        }

        fn eat(food: Option<Cooked>) {
            match food {
                Some(f) => println!("Mmm. I love {:?}", f),
                None => println!("Oh no! It wasn't edible."),
            }
        }

        let apple = Some(Food::Apple);
        let carrot = Some(Food::Carrot);
        let potato = None;

        let cooked_apple = cook(chop(peel(apple)));
        let cooked_carrot = process(carrot);
        let cooked_potato = process(potato);

        eat(cooked_apple);
        eat(cooked_carrot);
        eat(cooked_potato);
    }
    {
        #![allow(dead_code)]
        #[derive(Debug)]
        enum Food {
            CordonBleu,
            Steak,
            Sushi,
        }
        #[derive(Debug)]
        enum Day {
            Monday,
            Tuesday,
            Wednesday,
        }

        fn have_ingredients(food: Food) -> Option<Food> {
            match food {
                Food::Sushi => None,
                _ => Some(food),
            }
        }

        fn have_recipe(food: Food) -> Option<Food> {
            match food {
                Food::CordonBleu => None,
                _ => Some(food),
            }
        }

        fn cookable_v1(food: Food) -> Option<Food> {
            match have_ingredients(food) {
                None => None,
                Some(food) => match have_recipe(food) {
                    None => None,
                    Some(food) => Some(food),
                },
            }
        }

        fn cookable_v2(food: Food) -> Option<Food> {
            have_ingredients(food).and_then(have_recipe)
        }

        fn eat(food: Food, day: Day) {
            match cookable_v2(food) {
                Some(food) => println!("Yay! On {:?} I get to eat {:?}", day, food),
                None => println!("Oh no. I don't get to eat on {:?}", day),
            }
        }

        let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);
        eat(cordon_bleu, Day::Monday);
        eat(steak, Day::Tuesday);
        eat(sushi, Day::Wednesday);
    }
}

fn result_example() {
    {
        fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
            let first_number = first_number_str.parse::<i32>().unwrap();
            let second_number = second_number_str.parse::<i32>().unwrap();
            first_number * second_number
        }

        let twenty = multiply("10", "2");
        println!("double is {}", twenty);

        // let tt = multiply("t", "2");
        // println!("double is {}", tt);
    }
    {
        fn multiply_v1(
            first_number_str: &str,
            second_number_str: &str,
        ) -> Result<i32, std::num::ParseIntError> {
            let first_number = first_number_str.parse::<i32>()?;
            let second_number = second_number_str.parse::<i32>()?;
            Ok(first_number * second_number)
        }

        fn print(result: Result<i32, std::num::ParseIntError>) {
            match result {
                Ok(n) => println!("n is {}", n),
                Err(e) => println!("Error: {:?}", e),
            }
        }

        print(multiply_v1("10", "2"));
        print(multiply_v1("t", "2"));
    }
}
