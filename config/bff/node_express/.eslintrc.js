module.exports = {
  // 기본 설정 확장 (현재 비어있음)
  extends: '',
  
  // 사용할 ESLint 플러그인들
  plugins: [
    'unused-imports', // 사용하지 않는 import 제거
    'import'          // import/export 관련 규칙
  ],
  
  // TypeScript 파서 설정
  parser: '@typescript-eslint/parser',
  
  // 린트 규칙 설정
  rules: {
    // 사용하지 않는 import를 에러로 처리
    'unused-imports/no-unused-imports': 'error',
    
    // import 순서를 강제하는 규칙
    'import/order': [
      'error',
      {
        // import 그룹 순서 정의
        groups: [
          'builtin',           // Node.js 내장 모듈 (fs, path 등)
          'external',          // node_modules의 외부 패키지
          'internal',          // 프로젝트 내부 절대 경로
          ['parent', 'sibling'], // 상위/형제 디렉토리 상대 경로
          'index',             // index 파일
          'object',            // object imports
          'type',              // TypeScript type imports
          'unknown'            // 기타
        ],
        
        // 특정 패턴에 대한 그룹 설정
        pathGroups: [
          {
            pattern: '**/*.ts',
            group: 'unknown',
            position: 'after'
          }
        ],
        
        // 그룹 사이에 빈 줄 강제
        'newlines-between': 'always',
        
        // 알파벳순 정렬 설정
        alphabetize: {
          order: 'asc',           // 오름차순 정렬
          caseInsensitive: true   // 대소문자 구분 안함
        }
      }
    ]
  }
};