fn main() {
    let _v = 구문과_표현식();
    println!("{_v}")
}

// 1. 구문과 표현식
// 구문: 어떤 동작을 수행하고 "값을 반환하지 않는 명령"
// 표현식: 결괏값을 평가
fn 구문과_표현식() -> i32 {
    // let _x = (let _y = 6);  // error: expected expression, found `let` statement

    let _y = {
        let _x = 3;
        _x + 1
    };

    _y  // 세미콜론 없음 == 표현식 == 값을 반환함
}
