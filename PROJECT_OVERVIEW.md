# Nice Hand Core - 프로젝트 개요

## 프로젝트 소개

Nice Hand Core는 Rust로 구현된 고성능 텍사스 홀덤 포커 AI 라이브러리입니다. CFR+ 알고리즘, 포괄적인 토너먼트 지원, 정확한 EV 계산을 통해 프로덕션 환경에서 사용할 수 있는 완전한 포커 AI 솔루션을 제공합니다.

## 핵심 기능

### 1. 고급 CFR+ 알고리즘
- **CFR+ 구현**: 음수 후회값 제거로 향상된 수렴 성능
- **몬테카를로 CFR**: 샘플링 최적화를 통한 계산 효율성
- **무한 재귀 방지**: 깊이 추적으로 안정적인 학습
- **실시간 성능**: 평균 3.45μs 응답 시간

### 2. 포괄적 토너먼트 지원
- **ICM 계산기**: 독립 칩 모델을 통한 정확한 에퀴티 계산
- **버블 전략**: 토너먼트 버블 상황 특화 전략
- **MTT 관리**: 멀티테이블 토너먼트 및 테이블 밸런싱
- **블라인드 구조**: 다양한 토너먼트 형식 지원

### 3. 정밀한 EV 계산
- **액션별 기댓값**: 폴드/콜/레이즈 각각의 정확한 EV
- **핸드 레인지 분석**: 상대방 레인지 대비 에퀴티 계산
- **실시간 분석**: 마이크로초 단위 빠른 계산
- **상황별 최적화**: 프리플랍/포스트플랍 전용 알고리즘

### 4. 웹 API 인터페이스
- **무상태 API**: 즉석 휴리스틱 기반 전략 제공
- **상태 유지 API**: CFR 학습 모델 기반 고급 전략
- **배치 처리**: 고처리량 다중 요청 처리
- **실시간 응답**: 초당 289,603 요청 처리 능력

## 기술적 우수성

### 성능 지표
- **학습 속도**: CFR+ 50회 반복에 617ms
- **API 처리량**: 초당 289,603 요청
- **메모리 효율**: 14,005+ 노드 안정적 관리
- **ICM 계산**: 마이크로초 단위 초고속 처리

### 품질 보증
- **54개 단위 테스트**: 모든 테스트 통과 (토너먼트 7개 포함)
- **메모리 안전성**: Rust 소유권 시스템으로 메모리 보안
- **무한 재귀 해결**: 깊이 제한으로 완전한 안정성
- **포괄적 문서화**: API 및 사용자 가이드 완비

## 프로젝트 구조

```
src/
├── api/                    # 웹 API 계층
│   ├── web_api.rs         # CFR 기반 상태 유지 API
│   └── web_api_simple.rs  # 휴리스틱 기반 무상태 API
├── game/                   # 게임 로직 계층
│   ├── holdem.rs          # 기본 홀덤 게임
│   ├── tournament_holdem.rs # 토너먼트 홀덤
│   ├── tournament.rs      # 토너먼트 관리 (ICM, 버블 전략)
│   ├── hand_eval.rs       # 핸드 평가
│   └── card_abstraction.rs # 카드 추상화
└── solver/                 # AI 알고리즘 계층
    ├── cfr_core.rs        # CFR+ 핵심 알고리즘
    ├── mccfr.rs           # 몬테카를로 CFR
    └── ev_calculator.rs   # 기댓값 계산기
```

## 사용 사례

### 1. 포커 학습 및 훈련
```rust
use nice_hand_core::solver::*;

let mut trainer = Trainer::new();
trainer.run(vec![holdem::State::new()], 1000);
println!("{}개의 노드로 학습 완료", trainer.nodes.len());
```

### 2. 실시간 포커 어시스턴트
```rust
use nice_hand_core::api::QuickPokerAPI;

let api = QuickPokerAPI::new();
let strategy = api.get_optimal_strategy(game_state);
println!("추천: {} (EV: {:.2})", strategy.recommended_action, strategy.expected_value);
```

### 3. 토너먼트 분석
```rust
use nice_hand_core::game::tournament::ICMCalculator;

let icm = ICMCalculator::new();
let equity = icm.calculate_icm(&stacks, &payouts);
println!("ICM 에퀴티: {:.2}%", equity[player_id] * 100.0);
```

## 비즈니스 응용

### 상업적 활용
- **온라인 포커 플랫폼**: 봇 통합 및 게임 백엔드
- **포커 훈련 소프트웨어**: 전문가급 학습 도구
- **토너먼트 플랫폼**: ICM 기반 토너먼트 관리
- **게임 분석 도구**: 핸드 히스토리 분석
- **연구 플랫폼**: 학술 포커 AI 연구

### 기술적 통합
- **REST API**: 웹 애플리케이션 통합
- **마이크로서비스**: 컨테이너화된 배포
- **WASM 지원**: 브라우저 기반 애플리케이션
- **모바일 앱**: 크로스 플랫폼 포커 앱

## 예제 프로그램 (37개)

### 핵심 알고리즘
- `examples/simple_test.rs` - 기본 CFR 검증
- `examples/mccfr_demo.rs` - 몬테카를로 CFR 데모
- `examples/ev_calculator_demo.rs` - EV 계산기 사용법

### 토너먼트 전용
- `examples/tournament_demo.rs` - 기본 토너먼트 기능
- `examples/icm_pressure_analysis.rs` - ICM 압박 분석
- `examples/bubble_strategy_optimization.rs` - 버블 전략
- `examples/mtt_demo_extended.rs` - 멀티테이블 관리

### 성능 및 분석
- `examples/benchmark.rs` - 성능 벤치마크
- `examples/heuristic_demo.rs` - 휴리스틱 전략
- `examples/web_demo.rs` - 웹 API 데모

## 설치 및 사용

### 빠른 시작
```bash
git clone https://github.com/your-repo/nice-hand-core.git
cd nice-hand-core
cargo build --release
cargo run --example simple_test
```

### 예제 실행
```bash
# 기본 CFR 데모
cargo run --example simple_test

# 토너먼트 데모
cargo run --example tournament_demo

# 웹 API 데모
cargo run --example web_demo

# 성능 벤치마크
cargo run --example benchmark --release
```

## 프로젝트 상태

### 완료된 기능 (2025년 6월 기준)
- ✅ CFR+ 알고리즘 완전 구현
- ✅ 토너먼트 지원 (ICM, 버블, MTT)
- ✅ EV 계산기 시스템
- ✅ 웹 API (무상태/상태유지)
- ✅ 54개 테스트 모두 통과
- ✅ 포괄적 문서화

### 향후 계획
- 🎯 고급 AI 기능 (상대방 모델링)
- 🎯 웹 통합 (WASM, 멀티플레이어)
- 🎯 성능 최적화 (GPU 가속)
- 🎯 상용화 준비

## 기여 및 커뮤니티

### 개발 참여
- **이슈 리포팅**: 버그 및 기능 요청
- **코드 기여**: 풀 리퀘스트 환영
- **문서 개선**: 가이드 및 예제 추가
- **테스트 작성**: 커버리지 확장

### 라이선스
MIT 라이선스 하에 오픈소스로 제공되며, 상업적 사용 가능합니다.

---

**Nice Hand Core는 현재 프로덕션 환경에서 사용할 준비가 완료된 성숙한 포커 AI 라이브러리입니다.**
