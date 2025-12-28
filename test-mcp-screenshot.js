// Set environment variables before importing client
process.env.TAURI_MCP_CONNECTION_TYPE = 'tcp';
process.env.TAURI_MCP_TCP_HOST = '127.0.0.1';
process.env.TAURI_MCP_TCP_PORT = '4000';

// Use dynamic import to ensure env vars are set before client creation
const { socketClient } = await import('./tauri-plugin-mcp/mcp-server-ts/build/tools/client.js');

console.log('=== Testing MCP Screenshot Tool ===\n');

try {
  await socketClient.connect();
  console.log('✓ Connected to MCP socket\n');
  
  console.log('Taking screenshot of main window...');
  const result = await socketClient.sendCommand('take_screenshot', { window_label: 'main' });
  
  if (result && result.data_url) {
    console.log('✓ Screenshot captured successfully!');
    console.log(`  Image size: ${result.data_url.length} characters`);
    console.log(`  Format: ${result.data_url.substring(0, 30)}...`);
    console.log('\nScreenshot data available in result.data_url');
  } else {
    console.log('Result:', JSON.stringify(result, null, 2));
  }
  
  console.log('\n=== Testing get_dom Tool ===\n');
  const domResult = await socketClient.sendCommand('get_dom', { window_label: 'main' });
  
  if (domResult && domResult.dom) {
    console.log('✓ DOM retrieved successfully!');
    console.log(`  DOM length: ${domResult.dom.length} characters`);
    console.log(`  Preview: ${domResult.dom.substring(0, 200)}...`);
  } else {
    console.log('DOM Result:', JSON.stringify(domResult, null, 2));
  }
  
  process.exit(0);
} catch (error) {
  console.error('✗ Error:', error.message);
  if (error.message.includes('ECONNREFUSED')) {
    console.error('\n→ Connection refused. Make sure Tauri app is running with TCP server.');
  }
  process.exit(1);
}


















