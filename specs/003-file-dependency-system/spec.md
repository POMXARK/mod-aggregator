# Feature Specification: File Dependency Management and Advanced Collections

**Feature Branch**: `003-file-dependency-system`  
**Created**: 2025-12-19  
**Status**: Draft  
**Input**: User description: "добавь что моды/файлы могут зависить от других продумай логику зависимостей, тоесть как в менеджерах зависимостей таких как npm и прочих, когда один пакет использует другой, и все CRUD операции с возможностью выбора действия, и предупреждения что не хватает зависимостей или выбором определенной версии. добавь возможность  форму для самостоятельно добавления файлов/модов указания связей/зависимостей, любой параметр должен быть редактируемым, так же можно загрузить вручную вругую версию мода/файла, импротировать/экспортировать файл/мод/сборку, из сборок сборка состоит из нескольких файлов, у которых есть уникальная логика по которой сборка включается или выключается. сборки можно делать обьединяя несколько сборок, файлы из сборок можно просматировать и из нескольких сборок выбирать для создания другой сборки. сборка это колллекция из нескольких файлов. отмечать файлы можно мышкой и выполнять с ними различные действия. например перетаскиваниепм менять порядок сортировки, который сохранится при следующем открытии, сохранять последние действия при повторноми открытии программы"

## Clarifications

### Session 2025-12-19

- Q: How should the system uniquely identify files for dependency tracking and conflict resolution? → A: Combination of name and version (name@version, similar to npm)
- Q: When File A requires Dependency v1.0 and File B requires Dependency v2.0, what is the expected behavior? → A: System allows installation of both versions simultaneously (similar to npm peer dependencies)
- Q: How should users specify enable/disable logic for files in collections? → A: UI constructor with predefined conditions (checkboxes, dropdowns, toggles) - no text expression syntax required
- Q: What format should be used for importing/exporting files and collections? → A: Single JSON format for all types (files, collections, builds)
- Q: What should happen when a user tries to delete a file that other files depend on? → A: Block deletion, show list of dependent files, and offer actions (remove dependencies, replace with another version)

## User Scenarios & Testing *(mandatory)*

### User Story 1 - File Dependency Management (Priority: P1)

**Description**: As a user, I want to define dependencies between files/mods (similar to npm package dependencies), so that the system can track relationships, warn about missing dependencies, and ensure proper installation order.

**Why this priority**: This is core functionality for managing complex mod setups. Without dependency tracking, users cannot understand file relationships or resolve conflicts. Essential for reliability and user experience.

**Independent Test**: Can be fully tested by creating a file with dependencies, attempting to use it without dependencies, and verifying that warnings are shown and dependencies can be resolved. Delivers value of dependency-aware file management.

**Acceptance Scenarios**:

1. **Given** I want to add a file that depends on another file, **When** I specify dependencies in the file form, **Then** the system saves the dependency relationships
2. **Given** I have a file with dependencies, **When** I try to use it without installing dependencies, **Then** the system warns me about missing dependencies
3. **Given** I have a file with dependencies, **When** multiple versions of a dependency are available, **Then** I can select which version to use
4. **Given** I install a file with dependencies, **When** dependencies are missing, **Then** the system offers to install them automatically or shows a list to choose from
5. **Given** I have files with circular dependencies, **When** I attempt to install them, **Then** the system detects and warns about the circular dependency

---

### User Story 2 - Manual File Addition and Editing (Priority: P1)

**Description**: As a user, I want to manually add files/mods through a form, specify all parameters including dependencies, and edit any parameter later, so that I have full control over file management even for files not discovered through parsing.

**Why this priority**: Enables users to manage files from any source, not just parsed websites. Essential for flexibility and completeness of the file management system.

**Independent Test**: Can be fully tested by manually adding a file through the form, specifying all parameters including dependencies, and then editing those parameters. Delivers value of complete file management control.

**Acceptance Scenarios**:

1. **Given** I want to add a file manually, **When** I open the file addition form, **Then** I can specify name, version, path, dependencies, and all other file parameters
2. **Given** I have added a file manually, **When** I want to edit it, **Then** I can modify any parameter including dependencies, version, and metadata
3. **Given** I am editing file parameters, **When** I change dependencies, **Then** the system validates the dependency relationships and warns about issues
4. **Given** I want to manually load a different version of a file, **When** I select the file, **Then** I can upload a new version and specify its parameters

