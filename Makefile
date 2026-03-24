.PHONY: build clean run

# 컴파일 후 bin 폴더로 실행 파일을 복사합니다.
build:
	@echo "Building benchmark-tool (release)..."
	cd benchmark-tool && cargo build --release
	@echo "Copying binary to bin/..."
	mkdir -p bin
	cp benchmark-tool/target/release/benchmark-tool bin/
	@echo "Done! You can run it with: ./bin/benchmark-tool"

# 빌드 결과물 및 bin 폴더를 정리합니다.
clean:
	@echo "Cleaning artifacts..."
	cd benchmark-tool && cargo clean
	rm -rf bin/
	@echo "Cleaned."

# 빌드 및 벤치마클 실행 (기본 경로 . 사용)
run: build
	./bin/benchmark-tool --path .
