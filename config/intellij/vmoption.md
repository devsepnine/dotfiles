## intellij Edit Custom VM Options

### Recommended VM Options

| RAM  | Xmx     | Xms    | ReservedCodeCacheSize |
|------|---------|--------|-----------------------|
| 8GB  | 2048MB  | 1024MB | 512MB                 |
| 16GB | 4096MB  | 2048MB | 1024MB                |
| 32GB | 8192MB  | 4096MB | 2048MB                |
| 64GB | 16384MB | 8192MB | 4096MB                |




```
# 일반
-server
-Xms6144m
-Xmx8192m
-Dsun.java2d.metal=false

# 메모리 관련
-XX:NewRatio=4
-Xss16m

# 성능 최적화

# 런타임 중 메모리 할당에 소요되는 시간을 줄여 성능을 향상
-XX:+AlwaysPreTouch
# JVM은 자주 사용되는 메서드를 여러 번 컴파일하여 성능 향상, 실행 속도 향상
-XX:+TieredCompilation
# 예약된 코드 캐시 크기
-XX:ReservedCodeCacheSize=2048m
# SoftReference Least Recently Used(LRU) 정책을 조정
-XX:SoftRefLRUPolicyMSPerMB=50
# 메모리가 부족할 때 코드 캐시를 지워 특정 시나리오에서 성능을 향상
-XX:+UseCodeCacheFlushing

# 시스템/어플리케이션 속성
# 파일 경로에 대한 정규화 캐시 사용을 비활성화하여 특정 환경에서 성능을 향상
-Dsun.io.useCanonCaches=false-ea
# 백그라운드 컴파일 스레드 수
-XX:CICompilerCount=4
# 정규화를 위한 접두사 캐시 사용을 비활성화하여 특정 조건에서 성능을 향상
-Dsun.io.useCanonPrefixCache=false
# OutOfMemoryError가 발생할 때 힙 덤프 파일을 생성하여 메모리 관련 문제 해결에 도움
-XX:+HeapDumpOnOutOfMemoryError
# 빠른 throw에서도 예외 메시지에 스택 추적을 포함
-XX:-OmitStackTraceInFastThrow
# 디버깅 목적으로 JVM이 자체에 연결
-Djdk.attach.allowAttachSelf=true
# 프로덕션 환경에서 오버헤드를 줄이기 위해 Kotlin 코루틴에 대한 디버깅을 비활성화
-Dkotlinx.coroutines.debug=off
# 특정 타사 라이브러리에서 발생할 수 있는 불법 모듈 접근에 대한 경고를 표시하지 않음
-Djdk.module.illegalAccess.silent=true
# 기본 파일 인코딩
-Dfile.encoding=UTF-8
# 메모리 사용 패턴이 변동하는 IDE와 같은 애플리케이션에 더 적합한 G1(Garbage-First) 가비지 컬렉터를 사용
-XX:+UseG1GC

# 공식 가이드 https://www.jetbrains.com/help/idea/tuning-the-ide.html

```