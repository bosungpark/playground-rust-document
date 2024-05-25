
fn 반박_가능성_패턴이_매칭에_실패할지의_여부() {
    // 1. 반박 가능한 패턴: 일부 가능한 값에 대해 매칭에 실패할 수 있는 패턴

    // if let과 while let 표현식은 반박 가능한 패턴을 허용한다.
    // 이 표현식들이 잠재적인 실패를 처리할 목적으로 만들어졌기 때문

    // 2. 반박 불가능한 패턴: 넘겨진 모든 가능한 값에 대해 매칭되는 패턴

    // 함수 매개변수, let 구문 및 for 루프에는 반박 불가능한 패턴만 허용된다.
    // 값이 매칭되지 않으면 프로그램이 의미 있는 작업을 수행할 수 없기 때문

    let 숫자 = Some(3);
    // let Some(x) = 숫자;  // non-exhaustive pattern: `None` not covered

    // 반박 불가능한 패턴이 필요한 곳에서 반박 가능한 패턴을 사용한 경우, 패턴을 사용하는 코드를 변경하여 문제를 해결할 수 있다.
    // 예: let -> if let

    if let Some(x) = 숫자 {
        println!("{}", x);
    }

    // 반대에도 동일한 논리로 굳이 반박 불가능한 경우에 반박 가능한 패턴에 매칭시킨다면 경고를 띄운다
    // this pattern will always match, so the `if let` is useless consider replacing the `if let` with a `let`
    if let x = 5 {
        println!("{}", x);
    };
}
