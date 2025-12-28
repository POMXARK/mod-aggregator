// Set environment variables before importing client
process.env.TAURI_MCP_CONNECTION_TYPE = 'tcp';
process.env.TAURI_MCP_TCP_HOST = '127.0.0.1';
process.env.TAURI_MCP_TCP_PORT = '4000';

// Use dynamic import to ensure env vars are set before client creation
const { socketClient } = await import('./tauri-plugin-mcp/mcp-server-ts/build/tools/client.js');

console.log('=== Testing MCP Connection (TCP Mode) ===\n');
console.log('Environment variables:');
console.log(`  TAURI_MCP_CONNECTION_TYPE = ${process.env.TAURI_MCP_CONNECTION_TYPE}`);
console.log(`  TAURI_MCP_TCP_HOST = ${process.env.TAURI_MCP_TCP_HOST}`);
console.log(`  TAURI_MCP_TCP_PORT = ${process.env.TAURI_MCP_TCP_PORT}`);
console.log('\nConnecting to Tauri MCP via TCP (127.0.0.1:4000)...\n');

try {
  await socketClient.connect();
  console.log('✓ SUCCESS! Connected to MCP socket\n');
  
  console.log('Testing ping command...');
  const result = await socketClient.sendCommand('ping', {});
  console.log('✓ Ping successful!');
  console.log('Result:', JSON.stringify(result, null, 2));
  
  console.log('\n=== MCP Connection: WORKING ===');
  process.exit(0);
} catch (error) {
  console.error('✗ Connection failed:', error.message);
  if (error.message.includes('ECONNREFUSED')) {
    console.error('\n→ Connection refused. Possible reasons:');
    console.error('  1. Tauri app not running');
    console.error('  2. TCP server not started (check logs)');
    console.error('  3. Wrong port (should be 4000)');
    console.error('  4. App needs restart to use TCP mode');
  } else if (error.message.includes('ENOENT')) {
    console.error('\n→ Still using IPC mode. Check environment variables.');
  }
  process.exit(1);
}

