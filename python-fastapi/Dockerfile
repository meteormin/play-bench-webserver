# Python FastAPI
FROM python:3.13-slim
WORKDIR /app
COPY . .
RUN pip install fastapi[standard]==0.115.8
EXPOSE 8080
CMD ["fastapi", "run", "main.py", "--port", "8080"]
