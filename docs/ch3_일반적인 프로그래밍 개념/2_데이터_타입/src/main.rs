fn main() {
    // 1. 스칼라 타입:
    // 스칼라 (scalar) 타입은 하나의 값을 표현
    // 정수, 부동 소수점 숫자, 부울린, 문자, 이렇게 네 가지 스칼라 타입 존재

    // 정수:
    // 각 부호 있는 타입의 변수는 -(2n - 1)부터 2n - 1 - 1까지의 값
    // 부호 없는 타입은 0에서 2n - 1까지의 값

    // 정수형 리터럴:
    // 대략 정수가 표현되는 방식 정도로만 이해하고 가도 좋을듯,,
    // 여러 숫자 타입이 될 수 있는 숫자 리터럴에는 타입 접미사를 사용하여 타입을 지정
    // _ 같은 리터럴은 가독성을 위해 사용되기도 함

    let _a = 1u8;
    let _b = 0xff;
    let _c = 3f32;
    let _d = 1_000;

    // 정수 오버플로우
    // 지정된 크기를 벗어나면 오버플로우 나는거
    // 2의 보수 감싸기 (two's complement wrapping)를 수행하면 패닉은 피할수 있지만 의도치 않은 동작을 유발할 수 있다

    // let mut _overflow_u8: u8 = 255 + 1; // attempt to compute `u8::MAX + 1_u8`, which would overflow

    use std::num::Wrapping;
    let mut _not_overflow_with_wrapping = Wrapping(255u8) + Wrapping(1u8);
    println!("{_not_overflow_with_wrapping}"); // 0 (2의 보수 감싸기)

    let mut _num_1: u8 = 1;
    let _not_overflow_with_checked = _num_1.checked_add(255u8);
    println!("{:?}", _not_overflow_with_checked); // None

    let _is_it_overflowed = _num_1.overflowing_add(255u8);
    println!("{:?}", _is_it_overflowed); // (0, true)

    let _not_overflow_with_saturating = _num_1.saturating_add(255u8);
    println!("{_not_overflow_with_saturating}"); // 255

    // 2. 복합 타입
    // 여러 값을 하나의 타입으로 묶는 것
    // 튜플 (tuple) 과 배열 (array), 두 가지 기본 복합 타입 존재
    // 아무 값도 없는 튜플은 유닛 (unit)이라고 함

    let _tup = (500, 6.4, 1);
    let (_x, _y, _z) = _tup;

    println!("{:?}", _tup.1); // 6.4
    println!("{_y}"); // 6.4

    let _arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", _arr[2]); // 3
}
