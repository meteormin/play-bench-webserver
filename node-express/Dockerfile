# Node.js Express
FROM node:23-alpine
WORKDIR /app
COPY . .
# package.json이 이미 있다면 덮어쓰고 최신 express 설치
RUN rm -f package.json package-lock.json && \
    npm init -y && \
    npm install express@4.21.2
EXPOSE 8080
CMD ["node", "app.js"]
