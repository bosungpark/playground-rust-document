fn main() {}

fn 공통된_동작을_위한_트레이트의_정의() {
    println!(
        "
        트레이트의 명확한 목적은 공통된 동작에 대한 추상화를 가능하도록 하는 것

        트레이트 객체는 특정 트레이트를 구현한 타입의 인스턴스와 
        런타임에 해당 타입의 트레이트 메서드를 조회하는 데 사용되는 테이블 모두를 가리킨다.

        & 참조자나 Box<T> 스마트 포인터 같은 포인터 종류로 지정한 다음 dyn 키워드를 붙이고, 
        그 뒤에 관련된 트레이트를 특정하면 트레이트 객체를 생성할 수 있다.

        ㄴ> 사용법. 아래에 예시 있음.

        제네릭 타입이나 구체 타입 대신 트레이트 객체를 사용할 수 있다.
        트레이트 객체를 사용하는 곳이 어디든, 
        러스트의 타입 시스템은 컴파일 타임에 해당 컨텍스트에서 사용된 모든 값이 트레이트 객체의 트레이트를 구현할 것을 보장한다.
        컴파일 타임에 모든 가능한 타입을 알 필요가 없다.

        ㄴ> 쉽게 말하면 다형성에 관한 이야기

        트레이트 객체들은 데이터와 동작을 결합한다는 의미에서 다른 언어의 객체와 더 비슷하다.
        하지만 트레이트 객체는 트레이트 객체에 데이터를 추가할 수 없다는 점에서 전통적인 객체와 다르다.

        트레이트와 지네릭은 비슷하게 보일 수 있지만, 차이점이 있다.
        벡터를 예로 들면, 지네릭은 전부 같은 타입의 객체를 담아야 하는 반면, 트레이트는 여러 타입이더라도 동일한 트레이트만 구현하였다면 담는 것이 문제가 되지 않는다.
    "
    )
}

// 트레이트 정의 예시
pub trait Draw {
    fn draw(&self);
}

// components라는 벡터를 보유한 Screen 객체
pub struct Screen {
    // 트레이트 객체: Box<dyn Draw>. 즉, Draw 트레이트를 구현한 Box 안의 모든 타입을 의미
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

fn 트레이트_구현하기() {
    // Draw 트레이트를 구현한 Button, SelectBox를 다형성으로서 사용하는 예시

    // 이와 같이, 값의 구체적인 타입이 아닌 값이 응답하는 메시지만 고려하는 개념은 동적 타입 언어의 덕타입핑과 유사하다.
    // 이런 방식의 장점은 런타임에 어떤 값이 특정한 메서드를 구현했는지 여부를 검사하거나
    // 값이 메서드를 구현하지 않았는데 어쨌든 호출한 경우 에러가 발생할 것을 걱정할 필요가 없다는 것이다.
    // 트레이트 객체가 요구하는 트레이트를 해당 값이 구현하지 않았다면 러스트는 컴파일하지 않을 것이기 때문.

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
    // SelectBox의 메서드 호출!
    // Button의 메서드 호출!
}

// Draw 트레이트를 구현한 타입 예시
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // 실제로 버튼을 그리는 코드
        println!("Button의 메서드 호출!")
    }
}

// Draw 트레이트를 구현한 타입 예시
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // 실제로 선택 상자를 그리는 코드
        println!("SelectBox의 메서드 호출!")
    }
}

fn 트레이트_객체는_동적_디스패치를_수행합니다() {
    println!(
        "
        동적 디스패치는 컴파일러가 호출하는 메서드를 컴파일 시점에 알 수 없을 경우 수행된다.
        (지네릭의 단형성화와 정적 디스패치와는 반대되는 개념)

        컴파일러는 트레이트 객체를 사용 중인 코드와 함께 사용될 수 있는 모든 타입을 알지 못하므로, 
        어떤 타입에 구현된 어떤 메서드가 호출될지 알지 못한다.

        대신 런타임에서, 러스트는 트레이트 객체 내에 존재하는 포인터를 사용하여 어떤 메서드가 호출될지 알아낸다.
        이러한 조회는 정적 디스패치 시에는 발생하지 않을 런타임 비용을 만들어냄.

        또 동적 디스패치는 또한 컴파일러가 메서드의 코드를 인라인화하는 선택을 막아버려 일부 최적화를 하지 못하게 함.
    "
    )
}
