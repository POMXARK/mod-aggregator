<!--
Sync Impact Report:
- Version: 0.0.0 → 1.0.0 (Initial constitution creation)
- Principles: 7 core principles defined
- Sections: Technical Stack, Development Workflow, AI Agent Rules, Project-Specific Constraints
- Templates: ⚠ Requires review of plan-template.md, spec-template.md, tasks-template.md
- Follow-up: None
-->

# Mod Aggregator Constitution

## Core Principles

### I. Component-First Architecture
Every feature starts as a standalone, reusable component; Components must be self-contained, independently testable, and documented; Clear purpose required - no organizational-only components. This applies to both Svelte frontend components and Rust backend modules.

### II. Type Safety (NON-NEGOTIABLE)
TypeScript for frontend and Rust for backend - strict typing mandatory; All functions must have proper type annotations; No `any` types without explicit justification; Type safety ensures reliability and maintainability of the universal file aggregator system.

### III. Test-Driven Development (TDD)
TDD where applicable: Tests written → User approved → Tests fail → Then implement; Red-Green-Refactor cycle encouraged; Frontend: Vitest + Testing Library; Backend: Standard Rust tests; Critical parsing logic MUST have tests before implementation.

### IV. Separation of Concerns
Clear separation between Frontend (Svelte 5) and Backend (Rust/Tauri); Frontend handles UI/UX, visual parser builder, node editor, and browser integration; Backend handles business logic, parsing engine, database operations, and file system operations; Communication via Tauri commands only.

### V. Universal Extensibility
The system must support universal architectural blocks with UI for adding new functionality; Game-specific logic (Sims 4, Skyrim, Minecraft, RimWorld, Fallout 4, etc.) must be pluggable, not hardcoded; Parser logic must be configurable through visual node editor; Installation methods must be extensible per game type.

### VI. Documentation & Communication (NON-NEGOTIABLE)
All comments and documentation in Russian language; Every function must have JSDoc (TypeScript) or documentation (Rust); README files in each directory describe component architecture; Document "why" not just "what"; This is critical for AI agent collaboration and future maintenance.

### VII. Proactive Problem-Solving
Always propose multiple improvement options (minimum 2-3 variants); Be proactive and suggest concrete actions; Use format: "Вариант 1: ...", "Вариант 2: ...", "Вариант 3: ..."; After task completion, suggest additional improvements; This principle applies to both AI agents and human developers.

## Technical Stack

### Frontend
- **Framework**: Svelte 5 with TypeScript (NON-NEGOTIABLE)
- **Styling**: Tailwind CSS + custom styles
- **Node Graph**: @xyflow/svelte for visual parser builder and node editor
- **State Management**: Svelte 5 runes ($state, $derived, $effect) - NO legacy reactivity API
- **Browser Integration**: Embedded browser for visual selector picking

### Backend
- **Framework**: Tauri 2.0 for desktop applications (NON-NEGOTIABLE)
- **Database**: SQLite via SQLx for persistence
- **Parsing**: scraper crate for HTML parsing
- **HTTP**: reqwest for network requests
- **File Operations**: Tauri FS plugin for file system access

### Architecture Patterns
- **Parser Engine**: Visual node-based parser builder with executable graph
- **Page Caching**: Local page storage with versioning for offline parser development
- **Mod Management**: Universal mod installation system with game-specific plugins
- **Collection System**: Symlink-based mod collections (modpacks) for easy sharing

## Development Workflow

### Code Quality Standards
- Follow component-based approach for all UI elements
- Ensure type safety at all levels (TypeScript strict mode, Rust without unsafe)
- Isolate styles in components using Tailwind utility classes
- Use composables for reusable frontend logic
- Modular Rust code with clear separation of concerns

### Testing Strategy
- **Unit Tests**: Critical logic MUST have unit tests (parser nodes, data transformations)
- **Integration Tests**: Tauri commands, database operations, file system operations
- **E2E Tests**: User flows (Cypress) for critical paths (parser creation, mod installation)
- **Frontend**: Vitest + Testing Library for component testing
- **Backend**: Standard Rust test framework with async support

### Search & Research Protocol
**CRITICAL**: When solving tasks, ALWAYS follow this order:
1. **Search Internet First**: Use web_search for documentation, examples, best practices
2. **Search Open Projects**: Look for similar implementations on GitHub
3. **Analyze Current Codebase**: Use codebase_search to understand existing patterns
4. **Implement**: Only after research, adapt found solutions to project needs

### Anti-Pattern Prevention
- **NO Infinite Loops**: If same fix attempted 2+ times consecutively, STOP and analyze
- **NO Repeated Failures**: If search_replace fails multiple times, read file fully and change approach
- **Document Solutions**: Record successful patterns, avoid repeating failed approaches
- **Ask for Help**: If stuck, communicate problem to user instead of repeating attempts

### Git & Documentation
- Meaningful, structured commit messages
- README files in each directory describing component architecture
- Documentation in Docusaurus format (website/docs/)
- Keep documentation synchronized with code changes

## AI Agent Rules

### Multiple Solution Proposals
- **ALWAYS** propose minimum 2-3 different improvement options
- Each option must be concrete and implementable
- Explain advantages and disadvantages of each option
- Use format: "Вариант 1: ...", "Вариант 2: ...", "Вариант 3: ..."

### Proactive Suggestions
- After task completion, suggest additional improvements
- Use phrases like: "Если хочешь, я могу сразу [конкретное действие]..."
- Ask questions: "Хочешь такой вариант?" or "Какой вариант тебе больше подходит?"
- Show readiness to help with implementation of any option

### Communication Style
- Be friendly and initiative
- Propose options in conversational style
- Use questions to clarify user preferences
- Never propose only one solution option

## Project-Specific Constraints

### Universal File Aggregator Scope
- **Primary Purpose**: Universal file downloader/aggregator from internet
- **Core Features**: Web scraping generator, site collection management, change notifications
- **Mod Management**: Secondary use case (Sims 4, Skyrim, etc.) - must be extensible
- **Not Limited To**: Mods only - system must handle any file type and source

### Visual Parser Builder Requirements
- **Node Editor**: Visual graph-based editor for parser logic modification
- **Browser Integration**: Embedded browser for visual selector picking
- **Page Caching**: Local page storage with versioning for offline development
- **Link Navigation**: Support for following internal links with automatic page caching
- **Version Selection**: UI for selecting page version if multiple available

### Mod Installation System
- **Game Agnostic**: Support for multiple games (Sims 4, Skyrim, Minecraft, RimWorld, Fallout 4, etc.)
- **Extensible Architecture**: Universal way to add installation methods via UI
- **Symlink Collections**: Support for modpack creation and sharing via symlinks
- **Auto-Update**: Automatic mod update checking and installation
- **Filtering**: Advanced filtering system (already implemented, maintain and extend)

### Notification System
- **Change Detection**: Notify users about additions/changes to elements on websites
- **Update Alerts**: Notify about new mod versions
- **User Experience**: Non-intrusive, actionable notifications

## Governance

Constitution supersedes all other practices; Amendments require documentation, approval, and migration plan.

All PRs/reviews must verify compliance with constitution principles; Complexity must be justified; Use `.cursor/rules/` for runtime development guidance; Constitution violations must be addressed before merge.

**Version**: 1.0.0 | **Ratified**: 2025-12-19 | **Last Amended**: 2025-12-19
