use std::fs::File;

fn Result로_복구_불가능한_에러_처리하기() {
    print!("
    에러가 발생하더라도 간단하게 대응할 수 있는 경우도 많다.
    Result 열거형으로 이에 대한 대응을 할 수가 있다.
    \n");

    // 실패할 가능성이 있어서 Result 값을 반환하는 코드
    let Result_타입의_반환값 = File::open("없는 파일 ㅋ.txt");

    let 파일 = match Result_타입의_반환값 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => print!("흑흑 파일이 없어요 ㅜㅜ"),
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn unwrap과_expect에_대하여() {
    print!("
    unwrap

    1. Result 값이 Ok 배리언트라면, unwrap은 Ok 내의 값을 반환
    2. Result가 Err 배리언트라면 unwrap은 panic! 매크로를 호출
    \n");

    let 파일 = File::open("없는 파일 ㅋ.txt").unwrap();

    print!("
    expect

    unwrap과 비슷하지만, 에러 메시지를 지정할 수 있다.
    프로덕션급 품질의 코드에서 대부분의 러스타시안은 unwrap보다 expect를 선호하는 경향이 있다고 한다.
    \n");

    let 파일 = File::open("없는 파일 ㅋ.txt").expect("흑흑 파일이 없어요 ㅜㅜ");
}

fn 에러_전파하기() {
    print!("
    에러 전파하기 (propagating) 

    함수를 호출하는 코드 쪽으로 에러를 반환하여 그쪽에서 수행할 작업을 결정하는 방식
    \n");

    fn 에러를_반환하는_함수_예시() -> Result<File, io::Error> {
        // 방법 1
        let 파일 = File::open("없는 파일 ㅋ.txt");

        let 결과파일 = match Result_타입의_반환값 {
            Ok(_) => Ok(결과파일),
            Err(e) => return Err(e),
        };

        // 방법 2 (? 숏컷 버전)
        let 파일 = File::open("없는 파일 ㅋ.txt")?;  // 체이닝 가능
        Ok(파일)
    }

    print!("
    match 표현식과 ? 연산자의 차이점:

    ? 연산자를 사용할 때의 에러 값들은 from 함수를 거친다.

    from 함수는 표준 라이브러리 내의 From 트레이트에 정의됨.
    어떤 값의 타입을 다른 타입으로 변환하는 데에 사용된다.

    ? 연산자가 from 함수를 호출하면, ? 연산자가 얻게 되는 에러를 ? 연산자가 사용된 현재 함수의 반환 타입에 정의된 에러 타입으로 변환한다.

    어떤 함수가 다양한 종류의 에러로 인해 실패할 수 있지만, 모든 에러를 하나의 에러 타입으로 반환할 때 유용하다.
    \n");

    print!("
    ? 연산자가 사용될 수 있는 곳:

    ?는 ?이 사용된 값과 호환 가능한 반환 타입을 가진 함수에서만 사용가능하다.

    ㄴ> 당연한 소리!
    
    \n");
}