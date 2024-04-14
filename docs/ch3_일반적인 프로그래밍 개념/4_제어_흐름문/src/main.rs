fn main() {
    if_표현식();
    반복문을_이용한_반복();
}

fn if_표현식() {
    if true {
        println!("lalala 즐거운 if 표현식");
    } else {
        println!("이건 호츨될 일 없음")
    }

    // if "bool이 아닌 자료형" {
    //     // expected `bool`, found `&str`
    //     println!("에러가 나겠지 ㅠㅠ")
    // }

    let _let_구문에서_if_사용하기 = if true {
        "let 구문에서 if 사용할 수 있다!"
    } else {
        "이건 호츨될 일 없음"
    };
    println!("{:?}", _let_구문에서_if_사용하기)
}

fn 반복문을_이용한_반복() {
    // 1. loop
    // break시에 반환 값을 사용 가능
    // 루프 라벨 (loop label) 을 추가적으로 명시가능
    let _loop_result = loop {
        println!("이건 조건 없는 루프,,");
        break "휴 정지";
    };
    println!("{_loop_result}"); // 휴 정지

    // 2. while
    while 1 == 1 {
        println!("이건 조건 있는 와일,,");
        break;
    }

    // 3. for
    let _arr = ["for", "문", "이", "라", "네"];
    for element in _arr {
        println!("{element}");
    }
}
