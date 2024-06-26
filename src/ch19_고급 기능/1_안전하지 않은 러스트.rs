fn 안전하지_않은_러스트() {
    println!(
        "
        러스트는 기본적으로 컴파일 타임에 메모리 안전이 보장됨.
        정적 분석은 본질적으로 보수적이기 때문에 '안전하지 않은 러스트'라는 새로운 방식의 언어가 존재함.
        하지만, 안전하지 않은 러스트를 사용하는 것은 사용자의 책임하에 사용해야 한다.
        안전하지 않은 코드를 잘못 사용하면, 메모리 불안정성으로 인하여 널 포인터 역참조와 같은 문제가 발생할 수 있다.

        컴퓨터 하드웨어가 본질적으로 안전하지 않기 때문에 러스트가 안전하지 않은 작업을 허용하지 않으면, 특정 작업을 수행할 수 없다는 면도 한 가지의 다른 이유이다.

        안전하지 않은 러스트로 전환하려면 unsafe 키워드를 사용한 다음 새 블록을 시작하여 안전하지 않은 코드를 집어넣으면 된다.
        아래와 같은 5가지 슈퍼파워를 가지게 된다.

        - 원시 포인터 (raw pointer) 역참조하기
        - 안전하지 않은 함수 혹은 메서드 호출하기
        - 가변 정적 변수에 접근하기 및 수정하기
        - 안전하지 않은 트레이트 구현하기
        - union의 필드 접근하기
        
        unsafe가 대여 검사기를 끄거나 러스트의 다른 안전성 검사를 비활성화하지 않는다.
        안전하지 않은 코드에서 참조를 사용하면, 검사는 여전히 이루어진다.
        unsafe 키워드는 컴파일러가 메모리 안전성을 검사하지 않는 위의 다섯 가지 기능 허용만 제공할 뿐 어느 정도의 안전은 보장한다.

        안전하지 않은 연산을 unsafe로 주석 처리된 블록 안에 넣도록 하면 메모리 안전과 관련된 모든 에러의 범위를 특정할 수 있다.
        안전하지 않은 코드를 최대한 분리하려면 안전하지 않은 코드를 안전한 추상화 안에 넣고 안전한 API를 제공하는 것이 가장 좋디. 
        "
    )
}

fn 원시_포인터_역참조하기() {
    // 러스트에는 참조와 유사한 원시 포인터 (raw pointer) 라는 두 가지 새로운 타입이 있다.
    // 러스트의 보증이 적용되지 않는 다른 언어 또는 하드웨어와 인터페이싱할 수 있는 기능이나 더 나은 성능을 얻을 수 있다.

    // - 원시 포인터는 대여 규칙을 무시할 수 있으며, 같은 위치에 대해 불변과 가변 포인터를 동시에 가질 수 있거나 여러 개의 가변 포인터를 가질 수 있다.
    // - 원시 포인터는 유효한 메모리를 가리키는 것을 보장받지 못한다.
    // - 원시 포인터는 널 (null) 이 될 수 있다.
    // - 원시 포인터는 자동 메모리 정리를 구현하지 않는다.

    let mut num = 5;

    // 1. 가변과 불변 참조자를 동시에 사용할 수 있는 예시
    // unsafe 키워드 없이, 안전한 코드에서도 생성할 수는 있다.
    // (생성 자체는 문제가 없으므로. 단, 참조는 불가능)
    // as가 원시 포인터로 캐스팅 해준다.
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 2. 유효성을 반드시 보장할 수는 없는 경우의 예시
    let address = 0x012345usize;
    let r = address as *const i32;

    // unsafe 키워드에서 참조 가능
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

fn 안전하지_않은_함수_또는_메서드_호출하기() {
    // 정의 앞부분에 unsafe가 추가
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}

// 안전한 러스트만 사용하여 구현할 수는 없는 예시
// 러스트의 대여 검사기는 슬라이스의 서로 다른 부분을 빌린다는 것을 이해할 수 없다.
// fn 안전한_러스트만_사용하여_구현할_수는_없는_예시(
//     values: &mut [i32],
//     mid: usize,
// ) -> (&mut [i32], &mut [i32]) {
//     let len = values.len();

//     assert!(mid <= len);

//     (&mut values[..mid], &mut values[mid..])
// }

use std::slice;

fn unsafe_블록_원시_포인터_그리고_안전하지_않은_함수_호출을_사용하는_예시(
    values: &mut [i32],
    mid: usize,
) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr(); // 원시 포인터

    assert!(mid <= len);

    // unsafe 블록
    unsafe {
        (
            // 원시 포인터와 길이를 받아 슬라이스를 생성
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn extern_함수를_사용하여_외부_코드_호출하기() {
    // c 언어의 abs와의 통합을 하는 예시
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

fn 가변_정적_변수의_접근_혹은_수정하기() {
    // 정적 변수의 값은 메모리에 고정된 주소를 갖는다.
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// 안전하지 않은 트레이트 구현하기

// 메서드 중 하나 이상에 컴파일러가 확인할 수 없는 불변성 (invariant) 이 있는 경우 그 트레이트는 안전하지 않다.
// 이 때 unsafe를 추가하여 구현할 수 있다.

fn 유니온_필드에_접근하기() {
    // union은 struct와 유사하지만, 특정 인스턴스에서 한 번에 하나의 선언된 필드만 사용된다.
    // 유니온은 주로 C 코드의 유니온과 상호작용하는데 사용된다.

    union Union {
        i: i32,
        f: f32,
    }

    let u = Union { i: 42 };

    unsafe {
        println!("Integer: {}", u.i);
    }

    let u = Union { f: 3.14 };

    unsafe {
        println!("Float: {}", u.f);
    }
}
