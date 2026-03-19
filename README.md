# TODO API Benchmarking Project

이 프로젝트는 다양한 언어와 프레임워크를 사용하여 구현된 간단한 TODO API의 성능, 메모리 점유율, CPU 사용량 및 디스크 공간(이미지 크기)을 비교하기 위한 벤치마크 도구입니다.

## 대상 프레임워크
- **Go**: Gin
- **Node.js**: Express
- **Python**: FastAPI
- **Java**: Spring Boot
- **PHP**: Laravel 11 (API)

## 기능
각 API는 다음과 같은 동일한 RESTful 엔드포인트를 구현합니다.
- `GET /todos`: 전체 할 일 목록 조회
- `POST /todos`: 새로운 할 일 추가
- `GET /todos/:id`: 특정 할 일 조회
- `PUT /todos/:id`: 할 일 수정
- `DELETE /todos/:id`: 할 일 삭제

## 프로젝트 구조
```text
todo-api-bench/
├── go-gin/            # Go Gin 구현 및 Dockerfile
├── node-express/      # Node.js Express 구현 및 Dockerfile
├── python-fastapi/    # Python FastAPI 구현 및 Dockerfile
├── java-spring/       # Java Spring Boot 구현 및 Dockerfile
├── php-laravel/       # PHP 구현 및 Dockerfile
├── benchmark-tool/    # Rust 기반 벤치마크 툴 (src/main.rs)
└── docker-compose.yml # 컨테이너 오케스트레이션 설정
```

## 벤치마크 항목
1. **RPS (Requests Per Second)**: 초당 요청 처리 수
2. **CPU %**: 요청 처리 중의 CPU 사용률
3. **Memory Usage**: 컨테이너의 메모리 사용량
4. **Image Size**: 빌드된 Docker 이미지의 크기

## 시작하기

### 사전 요구 사항
- [Docker](https://www.docker.com/) 및 Docker Compose
- [Rust](https://www.rust-lang.org/) (벤치마크 툴 실행용)

### 실행 방법
1. 프로젝트 루트로 이동합니다.
2. 벤치마크 툴을 빌드하고 실행합니다.
   ```bash
   cd benchmark-tool
   cargo run
   ```

### 벤치마크 과정
- 툴은 `docker-compose.yml`을 사용하여 모든 이미지를 빌드합니다.
- 각 서비스를 하나씩 실행하며 10초간의 웜업(Warm-up) 시간을 가집니다.
- `curl`을 통해 수백 개의 요청을 보내 평균 처리 속도를 계산합니다.
- `docker stats`를 사용하여 리소스 사용량을 측정합니다.
- 모든 측정이 끝나면 비교 결과가 테이블 형태로 출력됩니다.
