fn 클로저로_환경_캡처하기() {
    println!(
        "
        반복자는 비록 고수준의 추상화지만, 컴파일되면 대략 직접 작성한 저수준의 코드와 같은 코드 수준의 속도로 내려간다.
        반복자는 러스트의 비용 없는 추상화 (zero-cost abstraction) 중 하나이다. (런타임 오버헤드가 없다)

        러스트 코드가 컴파일되면 직접 손으로 작성한 것과 같은 어셈블리 코드로 컴파일된다.
        러스트는 몇 번의 반복이 있다는 것을 알 수 있는 경우, 루프를 ‘풀어 (unrolls)’ 놓는다.
        언롤링(unrolling) 은 루프 제어 코드의 오버헤드를 제거하고 대신 루프의 각 순회에 해당하는 반복되는 코드를 생성하는 최적화 방법이다.
        모든 계수는 레지스터에 저장되어 값에 대한 접근 속도가 매우 빠르다.

        즉, 반복자와 클로저는 코드를 좀 더 고수준으로 보이도록 하지만, 런타임 성능에 불이익을 주지 않는다.
        "
    )
}