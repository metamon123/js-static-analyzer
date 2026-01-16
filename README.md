# JS Static Analyzer

![CI](https://github.com/metamon123/js-static-analyzer/actions/workflows/ci.yml/badge.svg)

JavaScript 소스 파일을 읽어 간단한 정적 분석을 수행하는 Rust CLI 도구입니다.

## 빠른 시작
```bash
cargo build
cargo run -- --file example.js
```

## 분석 포인트

### 함수/변수/임포트 개수 집계

예시:

```text
JavaScript AST Analysis:
========================
Functions: 1
Variables: 2
Imports: 1
```

## 프로젝트 구조
- `src/main.rs`: CLI 진입점 및 분석 로직
- `example.js`: 샘플 입력 파일

## 개발 동기

주 목표:
> 세상에 javascript 기반 프로그램이 정말 많다. 한 번 정적 분석으로 통계/보안적으로 재밌는 인사이트를 찾아보자.

부가 목표:
- Rust와 SWC를 활용한 AST 탐색 방식을 경험해보기
- 정적 분석 툴을 직접 만들어보며 장단점을 느껴보기

## 로드맵
- AST 순회를 통한 더 많은 메트릭 제공(예: 클래스/함수 중첩)
- 결과를 JSON으로 출력하는 `--format json` 옵션
- 디렉터리 단위 분석 및 리포트 저장
- 공급망 분석과 연결짓기 (e.g., package.json 등이 같이 제공되는 경우, 취약한 라이브러리를 찾고 해당 라이브러리를 vulnerable sink 로 잡고 분석해보기)

## README 업데이트 체크리스트
- 프로젝트 목적과 문제 정의가 최신인가?
- 핵심 기능과 데모 출력이 실제 동작과 일치하는가?
- 로드맵 항목이 완료/변경 사항을 반영하는가?
- 기술 스택과 버전 정보가 최신인가?
- CI 배지가 정상 동작하는가?

## 키워드, 스택
- Rust
  - swc_ecma_parser / swc_common
- Static analysis

## 라이선스
MIT License