// Quick DOM structure check
const resizeHandles = document.querySelectorAll('.resize-handle');
console.log('Total resize handles found:', resizeHandles.length);

const chatPanel = document.querySelector('.chat-panel');
console.log('Chat panel found:', !!chatPanel);

const nodeEditor = document.querySelector('.node-editor');
console.log('Node editor found:', !!nodeEditor);

// Check structure
const parserWorkspace = document.querySelector('.parser-workspace');
if (parserWorkspace) {
  const children = Array.from(parserWorkspace.children).map(child => ({
    tag: child.tagName,
    classes: child.className,
    type: child.getAttribute('role') || 'div'
  }));
  console.log('ParserWorkspace children:', children);
}