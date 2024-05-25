fn main() {
    패턴이_사용될_수_있는_모든_곳()
}

fn 패턴이_사용될_수_있는_모든_곳() {
    // 1. match 갈래

    // match 표현식의 값에 대한 모든 경우의 수를 고려해야 한다는 의미에서 철저해야 한다는 특징
    // 모든 가능성을 포괄하는 것을 보장하는 방법의 하나는 마지막 갈래에 캐치올 패턴을 사용하는 것

    let 숫자 = Some(3);

    match 숫자 {
        Some(3) => println!("3"),
        _ => (),
    }

    // 2. if let 조건 표현식

    // 컴파일러가 해당 구문이 모든 경우를 빠짐없이 포괄하는지 검사하지 않는다는 특징
    // else절을 생략하여 처리되지 않는 경우가 생기더라도
    // 컴파일러는 이에 따라 발생할 수 있는 논리적 버그를 경고해 주지 않는다

    if let Some(3) = 숫자 {
        println!("3");
    }

    // 3. while let 조건 루프

    // while let 조건 루프는 패턴이 계속 매칭되는 동안 while 루프를 실행할 수 있음

    let mut stack = Vec::new();

    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // 4. for 루프
    let v = vec![3];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // 5. let 구문

    // 알고보면 let 구문을 사용할 때마다 여러분은 패턴을 사용한 것과 같다.

    let x = 5;
    let (x, y, z) = (1, 2, 3);

    // 6. 함수 매개변수

    fn foo(x: i32) {}
}
