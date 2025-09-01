## PR 작성 가이드

### PR 제목 형식

```
[티켓번호] <한줄 요약>
```

예시:
- `[PP-XXXX] 사용자 인증 시스템 추가`
- `[PP-XXXX] 결제 모듈 버그 수정`

### PR 설명 템플릿

```markdown
#### Issue Type
- [ ] 기능 추가 (feat)
- [ ] 기능 삭제 (feat)
- [ ] 버그 수정 (fix)
- [ ] 리팩토링 (refactor)
- [ ] 성능 개선 (perf)
- [ ] 의존성, 환경변수, 설정 파일 관련 업데이트 (chore)
- [ ] 스타일링 (style)
- [ ] 문서 수정 (docs)
- [ ] 테스트 코드 (test)

#### Priority
> Issue 우선순위를 표기, 우선순위 항목은 현재 JIRA `우선순위`를 기준 작성
- [ ] Blocker
- [ ] Urgent
- [ ] Critical
- [ ] Major
- [ ] Trivial

#### Background
> 이 PR에 어떤 작업이 포함 됐는지, 왜 해당 작업을 했는지에 대한 내용을 요약

#### Changes
> 주요 수정 사항을 기재. 리뷰어가 이해하기 어려울 것 같은 부분은 추가 코멘트를 작성

**API 변경사항:**
- [ ] Breaking Change 없음
- [ ] Breaking Change 있음 (하위 호환성에 영향)

**데이터베이스 변경사항:**
- [ ] 스키마 변경 없음
- [ ] 스키마 변경 있음 (마이그레이션 필요)
- [ ] 데이터 마이그레이션 필요

**주요 변경 파일:**
- `경로/파일명.ext` - 변경 내용 요약

#### Testing
> PR 요청 전 테스트 내용. 테스트 케이스를 간략히 목록으로 작성

**자동화 테스트:**
- [ ] 단위 테스트 통과
- [ ] 통합 테스트 통과
- [ ] E2E 테스트 통과
- [ ] 새로운 테스트 추가 (새 코드의 경우)
- [ ] 회귀 테스트 추가 (버그 수정의 경우)

**수동 테스트:**
- [ ] 로컬 환경에서 정상 동작 확인
- [ ] 개발 환경에서 정상 동작 확인
- [ ] 브라우저 호환성 확인 (해당하는 경우)
- [ ] 모바일 반응형 확인 (해당하는 경우)

**성능 테스트:**
- [ ] 성능 영향도 없음
- [ ] 성능 개선 확인됨
- [ ] 성능 저하 있음 (사유: )

#### Screenshots
> UI 변경사항이 있는 경우 Before/After 스크린샷 첨부

**Before:**
<!-- 변경 전 스크린샷 -->

**After:**
<!-- 변경 후 스크린샷 -->

#### Links
> 관련 문서, 작업 티켓, 디자인 가이드 문서 링크 추가
- [ ] JIRA Ticket: [PP-XXXX](https://ggnetwork.atlassian.net/browse/PP-XXXX)
- [ ] 관련 문서: [문서명](링크)
- [ ] 디자인 가이드: [Figma](링크)
- [ ] 관련 PR: #번호

#### Checklist
> PR 생성 전 필수 확인사항
- [ ] 자체 리뷰 완료
- [ ] 커밋 메시지가 컨벤션을 준수함
- [ ] 코드 컨벤션을 준수함
- [ ] 불필요한 콘솔 로그/주석 제거
- [ ] 비밀키나 민감정보 포함하지 않음
- [ ] 문서 업데이트 (필요한 경우)
- [ ] 의존성 업데이트 시 package-lock.json 포함
- [ ] Breaking Change 시 CHANGELOG 업데이트
```

### PR 생성 명령어

**GitHub CLI 사용 (권장):**
```bash
# upstream이 있는 경우: upstream/develop을 기본 타겟으로 PR 생성
gh pr create --base upstream/develop --title "[PP-XXXX] 작업 내용" --body-file .github/PULL_REQUEST_TEMPLATE.md

# upstream이 없는 경우: origin/develop을 타겟으로 PR 생성
gh pr create --base origin/develop --title "[PP-XXXX] 작업 내용" --body-file .github/PULL_REQUEST_TEMPLATE.md

# 자동으로 적절한 타겟 선택 (간단 버전)
gh pr create
```

**브랜치 전략:**
```bash
# upstream이 있는 경우
git fetch upstream
git checkout develop
git merge upstream/develop
git checkout -b feature/PP-XXXX-description
# 작업 완료 후
gh pr create --base upstream/develop

# upstream이 없는 경우 (origin만 있는 경우)
git fetch origin
git checkout develop
git merge origin/develop
git checkout -b feature/PP-XXXX-description
# 작업 완료 후
gh pr create --base origin/develop
```

