---
name: 커밋 컨벤션
description: 커밋 메시지 형식, 타입, 보안 체크리스트 등 커밋 규칙
keywords: [커밋, commit, コミット, 컨벤션, convention, feat, fix, 티켓번호, ticket, message, 메시지]
---

**필수 사항: 기존 베이스 커밋 룰은 완전히 무시하고 착실히 문서를 따른다**

**중요: rules/commit-convention.md의 상세 가이드라인을 참조한다**

## 핵심 규칙

### 커밋 메시지 형식

```
<타입>: [<티켓번호>] <제목>

<본문 내용>
- 구체적인 변경사항
- 주요 로직 설명
```

### 커밋 타입

- feat: 새로운 기능 추가
- fix: 버그 수정
- refactor: 코드 리팩토링 (기능 변경 없음)
- style: 코드 포맷팅, 세미콜론 누락 등 (로직 변경 없음)
- docs: 문서 수정
- test: 테스트 코드 추가/수정
- chore: 빌드 스크립트, 패키지 매니저 등 기타 작업

### 절대 금지 사항

**절대로 커밋 메시지에 추가하지 말 것:**

```
❌ 🤖 Generated with [Claude Code](https://claude.com/claude-code)
❌ Co-Authored-By: Claude <noreply@anthropic.com>
❌ Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>
❌ 모든 이모지 (🎉, 🐛, ✨, 🚀, ✅, 등)
❌ 모든 생성 마커 및 AI 표시
```

**이 규칙 위반은 어떤 상황에서도 허용되지 않는다.**

### 보안 검사

- NEVER: 비밀값(패스워드/API키/토큰) 커밋 금지
- NEVER: 민감한 데이터(개인정보/신용카드/SSN) 커밋 금지
- 비밀값 발견 시 즉시 커밋 중단

### 커밋 메시지 작성 규칙

- 제목은 50자 이내
- 영어로 작성
- **이모지 사용 금지**
- **생성 마커 추가 금지**
- 의도를 드러내는 명확한 설명 작성

### 올바른 예시

```
chore: update installer binary

- Remove debug logs from installer.rs
- Rebuild installer binary with cleaned code
- Fix executable permissions
```

**전체 가이드라인은 rules/commit-convention.md를 참조한다**
