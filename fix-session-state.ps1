// Fix session-state.ts syntax error
 = Get-Content 'src/lib/session/session-state.ts' -Raw
 =  -replace 'import \{ invoke \} from ''@/lib/tauri-wrapper'';\s*import type \{\s*SessionState,\s*SessionRestoreResult,\s*UiPreferences,\s*RecentAction,\s*\} from ''@/types/session'';\s*// Type declarations for browser APIs\s*declare const setTimeout', "import { invoke } from '@/lib/tauri-wrapper';
import type {
  SessionState,
  SessionRestoreResult,
  UiPreferences,
  RecentAction,
} from '@/types/session';

// Type declarations for browser APIs
declare const setTimeout"
Set-Content 'src/lib/session/session-state.ts'  -NoNewline -Encoding UTF8
