// 러스트 열거형의 가장 중요한 특징은 반드시 하나에 매칭이 된다는 것이다
// 구조체는 and  조건으로 나열될 수 있으나, 열거형은 걸리는 정답이 하나라는 특성을 가진다.
// 이 점을 잘 이용하면 match 절 등에서 예외없는 검증이 가능하다.

match 흐름 제어 연산자
=

```
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

만일 매치 갈래 내에서 여러 줄의 코드를 실행시키고 싶다면, 중괄호를 이용할 수 있다. 
-

```
match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
```

매치 갈래의 또 다른 유용한 기능은 패턴과 매치된 값들의 부분을 바인딩할 수 있다
-
```
#[derive(Debug)] // So we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
```

Option<T>를 이용하는 매칭
-
```
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
```

ㄴ> None의 경우를 무시하면 컴파일 에러

_ 변경자(placeholder)
-

-는 모든 가능한 값을 나열하고 싶지 않을 경우에 사용할 수 있는 패턴이다

