# Go Gin
FROM golang:1.24-alpine AS builder
WORKDIR /app
COPY . .
# Gin 1.10.0 is the latest stable
RUN rm -f go.mod go.sum && \
    go mod init todo-api-bench/go-gin && \
    go get github.com/gin-gonic/gin@v1.10.0 && \
    go mod tidy && \
    go build -o main .

FROM alpine:latest
WORKDIR /root/
COPY --from=builder /app/main .
EXPOSE 8080
CMD ["./main"]