---

### User Story 3 - File Import and Export (Priority: P1)

**Description**: As a user, I want to import and export individual files/mods and collections, so that I can share configurations, backup my setup, and transfer files between systems.

**Why this priority**: Enables sharing and portability of file configurations. Essential for collaboration and backup scenarios.

**Independent Test**: Can be fully tested by exporting a file or collection, then importing it on another system or after deletion, and verifying that all parameters and dependencies are preserved. Delivers value of file portability and sharing.

**Acceptance Scenarios**:

1. **Given** I have a file or collection configured, **When** I export it, **Then** a file is created with all parameters, dependencies, and metadata
2. **Given** I have an exported file, **When** I import it, **Then** the system recreates the file/collection with all parameters and dependencies
3. **Given** I import a file with dependencies, **When** dependencies are not available, **Then** the system warns me and offers to resolve them
4. **Given** I export a collection, **When** I import it, **Then** all files in the collection are imported with their relationships preserved

---

### User Story 4 - Advanced Collection Logic and Composition (Priority: P2)

**Description**: As a user, I want to create collections with custom enable/disable logic for files, combine multiple collections into new collections, and select files from multiple collections, so that I can create complex, conditional file setups.

**Why this priority**: Enables advanced users to create sophisticated mod setups with conditional logic. Enhances flexibility beyond simple file grouping.

**Independent Test**: Can be fully tested by creating a collection with enable/disable logic, combining it with another collection, and verifying that the logic works correctly. Delivers value of advanced collection management.

**Acceptance Scenarios**:

1. **Given** I am creating a collection, **When** I add files to it, **Then** I can specify unique enable/disable logic for each file using UI constructor with predefined conditions (checkboxes, dropdowns, toggles)
2. **Given** I have multiple collections, **When** I want to create a new collection, **Then** I can combine existing collections and select specific files from them
3. **Given** I am viewing files from multiple collections, **When** I browse them, **Then** I can see which collection each file belongs to and select files for a new collection
4. **Given** I have a collection with enable/disable logic, **When** I apply the collection, **Then** files are enabled or disabled according to their logic rules

---

### User Story 5 - Multi-Selection and Batch Operations (Priority: P2)

**Description**: As a user, I want to select multiple files with mouse (click, drag selection) and perform batch operations on them, so that I can efficiently manage large numbers of files.

**Why this priority**: Essential for productivity when managing many files. Standard interaction pattern that users expect in modern file managers.

**Independent Test**: Can be fully tested by selecting multiple files, performing a batch operation (delete, move, change properties), and verifying that the operation applies to all selected files. Delivers value of efficient bulk file management.

**Acceptance Scenarios**:

1. **Given** I have multiple files displayed, **When** I click and drag to select multiple files, **Then** the selected files are highlighted
2. **Given** I have selected multiple files, **When** I right-click or use a menu, **Then** I can choose from available batch operations (delete, move to collection, change properties, etc.)
3. **Given** I perform a batch operation, **When** the operation affects dependencies, **Then** the system warns me about potential dependency issues
4. **Given** I select files with Ctrl+Click or Shift+Click, **When** I make the selection, **Then** multiple files are selected for batch operations

---

### User Story 6 - State Persistence and Session Recovery (Priority: P2)

**Description**: As a user, I want the system to remember my file order, view preferences, and recent actions, so that when I reopen the application, my workspace is restored as I left it.

**Why this priority**: Improves user experience significantly by maintaining continuity between sessions. Users expect modern applications to remember their state.

**Independent Test**: Can be fully tested by customizing file order and view, closing the application, reopening it, and verifying that all preferences and order are restored. Delivers value of persistent workspace.

**Acceptance Scenarios**:

1. **Given** I have reordered files via drag and drop, **When** I close and reopen the application, **Then** my custom file order is preserved
2. **Given** I have changed view mode and other UI preferences, **When** I reopen the application, **Then** all preferences are restored
3. **Given** I have performed recent actions (installed files, created collections), **When** I reopen the application, **Then** I can see a history or list of recent actions
4. **Given** I have open collections or selected files, **When** I reopen the application, **Then** the system attempts to restore my previous session state

