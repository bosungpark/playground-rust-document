fn main() {
    모호성_방지를_위한_완전_정규화_문법_같은_이름의_메서드_호출하기()
}

fn 연관_타입으로_트레이트_정의에서_플레이스_홀더_타입_지정하기() {
    // why?
    // 1. 사용 이유는 제네릭과 비슷하나, 트레이트에 대한 각각의 구현에 대해 타입 명시없이 사용하고 싶을때

    println!(
        "
    연관 타입은 타입 플레이스 홀더와 트레이트를 연결하여 트레이트 메서드 정의를 할 때 
    이러한 플레이스 홀더 타입을 시그니처에서 사용할 수 있도록 한다.

    트레이트의 구현자는 특정 구현을 위해서 플레이스 홀더 타입 대신 사용할 구체적인 타입을 지정한다.

    트레이트가 구현될 때까지 해당 타입이 무엇인지 정확히 알 필요 없이 
    임의의 타입을 사용하는 트레이트를 정의할 수 있다.
    "
    );

    // 예:
    // pub trait Iterator {
    //     type Item; // 플레이스 홀더

    //     // 정의에 플레이스 홀더 사용
    //     // 실제 구현시에 구체 타입을 지정한다
    //     // 제네릭과 유사해보이지만
    //     // 제네릭을 사용하면 각 구현에 대해 타입을 명시하고
    //     // 제네릭 타입 매개변수의 구체적인 타입을 변경하며 한 트레이트에 대해 구현을 여러번 할 수 있다는 차이가 있다
    //     fn next(&mut self) -> Option<Self::Item>;
    //     // ㄴ> 생각해보면 당연한것!
    // }
}

fn 기본_제네릭_타입_매개변수와_연산자_오버로딩() {
    // why?
    // 1. 기존 코드를 깨는 일 없이 타입을 확장하기 위해
    // 2. 대부분의 사용자가 필요로 하지 않는 특정 상황에 대한 커스터마이징을 허용하기 위해

    println!(
        "
        제네릭 타입 매개변수를 사용하면
        제네릭 타입에 대한 기본 구체적 타입을 지정할 수 있다.
        연산자 오버로딩 (operator overloading) 과 함께 쓰이는 경우 특히 유용하다.
        
        러스트에서는 자체 연산자를 만들거나 임의의 연산자를 오버로딩할 수 없다
        하지만 std::ops에 나열된 연산자와 연관된 트레이트를 구현하여 연산자 및 해당 트레이트를 오버로딩할 수 있다.
    "
    );

    // 1. +을 오버로딩하는 예시를 살펴보자!
    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        // ㄴ> Add 구현 예시:
        // ㄴ> pub trait Add<Rhs = Self> {
        // ㄴ> 기본 타입 매개변수가 Self로 이미 지정이 되어 있다!

        type Output = Point;

        // std::ops::Add 오버 로딩
        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    assert_eq!(
        // 오버로딩 된 + 사용 가능
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

fn 모호성_방지를_위한_완전_정규화_문법_같은_이름의_메서드_호출하기() {
    // why?
    // 1. 러스트에서는 메서드 명이 중복되거나 모호해질 수 있기 때문

    // 러스트는 어떤 트레이트가 다른 트레이트의 메서드와 같은 이름의 메서드가 있는 것을 막지 않는다.
    // 또 한 타입에 여러 트레이트를 구현하는 것도 가능하다. (메서드명의 중복이 생길 수 있음)
    // 또 트레이트의 메서드와 이름이 같은 메서드를 타입에 직접 정의해도 된다,,

    // 예시 1:

    // ㄴ> 메서드 이름이 지나칠 정도로 중첩되는 상황

    trait Pilot {
        // ?
        fn fly(&self);
    }

    trait Wizard {
        // ?
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        // ?
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        // ?
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        // ?
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    let person = Human;
    person.fly();
    // ㄴ> Human 에 대한 메서드 호출: *waving arms furiously*

    Pilot::fly(&person); // This is your captain speaking.
    Wizard::fly(&person); // Up!
                          // ㄴ> 더 명확한 문법 사용!

    // 예시 2:
    // 완전 정규화 문법 (fully qualified syntax) 을 사용해야 하는 이유

    // ㄴ> trait을 구현하여 puppy가 나오기를 기대하는 상황

    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    // 실패 예시:
    // println!("A baby dog is called a {}", Animal::baby_name());

    // cannot call associated function of trait
    // ㄴ> 직접 사용하려고 하면 문제가 생긴다!
    // ㄴ> 트레이드에 대한 구현체가 아니라 구조체에 대한 Animal 구현체를 사용하고 싶다는 것을 명확히하려면 완전 정규화 문법을 사용해야 한다.

    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    // A baby dog is called a puppy!
}

fn 슈퍼트레이트를_사용하여_한_트레이트에서_다른_트레이트의_기능을_요구하기() {
    // 트레이트 정의가 의존하고 있는 트레이트를 트레이트의 슈퍼트레이트 (supertrait) 이라고 한다.

    use std::fmt;

    // 예시: fmt::Display가 필요함을 명시
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    // Display를 구현하는 타입만 OutlinePrint를 구현하여 사용할 수 있음!

    // ㄴ> 즉! 쓰고 싶으면 걸려있는 모든 트레이트를 모두 구현해야 한다!
}

fn 뉴타입_패턴을_사용하여_외부_타입에_외부_트레이트_구현하기() {
    println!(
        "
        
        "
    )
}

