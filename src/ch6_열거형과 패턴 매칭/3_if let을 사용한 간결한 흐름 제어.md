// 러스트 열거형의 가장 중요한 특징은 반드시 하나에 매칭이 된다는 것이다
// 구조체는 and  조건으로 나열될 수 있으나, 열거형은 걸리는 정답이 하나라는 특성을 가진다.
// 이 점을 잘 이용하면 match 절 등에서 예외없는 검증이 가능하다.

if let을 사용한 간결한 흐름 제어
-

```
if let Some(3) = some_u8_value {
    println!("three");
}
```

ㄴ> bool이 아니여도 된다!

ㄴ> match에 비교해 더 간결한 표현이 가능하다.

ㄴ> 하지만, match가 강제했던 하나도 빠짐없는 검사는 불가하다!

ㄴ> if let를 어떤 값이 하나 패턴에 매칭 되었을 때 코드를 실행하고 다른 값들에 대해서는 무시하는 match 문을 위한 문법적 설탕(syntax sugar)으로 생각 가능.

else를 사용하는 것도 가능하다.

```
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```