---

### Edge Cases

- What happens when a dependency is deleted but other files depend on it? **RESOLVED**: System blocks deletion, displays list of all dependent files, and offers user actions to resolve (remove dependencies from dependent files, replace dependency with another version, or cancel deletion).
- How does the system handle dependency version conflicts (File A needs Dependency v1.0, File B needs Dependency v2.0)? **RESOLVED**: System allows multiple versions of the same dependency to coexist simultaneously, similar to npm peer dependencies. Each file can use its required version independently.
- What happens when importing a file/collection with dependencies that don't exist in the system? **RESOLVED**: System validates imported dependencies during import. If dependencies are missing, import result includes list of missing dependencies with available versions (if any). User can choose to: (1) import without dependencies (file marked as having missing dependencies), (2) cancel import, or (3) install available versions of dependencies before completing import.
- How does the system handle circular dependencies in collections? **RESOLVED**: Collections use the same circular dependency detection as files. When adding a file to a collection, if that file's dependencies would create a cycle through collection relationships, the system warns and blocks the operation. Collection logic rules are validated to prevent circular references between collections.
- What happens when enable/disable logic in a collection conflicts or creates impossible states? **RESOLVED**: System validates collection logic rules during creation/update. If logic creates impossible states (e.g., file must be both enabled and disabled), validation fails with error message explaining the conflict. Logic evaluator uses deterministic evaluation order (boolean → file_check → collection_check → and/or) to resolve conflicts consistently.
- How does batch selection work when files are in different view modes (tiles vs list)? **RESOLVED**: Selection state is independent of view mode. Files maintain selection state across view mode changes. When switching between tiles and list view, selected files remain selected. Batch operations work identically in both modes, operating on the current selection set regardless of view.
- What happens when combining collections with conflicting file versions? **RESOLVED**: When combining collections, if multiple collections contain different versions of the same file (same name, different version), the system shows a conflict resolution dialog. User can choose: (1) keep all versions (all versions included in new collection), (2) keep latest version, (3) keep specific version, or (4) exclude conflicting files. Default behavior: keep all versions to preserve data integrity.
- How does the system handle state persistence if the database is corrupted or missing? **RESOLVED**: On application startup, system validates session_state table integrity. If corruption detected or table missing: (1) attempt to restore from backup (if available), (2) if restoration fails, initialize with default state (empty file order, default UI preferences), (3) log error for user notification, (4) continue application startup with default state. User can manually reset session state if needed.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST support defining dependencies between files/mods (one file depends on another)
- **FR-002**: System MUST track dependency relationships similar to package managers (npm-style dependency graph)
- **FR-003**: System MUST warn users when required dependencies are missing
- **FR-004**: System MUST allow users to select specific versions of dependencies when multiple versions are available
- **FR-004a**: System MUST support installation of multiple versions of the same dependency simultaneously when different files require different versions (version coexistence)
- **FR-005**: System MUST detect and warn about circular dependencies
- **FR-006**: System MUST provide CRUD operations (Create, Read, Update, Delete) for files with choice of actions
- **FR-006a**: System MUST block deletion of files that have dependent files, display list of dependent files, and offer resolution actions (remove dependencies, replace with another version, cancel deletion)
- **FR-007**: System MUST provide a form for manually adding files/mods with all parameters editable
- **FR-008**: System MUST allow editing of any file parameter including dependencies, version, metadata
- **FR-009**: System MUST support manual upload of different file versions
- **FR-010**: System MUST support importing files/mods/collections from external files
- **FR-011**: System MUST support exporting files/mods/collections to external files
- **FR-012**: System MUST preserve all parameters, dependencies, and relationships during import/export
- **FR-012a**: System MUST use single JSON format for all import/export operations (files, collections, builds) for consistency and compatibility
- **FR-013**: System MUST allow collections to contain files with unique enable/disable logic (conditional rules)
- **FR-013a**: System MUST provide UI constructor interface for defining enable/disable logic using predefined conditions (checkboxes, dropdowns, toggles) without requiring text expression syntax
- **FR-014**: System MUST support combining multiple collections into new collections
- **FR-015**: System MUST allow viewing files from multiple collections simultaneously
- **FR-016**: System MUST allow selecting files from multiple collections to create a new collection
- **FR-017**: System MUST support mouse-based file selection (single click, drag selection, Ctrl+Click, Shift+Click)
- **FR-018**: System MUST support batch operations on selected files (delete, move, change properties, etc.)
- **FR-019**: System MUST persist custom file order set via drag and drop across application restarts
- **FR-020**: System MUST save and restore UI preferences (view mode, window state) across sessions
- **FR-021**: System MUST maintain history of recent actions (installed files, created collections, etc.)
- **FR-022**: System MUST attempt to restore previous session state (open collections, selected files) on application restart

