fn 테스트_작성_방법() {
    print!(
        "
    러스트에서 테스트란 test 속성(#[test])이 어노테이션된 함수
    속성은 러스트 코드 조각에 대한 메타데이터
    
    $ cargo test 명령어로 실행

    여러 테스트가 실행될 때, 각각의 테스트는 새로운 스레드에서 실행
    메인 스레드에서 테스트 스레드가 죽은 것을 알게 되면 해당 테스트는 실패한 것으로 처리
    \n"
    );
}

#[test]
fn 테스트_함수_성공하는_예시() {
    let 글자 = "글자";

    assert_eq!(글자, "글자");

    // running 1 test
    // test 테스트_함수_성공하는_예시 ... ok

    // test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
}

#[test]
fn 테스트_함수_실패하는_예시() {
    panic!("테스트 실패!")

    // running 1 test
    // test 테스트_함수_실패하는_예시 ... FAILED

    // failures:

    // ---- 테스트_함수_실패하는_예시 stdout ----
    // thread '테스트_함수_실패하는_예시' panicked at src/main.rs:6:5:
    // 테스트 실패!
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    // failures:
    //     테스트_함수_실패하는_예시

    // test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
}

#[test]
fn assert_매크로로_결과_검사하기() {
    print!(
        "
    어떤 조건이 true임을 보장하는 테스트를 작성할 땐 표준 라이브러리의 assert! 매크로가 유용
    assert! 매크로는 부울린 값으로 평가되는 인수를 전달받아 true면 통과, false면 panic을 일으킴
    \n"
    );

    assert!(1 == 1);
    // successes:
    // assert_매크로로_결과_검사하기

    // test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    assert!(1 == 2)
    // failures:
    // assert_매크로로_결과_검사하기

    // test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

    // error: test failed, to rerun pass `-p docs --bin docs`
}

#[test]
fn assert_eq_assert_ne_매크로를_이용한_동등_테스트() {
    print!(
        "
    assert_eq!, assert_ne! 매크로는 각각 두 인수를 비교하고 동등한지 그렇지 않은지 판단
    실패하면 두 값을 출력하여 테스트의 실패 사유를 assert!보다 더 알기 쉽게 보여줌

    내부적으로 ==, != 연산자를 사용하고 단언에 실패할 경우, 매크로는 인수를 디버그 형식으로 출력
    PartialEq, Debug 트레이트를 구현해야 함

    ㄴ> expected, actual이 아니라 left, right인 것도 개인적으로 아주 좋게 느껴지는 부분
}
    \n"
    );

    assert_eq!(1, 1);
    // successes:
    // assert_eq_assert_ne_매크로를_이용한_동등_테스트

    // test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    assert_eq!(1, 2);
    // thread 'assert_eq_assert_ne_매크로를_이용한_동등_테스트' panicked at src/main.rs:18:5:
    // assertion `left == right` failed
    // left: 1
    // right: 2
    // stack backtrace:

    // -- 중간 생략 --

    // failures:
    // assert_eq_assert_ne_매크로를_이용한_동등_테스트

    // test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
}

#[test]
fn 커스텀_실패_메시지_추가하기() {
    print!(
        "
    필수적인 인수들 이후의 인수는 format! 매크로로 전달
    \n"
    );

    let msg = "커스텀 에러 메시지";

    assert!(false, "msg: `{}`", msg);
    // thread '커스텀_실패_메시지_추가하기' panicked at src/main.rs:13:5:
    // msg: `커스텀 에러 메시지`
}

#[test]
#[should_panic]
fn should_panic_매크로로_패닉_발생_검사하기() {
    print!(
        "
    #[should_panic] 속성을 추가하면 패닉이 일어나는지 여부를 검사할 수 있다.

    should_panic을 사용하는 테스트는 정확하지 않을 수 있다.
    만약 테스트가 의도한 것과는 다른 이유로 패닉이 발생하더라도 should_panic 테스트는 통과할 것이다.
    \n"
    );

    panic!("패닉 발생! -> 통과!")
    // thread 'should_panic_매크로로_패닉_발생_검사하기' panicked at src/main.rs:12:5:
    // 패닉 발생! -> 통과!

    // -- 중간 생략 --

    // successes:
    // should_panic_매크로로_패닉_발생_검사하기

    // test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
}

#[test]
fn Result를_이용한_테스트() -> Result<(), String> {
    print!(
        "
    테스트는 Result<T, E>를 사용해 작성할 수도 있다.

    Result<T, E>를 반환하는 테스트에서는 ? 연산자를 사용할 수 있다. 
    내부 작업이 Err를 반환할 경우 실패해야 하는 테스트를 작성하기 편리하다.

    Result<T, E> 테스트에서는 #[should_panic] 어노테이션을 사용할 수 없다.
    \n"
    );

    if true {
        Ok(())
    } else {
        Err(String::from("에러 발생!"))
    }
}
