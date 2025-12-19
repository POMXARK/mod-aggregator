# Установка Docusaurus

## Быстрая установка

```bash
cd website
npm install
```

## Проверка установки

```bash
npm start
```

Документация должна открыться в браузере по адресу http://localhost:3000

## Проблемы при установке

### Ошибка с зависимостями

Если возникают проблемы с зависимостями:

```bash
rm -rf node_modules package-lock.json
npm install
```

### Проблемы с портом 3000

Если порт 3000 занят, можно изменить порт:

```bash
npm start -- --port 3001
```

Или в `docusaurus.config.ts`:

```typescript
presets: [
  [
    'classic',
    {
      // ...
    },
  ],
],
server: {
  port: 3001,
},
```

