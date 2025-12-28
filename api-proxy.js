// Простой HTTP прокси для API ErkaPharm
const http = require('http');
const https = require('https');

const PORT = 3000;
const TARGET_HOST = 'localhost';
const TARGET_PORT = 9443;
const API_HOST = 'api.erkapharm.com';

console.log('=== API Proxy Server ===');
console.log(`Proxy: http://localhost:${PORT} -> https://${TARGET_HOST}:${TARGET_PORT}`);
console.log(`Host header: ${API_HOST}`);

const server = http.createServer((req, res) => {
    console.log(`${new Date().toISOString()} - ${req.method} ${req.url}`);

    // Создаем запрос к целевому серверу
    const options = {
        hostname: TARGET_HOST,
        port: TARGET_PORT,
        path: req.url,
        method: req.method,
        headers: {
            ...req.headers,
            'Host': API_HOST  // Подменяем Host header
        },
        rejectUnauthorized: false  // Игнорируем SSL ошибки
    };

    const proxyReq = https.request(options, (proxyRes) => {
        // Копируем заголовки ответа
        res.writeHead(proxyRes.statusCode, proxyRes.headers);

        // Передаем данные
        proxyRes.pipe(res);
    });

    proxyReq.on('error', (err) => {
        console.error('Proxy request error:', err.message);
        res.writeHead(500, { 'Content-Type': 'text/plain' });
        res.end('Proxy error: ' + err.message);
    });

    // Передаем тело запроса
    req.pipe(proxyReq);
});

server.listen(PORT, () => {
    console.log(`Proxy server listening on http://localhost:${PORT}`);
    console.log('Press Ctrl+C to stop');
});

// Graceful shutdown
process.on('SIGINT', () => {
    console.log('\nShutting down proxy server...');
    server.close(() => {
        console.log('Proxy server stopped');
        process.exit(0);
    });
});
























