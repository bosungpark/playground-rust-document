fn 패턴_문법() {
    // 1. 리터럴 매칭
    // 코드에서 특정한 구체적인 값을 가질 때 어떤 동작을 수행하려는 목적

    let x = 1;

    match x {
        1 => println!("one"),
        _ => println!("anything"),
    }

    // 2. 명명된 변수 매칭

    // match의 스코프를 이해하지 못한다면 약간은 복잡한 상황이 생길 수 있다
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // match 표현식 내부의 새로운 스코프에 있기 때문에, 이것은 처음에 10이라는 값으로 선언한 y가 아니라 새로운 y 변수
        // Some 내부의 모든 값에 매칭되고, x 안의 Some 내부 값에 바인딩
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
    // ㄴ> at the end: x = Some(5), y = 10

    // 3. 다중 패턴
    // (or) 연산자를 사용해 여러 패턴 매칭
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    // ㄴ> one or two

    // 4. ..=를 이용한 값의 범위 매칭

    // ..= 문법은 경계 값을 포함하는 범위와 매칭
    //  러스트가 범위가 비어 있는지를 알 수 있는 유일한 타입은 char와 숫자 값이므로,
    // 범위는 숫자 또는 char 값으로만 허용
    let x = 3;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    // ㄴ> one through five
}

fn 값을_해체하여_분리하기() {
    // 구조체, 열거형, 튜플을 분해하여 이 값의 부분들을 쓰기 위해 패턴을 사용할 수도 있음

    // 1. 구조체 해체하기
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    // 이런식으로 할당이 가능하다,,,
    let Point { x: a, y: b } = p;

    assert_eq!(0, a);
    assert_eq!(7, b);

    // 이런식도 가능하다,,,
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // 위의 패턴 매칭을 응용하면 아래와 같이 사용할 수 있다!
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
    // ㄴ> On the y axis at 7

    // 2. 열거형 해체하기
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            // 데이터가 없으므로 해체 불가능
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            // r g b 해체!
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }

    // 3. 구조체와 튜플 해체하기

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // ㄴ> 개인적으로 이 정도까지 오면 뇌절인듯,,
    // ㄴ> 그냥 이런 것도 가능하다,,,정도로 생각하면 될 듯,,,
}

fn 패턴에서_값_무시하기() {
    // 1. _로 전체 값 무시하기

    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    foo(3, 4);

    // 2. 중첩된 _로 값의 일부 무시하기
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        // 일부 무시
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    // 3. _로 시작하는 이름으로 사용하지 않는 변수 무시하기

    // 변수를 선언하고 사용하지 않는 경우, 사용하지 않는 변수는 버그가 될 수 있으므로 러스트는 보통 경고를 표시
    // 프로토타이핑 중인 경우 _ 를 사용해 경고를 무시 가능

    let _x = 5;
    // ㄴ> _x는 여전히 변수에 값을 바인딩하는 반면, _는 전혀 바인딩하지 않는다

    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
    // ㄴ> Some("Hello!")
    // ㄴ> _는 바인딩이 되지 않으므로 문제 없다!

    // 4. ..로 값의 나머지 부분 무시하기

    // 여러 부분으로 구성된 값의 경우, .. 문법을 사용하여 특정 부분만 사용하고 나머지는 무시할 수 있다.
    // 구조체를 예로 들면, 필드마다 _를 적어줄 필요가 없다!
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
    // 튜플은 아래와 같이 사용한다.
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}

fn 매치_가드를_사용한_추가_조건() {
    // 매치 가드는 패턴만 가지고는 할 수 없는 더 복잡한 아이디어를 표현할 때 유용

    let num = Some(4);

    match num {
        // 조건 1: Some(x), 조건 2: if x % 2 == 0 (매치가드)
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }
}

fn at_연산자_바인딩() {
    // @ 바인딩
    // @을 사용하면 값에 대한 패턴 매칭 여부를 테스트하는 동시에 해당 값을 갖는 변수를 만들 수 있다.

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            // 변수에 바인딩!
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
    // ㄴ> Found an id in range: 5
}

// 전반적으로 직관적이고 좋은듯..!