### Key Entities

- **File**: Represents a file/mod in the system. Uniquely identified by combination of name and version (name@version format, similar to npm packages). Has name, version, path, dependencies, metadata, and other parameters. Multiple versions of the same file name can coexist.
- **File Dependency**: Represents a dependency relationship between files. Has source file (identified by name@version), target file (dependency, identified by name@version), version requirement (optional), and dependency type (required, optional, peer).
- **Dependency Graph**: Represents the complete network of file dependencies. Used for validation, conflict detection, and installation ordering. Nodes are files identified by name@version.
- **Collection Logic Rule**: Represents conditional logic for enabling/disabling files in a collection. Configured via UI constructor with predefined conditions (checkboxes, dropdowns, toggles) rather than text expressions. Has condition type, parameters, and target files. Determines when files in collection are active.
- **File Selection**: Represents user's selection of files for batch operations. Has selected file IDs (name@version identifiers), selection method (click, drag, keyboard), and timestamp.
- **Session State**: Represents saved application state. Has file order, view preferences, open collections, selected files, and recent actions history.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can define dependencies for files with 100% accuracy in dependency relationship tracking
- **SC-002**: System detects missing dependencies and shows warnings within 1 second of file operation attempt
- **SC-003**: Users can manually add a file with all parameters in under 2 minutes
- **SC-004**: File import/export preserves 100% of parameters, dependencies, and relationships
- **SC-005**: Users can combine 5+ collections into a new collection without data loss
- **SC-006**: Users can select and perform batch operations on 50+ files simultaneously
- **SC-007**: Custom file order and UI preferences persist correctly across 100% of application restarts
- **SC-008**: System detects circular dependencies and version conflicts within 2 seconds
- **SC-009**: Users can view files from 10+ collections simultaneously without performance degradation (response time for loading files <500ms, UI remains responsive during scrolling/filtering)
- **SC-010**: Recent actions history displays last 50 actions with 100% accuracy

## Assumptions

- Users understand dependency concepts from experience with package managers (npm, pip, etc.)
- Dependency version requirements follow semantic versioning or similar patterns
- File enable/disable logic is configured via UI constructor with predefined conditions (no text expression syntax required)
- Import/export uses single JSON format for all types (files, collections, builds) for compatibility, debugging, and consistency
- Users expect standard file selection patterns (click, drag, Ctrl+Click, Shift+Click)
- Session state can be stored persistently (database or file system)

## Dependencies

- Existing file management system - needs extension for dependencies
- Existing collection system - needs enhancement for advanced logic and composition
- Database schema - needs extension for dependency relationships, collection logic, session state
- Import/export functionality - needs implementation for file/collection serialization
- State persistence mechanism - needs implementation for session recovery

## Terminology

- **File**: A file/mod in the system, uniquely identified by `name@version` (e.g., `my-mod@1.0.0`)
- **Collection**: A group of files with optional enable/disable logic. Collections can be combined to create new collections.
- **Build**: A combined collection created by merging multiple collections and/or selecting specific files. Used primarily in import/export context. A build can be exported and imported as a single unit. When imported, a build may be converted to a collection for easier management.
- **Dependency**: A relationship where one file requires another file to function properly
- **Session State**: Saved application state including file order, UI preferences, open collections, and recent actions

## Out of Scope

- Automatic dependency resolution algorithms (user manually selects versions)
- Dependency conflict auto-resolution (system warns, user decides)
- Complex scripting language for collection logic (simple conditional rules only)
- Real-time collaboration on collections
- Cloud-based dependency repository (local dependencies only)
- Dependency version range parsing (exact version matching, user specifies ranges manually if needed)
