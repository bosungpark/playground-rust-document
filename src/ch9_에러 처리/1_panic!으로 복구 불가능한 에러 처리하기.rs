fn panic으로_복구_불가능한_에러_처리하기() {
    // 패닉에 대하여
    print!("
    패닉을 일으키는 두 가지 방법:

    1. 코드가 패닉을 일으킬 동작을 하는 것 
    2. panic! 매크로를 호출하는 것

    기본적으로 패닉은 아래의 순서로 처리된다:
    
    1. 실패 메시지를 출력
    2. 되감기 (unwind)
    3. 스택을 청소
    4. 종료

    패닉의 근본적 원인을 쉽게 추적하기 위해 환경 변수로 러스트가 호출 스택을 보여주도록 할 수 있음
    \n");

    // 매크로가 패닉을 일으키는 경우
    panic!("흑흑 패닉");
}

fn panic에_대응하여_스택을_되감거나_그만두기() {
    print!("
    기본적으로, panic!이 발생하면, 프로그램은 되감기 (unwinding) 를 시작한다.
    이는 러스트가 패닉을 발생시킨 각 함수로부터 스택을 거꾸로 훑어가면서 데이터를 청소한다는 뜻이다.

    하지만 되감기는 비용이 있다.
    러스트에서는 프로그램이 데이터 정리 작업 없이 즉각 종료되는 대안인 그만두기 (aborting) 를 선택할 수도 있다.
    \n");
}

fn panic_백트레이스_이용하기() {
    // 직접 매크로를 호출하는 대신, 코드의 버그 때문에 라이브러리로부터 panic! 호출이 발생하는 경우
    let v = vec![1];
    v[99];  // 버퍼 초과 읽기 (buffer overread) 발생을 막기 위한 패닉!

    print!("
    $ RUST_BACKTRACE=1 cargo run

    위와 같이 환경 변수를 설정하면 런타임에 panic!이 발생한 곳을 보다 자세히 추적할 수 있다.
    \n");
}
