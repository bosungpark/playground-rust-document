커져 가는 프로젝트를 패키지, 크레이트, 모듈로 관리하기
=

1. 한 패키지에는 여러 개의 바이너리 크레이트와 (원할 경우) 라이브러리 크레이트가 포함될 수 있음.
2. 커진 프로젝트의 각 부분을 크레이트로 나눠서 외부 라이브러리처럼 쓸 수 있음.

모듈 시스템
-

1. 패키지: 크레이트를 빌드하고, 테스트하고, 공유하는 데 사용하는 카고 기능입니다.
2. 크레이트: 라이브러리나 실행 가능한 모듈로 구성된 트리 구조입니다.
3. 모듈과 use: 구조, 스코프를 제어하고, 조직 세부 경로를 감추는 데 사용합니다.
4. 경로: 구조체, 함수, 모듈 등의 이름을 지정합니다.
