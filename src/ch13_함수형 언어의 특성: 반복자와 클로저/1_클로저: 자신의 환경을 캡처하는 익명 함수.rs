fn 클로저로_환경_캡처하기() {
    println!(
        "
        클로저는 변수에 저장하거나 다른 함수에 인수로 전달할 수 있는 익명 함수
        함수와 다르게 클로저는 정의된 스코프에서 값을 캡처할 수 있다.

        '|매개변수| 표현식'의 형태로 작성

        클로저는 인스턴스의 불변 참조자를 캡처하여 우리가 지정한 코드와 함께 이 값을 메서드에 넘겨중 수 있다. 
        반면에 함수는 이런 방식으로 자신의 환경을 캡처할 수 없다.

        예:

        fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
            user_preference.unwrap_or_else(|| self.most_stocked())
        }
        "
    )
}

fn 클로저_타입_추론과_명시() {
    println!(
        "
        클로저는 보통 fn 함수에서처럼 매개변수 혹은 반환 값의 타입을 명시하도록 요구하지 않는다.
        함수의 타입 명시는 그 타입이 사용자들에게 노출되는 명시적인 인터페이스의 일부분이기 때문에 요구된다.

        반면에 클로저는 함수처럼 노출된 인터페이스로 사용되지 않는다.
        클로저는 이름이 지어지거나 라이브러리의 사용자들에게 노출되지 않은 채로 변수에 저장되고 사용된다.

        클로저는 통상적으로 짧고,짧은 컨텍스트 내에서만 관련된다.
        이러한 한정된 컨텍스트 내에서, 컴파일러는 대부분의 변수에 대한 타입을 추론하는 방법과 비슷한 식으로 클로저의 매개변수와 반환 타입을 추론한다.
        당연하지만 하나의 클로저에 대해 서로 다른 두 호출에서 다른 값을 입력하면 컴파일러는 에러를 반환한다.

        하지만 명시성과 명확성을 올리고 싶다면 타입 명시를 추가할 수도 있다.
        타입을 명시하면 일반 함수와 매우 비슷한 모양을 보인다.

        예시:

        fn  add_one_v1   (x: u32) -> u32 { x + 1 }   // 함수 정의
        let add_one_v2 = |x: u32| -> u32 { x + 1 };  // 모든 것이 명시된 클로저 정의
        let add_one_v3 = |x|             { x + 1 };  // 타입 명시를 제거
        let add_one_v4 = |x|               x + 1  ;  // 중괄호를 제거 (클로저의 본문이 딱 하나의 표현식이기 때문에 가능)
        "
    );

    let 타입을_명시한_클로저_예시 = |매개변수: u32| -> u32 { 매개변수 };
}

use std::thread;

fn 참조자를_캡처하거나_소유권_이동하기() {
    println!(
        "
        세 가지 방식으로 자신의 환경으로부터 값을 캡처할 수 있다.
        (함수가 매개변수를 취하는 세 가지 방식과 직접적으로 대응)

        1. 불변으로 빌려오기
        2. 가변으로 빌려오기
        3. 소유권 이동

        엄밀하게는 클로저의 본문에서 사용하고 있는 값의 소유권이 필요하진 않더라도 
        만약 클로저가 소유권을 갖도록 만들고 싶다면, 매개변수 리스트 전에 move 키워드를 사용할 수 있다.     
        대체로 클로저를 새 스레드에 넘길 때 데이터를 이동시켜서 새로운 스레드가 이 데이터를 소유하게 하는 경우 유용하다.
        (아직은 모르겠음. 동시성 때 나온다함.)
        ",
    );

    // 1. 불변으로 빌려오기
    let 불변_벡터 = vec![1, 2, 3];

    // 어떤 변수가 클로저의 정의에 바인딩될 수 있고,
    // 이 클로저는 나중에 마치 변수 이름이 함수 이름인 것처럼 변수 이름과 괄호를 사용하여 호출될 수 있음.
    let 불변으로_빌려오기 = || println!("불변으로 빌려오기 중: {:?}", 불변_벡터);

    println!("불변으로 빌려오기 전: {:?}", 불변_벡터);
    불변으로_빌려오기();
    println!("불변으로 빌려오기 후: {:?}", 불변_벡터);
    // 불변으로 빌려오기 전: [1, 2, 3]
    // 불변으로 빌려오기 중: [1, 2, 3]
    // 불변으로 빌려오기 후: [1, 2, 3]

    // 2. 가변으로 빌려오기
    let mut 가변_벡터 = vec![1, 2, 3];

    println!("가변으로 빌려오기 전: {:?}", 가변_벡터);
    let mut 가변으로_빌려오기 = || 가변_벡터.push(4);

    // 클로저가 호출된 이후로 다시 클로저를 사용하고 있지 않으므로, 가변 대여가 그 시점에서 끝남.
    // 클로저 정의와 호출 사이에는 출력을 위한 불변 대여가 허용되지 않는다.
    // 왜냐하면 가변 대여가 있을 때는 다른 대여가 허용되지 않기 때문이다.
    가변으로_빌려오기();
    println!("가변으로 빌려오기 후: {:?}", 가변_벡터);
    // 가변으로 빌려오기 전: [1, 2, 3]
    // 가변으로 빌려오기 후: [1, 2, 3, 4]

    thread::spawn(move || println!("소유권 이동!: {:?}", 불변_벡터))
        .join()
        .unwrap();
}

fn 캡처된_값을_클로저_밖으로_이동하기와_Fn_트레이트() {
    println!(
        "
        어떤 클로저가 자신이 정의된 환경으로부터 값의 참조자 혹은 소유권을 캡처하면 (그래서 클로저의 안으로 이동되는 것에 영향을 준다면), 
        클로저 본문의 코드는 이 클로저가 나중에 평가될 때 그 참조자나 값에 어떤 일이 발생하는지 정의한다.

        본문이 하는 예시:

        1. 캡처된 값을 클로저 밖으로 이동시키기
        2. 캡처된 값을 변형하기
        3. 이동시키지도 변형시키지도 않기
        4. 시작 단계에서부터 환경으로부터 아무 값도 캡처하지 않기

        클로저가 환경으로부터 값을 캡처하고 다루는 방식은 이 클로저가 구현하는 트레이트에 영향을 주고, 
        트레이트는 함수와 구조체가 사용할 수 있는 클로저의 종류를 명시할 수 있는 방법이다.
        클로저는 클로저의 본문이 값을 처리하는 방식에 따라서 적절한 Fn 트레이트를 추가하는 방식으로 자동으로 구현한다.

        1. FnOnce: 한 번만 호출될 수 있는 클로저에게 적용. 캡처된 값을 이동시키는 경우.
        2. FnMut: 본문 밖으로 캡처된 값을 이동시키지는 않지만 값을 변경할 수는 있는 클로저에 대해 적용
        3. Fn: 캡처된 값을 본문 밖으로 이동시키지 않고 캡처된 값을 변경하지도 않는 클로저는 물론, 환경으로부터 아무런 값도 캡처하지 않는 클로저에 적용

        예:

        impl<T> Option<T> {
            pub fn unwrap_or_else<F>(self, f: F) -> T
            where
                F: FnOnce() -> T
            {
                match self {
                    Some(x) => x,
                    None => f(),
                }
            }
        }


        "
    )
}
