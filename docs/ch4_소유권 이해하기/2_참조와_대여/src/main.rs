fn main() {
    참조자와_대여();
    가변_참조자();
    댕글링_참조();
}

fn 참조자와_대여() {
    // 1. 참조자 (reference):
    // 해당 주소에 저장된 데이터에 접근할 수 있도록 해주는 주솟값에 해당하는, 포인터와 같은 것
    // 소유권은 넘겨주지 않고, 참조자가 살아있는 동안 특정 타입에 대한 유효한 값을 가리킴을 보장해줌
    // 이러한 행위를 대여하고 함

    let _참조자만_넘길거임 = String::from("무효화 안됐지롱");
    cannot_change(&_참조자만_넘길거임);

    println!("{_참조자만_넘길거임}"); // 무효화 안됐지롱

    // ㄴ> 참조자가 가르키는 방향: _s(foo) -> _참조자만_넘길거임 -> "무효화 안됐지롱"
}

fn cannot_change(_s: &String) {
    // _s.push_str("빌린 값은 바꿀 수 없음"); // error[E0596]: cannot borrow `*_s` as mutable, as it is behind a `&` reference
}

fn 가변_참조자() {
    let mut _이건_가변_참조자임 = String::from("");
    can_change(&mut _이건_가변_참조자임);

    println!("{_이건_가변_참조자임}"); // 바뀜

    // 하지만 가변 참조자는 한 가지 큰 제약사항이 있다
    // 어떤 값에 대한 가변 참조자가 있다면, 그 값에 대한 참조자는 더 이상 만들 수 없다

    let 가변_참조자_생성1 = &mut _이건_가변_참조자임;
    let 가변_참조자_생성2 = &mut _이건_가변_참조자임;

    // error[E0499]: cannot borrow `_이건_가변_참조자임` as mutable more than once at a time
    // println!("{}, {}", 가변_참조자_생성1, 가변_참조자_생성2);

    // ㄴ> 동시에 두 번 빌리려고 하기 때문
    // ㄴ> 이런 접근은 데이터 경합 (data race) 을 방지에 도움이 됨
    // ㄴ> {} 스코프를 사용하여 안전한 사용을 할 것을 권장
    // ㄴ> 가변과 불변을 혼용하여 참조하는 경우 역시 마찬가지
}

fn can_change(_s: &mut String) {
    _s.push_str("바뀜");
}

fn 댕글링_참조() {
    // 1. 댕글링 포인터
    // 포인터가 여전히 해제된 메모리 영역을 가리키고 있는 포인터.
    // 댕글링 포인터가 가리키는 메모리는 더는 유효하지 않다.
    // 러스트는 댕글링 포인터가 생기지 않음을 보장한다.

    // error[E0106]: missing lifetime specifier
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    // error[E0106]: missing lifetime specifier
    // let s = String::from("dangle");

    // &s

    // ㄴ> &s의 값은 버려질테니까 댕글이다!
    // ㄴ> s이렇게 값을 직접 반환해 소유권을 넘겨야한다.
}
