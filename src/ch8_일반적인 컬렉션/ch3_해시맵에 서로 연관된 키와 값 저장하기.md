해시맵에 서로 연관된 키와 값 저장하기
=

해시맵은 벡터에서처럼 인덱스를 이용하는 것이 아니라 임의의 타입으로 된 키를 이용하여 데이터를 찾고 싶을 때 유용

벡터와 마찬가지로, 해시맵도 데이터를 힙에 저장
벡터와 비슷하게 해시맵도 동질적입니다: 모든 키는 서로 같은 타입이어야 하고, 모든 값도 같은 타입이여야 함

해시맵과 소유권
-
i32처럼 Copy 트레이트를 구현한 타입의 값은 해시맵 안으로 복사

String처럼 소유권이 있는 값의 경우,  같이 값들이 이동되어 해시맵이 그 값의 소유자가 됨

해시 함수
-

기본적으로 HashMap은 해시 테이블과 관련된 서비스 거부 공격 (Denial of Service(DoS) attack) 에 저항 기능을 제공할 수 있는 SipHash라 불리는 해시 함수를 사용

만일 기본 해시 함수가 너무 느리다면, 다른 해시어를 지정하여 다른 함수로 바꿀 수 있다