### 기본 설정

**타겟 브랜치:**
- 우선순위: `upstream/develop` > `origin/develop`
- upstream이 설정된 경우: upstream/develop을 타겟으로 사용
- upstream이 없는 경우: origin/develop을 타겟으로 사용

**필수 확인사항:**

**코드 품질:**
- 파일 크기 제한: ≤ 300 LOC
- 함수 크기 제한: ≤ 50 LOC  
- 매개변수 제한: ≤ 5개
- 순환 복잡도: ≤ 10
- 제한 초과시 분할/리팩토링 필수

**보안 검사:**
- NEVER: 비밀값(패스워드/API키/토큰) 포함 금지
- NEVER: 민감한 데이터(PII/카드정보/SSN) 포함 금지
- NEVER: SQL 인젝션, XSS, CSRF 취약점 생성 금지
- ALWAYS: 모든 입력값 검증, 정규화, 인코딩
- ALWAYS: 매개변수화된 쿼리 사용
- ALWAYS: 인증/권한 확인 적용

**테스트 요구사항:**
- 새 코드 → 새 테스트 필수
- 버그 수정 → 회귀 테스트 필수
- 테스트는 먼저 실패하도록 작성 후 수정
- E2E 테스트: 성공/실패 경로 각각 ≥1개

**PR 크기 원칙:**
- 작업, 커밋, PR을 작게 유지
- 논리적 단위로 분리
- 독립적으로 빌드/테스트 가능해야 함

### PR 생성 전 체크리스트

**1. 코드 품질 확인**
```bash
# 린트 검사
npm run lint
# 또는
yarn lint

# 타입 체크
npm run type-check
# 또는
yarn type-check

# 테스트 실행
npm test
# 또는
yarn test
```

**2. 필수 확인사항**
- [ ] **브랜치 확인**: 현재 브랜치가 feature 브랜치인지 확인
- [ ] **최신화**: 타겟 브랜치(upstream/develop 또는 origin/develop) 최신 변경사항 반영
- [ ] **커밋 정리**: 불필요한 커밋은 squash 하여 정리
- [ ] **컨플릭트 해결**: merge conflict가 없는지 확인
- [ ] **테스트 실행**: 모든 테스트가 통과하는지 확인
- [ ] **문서 업데이트**: 필요시 관련 문서도 함께 수정
- [ ] **빌드 확인**: 빌드가 성공하는지 확인

**3. 보안 체크**
- [ ] 비밀키, API 키, 토큰 등 민감정보 미포함
- [ ] 개발용 디버그 코드 제거
- [ ] 콘솔 로그 정리

### 리뷰 가이드라인

**리뷰어 관점:**
- [ ] **기능성**: 요구사항을 올바르게 구현했는가?
- [ ] **코드 품질**: 가독성, 유지보수성이 좋은가?
- [ ] **설계**: 적절한 아키텍처와 패턴을 사용했는가?
- [ ] **보안**: 보안 취약점은 없는가?
- [ ] **성능**: 성능에 부정적 영향은 없는가?
- [ ] **테스트**: 적절한 테스트 커버리지를 가지고 있는가?
- [ ] **문서**: 필요한 문서화가 되어있는가?

**작성자 관점:**
- [ ] **자체 리뷰**: PR 생성 전 자체 리뷰 완료
- [ ] **컨텍스트 제공**: 변경 이유와 의도 명확히 설명
- [ ] **리뷰 대응**: 피드백에 24시간 내 응답
- [ ] **변경 반영**: 요청된 수정사항 빠르게 반영
- [ ] **CI/CD**: 모든 자동화 검사 통과

### 자주 사용하는 명령어

**PR 상태 확인:**
```bash
# PR 목록 조회
gh pr list

# 특정 PR 상세 정보
gh pr view <PR번호>

# PR 체크아웃 (리뷰용)
gh pr checkout <PR번호>
```

**PR 업데이트:**
```bash
# PR 제목/설명 수정
gh pr edit <PR번호> --title "새 제목" --body "새 설명"

# Draft PR을 Ready로 변경
gh pr ready <PR번호>

# PR 병합
gh pr merge <PR번호> --squash
```

### PR 작성 언어 가이드

**한글로 작성하되 기술명은 영어 유지**
- 설명, 요약: 한글 작성
- 기술 용어: 영어 유지 (API, database, migration, refactoring 등)
- 예시: "API 응답 속도 개선을 위한 caching 로직 추가"