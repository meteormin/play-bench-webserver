# Node.js Express
FROM node:23-alpine
WORKDIR /app
COPY . .
RUN npm install --production
EXPOSE 8080
CMD ["node", "app.js"]
