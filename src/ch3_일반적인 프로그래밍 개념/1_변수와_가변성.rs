fn 불변_변수() {
    // 변수는 기본적으로 불변이다 (안정성과 쉬운 동시성을 활용하기 위한 넛지)
    // 이 말은 코드를 읽고 쓸 때 값이 어디서 어떻게 바뀔지 추적할 필요가 없다는 것

    // let 불변_변수_예시1 = 5;
    // 불변_변수_예시1 = 6; // 실패! cannot assign twice to immutable variable

    // ㄴ> 이거 에러 추적하는거 되게 친절하다..
}

fn 가변_변수() {
    // 하지만 원한다면 가변적이게 사용할 수 있다
    // 변수명 앞에 mut을 붙여서 가변으로 만들 수 있다
    // mut를 추가하는 것은 또한 미래에 코드를 읽는 이들에게 코드의 다른 부분에서 이 변수의 값이 변할 것이라는 의도를 전달

    let mut 가변_변수_예시1 = 5; // warning: value assigned to `가변_변수1` is never read
    가변_변수_예시1 = 6;
    println!("{가변_변수_예시1}"); // 6

    // ㄴ> 이거 warning 뜨는거 착각하지 말라고 해주는건가? 친절하다..
}

fn 상수() {
    // 상수는 불변 변수와 비슷하지만 mut와 함께 사용할 수 없다 (항상 불변)
    // 값의 타입은 반드시 명시
    // 상수는 전역 스코프를 포함한 어떤 스코프에서도 선언 가능
    // 상수는 반드시 상수 표현식으로만 설정될 수 있고 런타임에서만 계산될 수 있는 결괏값으로는 못한다

    const 상수_예시1: u32 = 60 * 60 * 3;
    println!("{상수_예시1}"); // 10800
}

fn 셰도잉() {
    // 새 변수를 이전 변수명과 같은 이름으로 선언한 경우 앞의 변수가 "가려졌다"고 표현한다
    // 변수를 재선언 하는 것은 값을 바꾸는 것과는 다르다
    // mut와 다르게 해당 스코프 안에서는 변수는 불변을 유지한다
    // mut와 다르게 원한다면 다른 타입으로 재선언 해도 좋다

    let 셰도잉_될_변수 = 10;
    {
        let 셰도잉_될_변수 = 셰도잉_될_변수 * 10;

        println!("{셰도잉_될_변수}"); // 100
    }
    println!("{셰도잉_될_변수}"); // 10

    let 셰도잉_될_변수 = "랄라라 재선언해서 타입 변환이라네";
    println!("{셰도잉_될_변수}"); // 랄라라 재선언해서 타입 변환이라네

    // ㄴ> 이거,,,"섀도잉"이라는 표현,,,뭔가 러스트가 소유권에 예민한거랑 비슷한 건가....
    // ㄴ> 누가 뭘 바라보는지에 예민한 거 같아보인다,,,
    // ㄴ> 근데 러스트는 왜 불변 변수를 기본으로 하면서 섀도잉을 허용하는지,,똑같이 헷갈릴 수 있으면 모순 아닌가,,,
    // ㄴ> 고랭처럼 에초에 재선언을 막던, 아니먄 변수는 기본적으로 불변이라는 컨셉을 버리던 둘 중 하나를 택해야 하는거 아닌가?
    // ㄴ> 변형이 완료된 후에는 불변으로 유지되니까, 스코프 안에서는 선언된 이후에 헷갈릴 일이 없다는 그런 취지인가..?
    // ㄴ> 뭔가 무책임한거 같은데,,누군가 극단적으로 매번 같은 값을 재할당하는 식으로 코드를 짜면,,그건 그냥 그 사람 개인의 뇌절인건가,,,,
}
