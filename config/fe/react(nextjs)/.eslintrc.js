/** @type {import('eslint').Linter.Config} */
module.exports = {
  // Next.js 권장 ESLint 설정 + 핵심 웹 바이탈 규칙
  extends: 'next/core-web-vitals',
  
  // 추가 플러그인
  plugins: [
    'unused-imports', // 사용하지 않는 import 제거
    'import'          // import 순서 및 구조 관리
  ],
  
  // 전역 변수 설정
  globals: {
    React: 'writable' // React 전역 사용 허용
  },
  
  rules: {
    // React 17+ JSX Transform 지원 (import React 불필요)
    'react/react-in-jsx-scope': 'off',
    
    // 사용하지 않는 import 자동 제거
    'unused-imports/no-unused-imports': 'error',
    
    // 파일 확장자 import 규칙
    'import/extensions': [
      'error',
      'ignorePackages',
      {
        js: 'optional',
        jsx: 'optional',
        ts: 'optional',
        tsx: 'optional'
      }
    ],
    
    // import 순서 규칙
    'import/order': [
      'error',
      {
        // import 그룹 순서 정의
        groups: [
          'builtin',           // Node.js 내장 모듈 (fs, path 등)
          'external',          // 외부 라이브러리 (react, next 등)
          'internal',          // 프로젝트 내부 모듈 (@/ 등)
          ['parent', 'sibling'], // 상위/동일 레벨 파일 (../, ./)
          'index',             // index 파일 (./index)
          'object',            // 객체 import (import { a } from 'module')
          'type',              // 타입 import (import type)
          'unknown'            // 기타
        ],
        
        // 특정 패턴에 대한 그룹 설정
        pathGroups: [
          {
            pattern: 'next',
            group: 'builtin',
            position: 'before'   // Next.js를 가장 먼저
          },
          {
            pattern: '**/*.tsx',
            group: 'unknown',
            position: 'after'    // TSX 파일을 마지막에
          },
          {
            pattern: '**/*.css.ts',
            group: 'unknown',
            position: 'after'    // CSS-in-JS 파일을 마지막에
          }
        ],
        
        // 그룹 간 빈 줄 강제
        'newlines-between': 'always',
        
        // 각 그룹 내에서 알파벳 순 정렬
        alphabetize: {
          order: 'asc',           // 오름차순
          caseInsensitive: true   // 대소문자 구분 안함
        }
      }
    ]
  }
}