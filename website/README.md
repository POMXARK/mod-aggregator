# Mod Aggregator Documentation

Документация проекта на Docusaurus.

## Разработка

```bash
# Установка зависимостей
npm install

# Запуск в режиме разработки
npm start

# Сборка
npm run build

# Запуск собранного сайта
npm run serve
```

## Docker

### Production (собранный сайт)

```bash
# Сборка образа
docker build -t mod-aggregator-docs .

# Запуск через Docker Compose
docker-compose up -d

# Или напрямую
docker run -p 3000:3000 mod-aggregator-docs
```

Документация будет доступна по адресу http://localhost:3000

### Development (с hot reload)

```bash
# Запуск в режиме разработки
docker-compose -f docker-compose.dev.yml up -d
```

Изменения в файлах будут автоматически применяться.

## Структура

- `docs/` - исходные Markdown файлы документации
- `src/` - React компоненты и стили
- `static/` - статические файлы (изображения, favicon)
- `docusaurus.config.ts` - конфигурация Docusaurus
- `sidebars.ts` - конфигурация боковой панели

