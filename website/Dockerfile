# Многоэтапная сборка для оптимизации размера образа
FROM node:18-alpine AS builder

# Установка рабочей директории
WORKDIR /app

# Копирование файлов package.json и package-lock.json
COPY package*.json ./

# Установка зависимостей
RUN npm ci

# Копирование остальных файлов
COPY . .

# Сборка статического сайта
RUN npm run build

# Production образ
FROM node:18-alpine

WORKDIR /app

# Копирование только собранных файлов
COPY --from=builder /app/build ./build
COPY --from=builder /app/package*.json ./

# Установка serve для статических файлов
RUN npm install -g serve

# Открытие порта
EXPOSE 3000

# Запуск сервера
CMD ["serve", "-s", "build", "-l", "3000", "--host", "0.0.0.0"]

