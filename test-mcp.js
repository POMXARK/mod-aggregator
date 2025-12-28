import { socketClient } from './tauri-plugin-mcp/mcp-server-ts/build/tools/client.js';

console.log('=== MCP Connection Test ===\n');
console.log('Connecting to Tauri socket...');

socketClient.connect()
  .then(() => {
    console.log('✓ Connected successfully!');
    console.log('\nTesting ping command...');
    return socketClient.sendCommand('ping', {});
  })
  .then((result) => {
    console.log('✓ Ping result:', JSON.stringify(result, null, 2));
    console.log('\n=== Connection Test: SUCCESS ===');
    process.exit(0);
  })
  .catch((err) => {
    console.error('✗ Connection failed:', err.message);
    if (err.message.includes('ENOENT') || err.code === 'ENOENT') {
      console.error('\n→ Tauri app is not running or MCP plugin not started');
      console.error('  Start Tauri app with: npm run tauri:dev');
      console.error('  Check logs for: "Development build detected, enabling MCP plugin"');
    } else if (err.message.includes('ECONNREFUSED')) {
      console.error('\n→ Connection refused - socket server not running');
    } else {
      console.error('\n→ Error details:', err);
    }
    process.exit(1);
  });

