# Feature Specification: Universal File Aggregator with Visual Parser

**Feature Branch**: `001-universal-file-aggregator`  
**Created**: 2025-12-19  
**Status**: Draft  
**Input**: User description: "сформируй правила, разбей задачу на этапы сейчас мы занимается созданием визуального парсера страниц + nodeeditor в котором можно менять логику парсера и парсер связан с визуальным браузером который позволяет выбрать селектор и это становится нодой парсера, частично этот функционал сделан в вкладке конструктор, сначала переходишь на сайт скачивается локаьлная версия, на которой можно выделять, можно переходить по внутренним ссылкам, которые тоже формируют локальные страницы, так же можно выбрать версию страницы если доступно несколько. всё это нужно для работы парсера, который спарсит моды с сайта, так же должна быть авто установка модов, для разных игр, это не должно быть константой сейчас проверяетм на симс 4, но могут быть любые скайрим, майнкрафт, римворлд, фолаут 4, и т.д. должен быть уневернальный способ добавления архитектурных блоков желательно с ui , так же эта программа будет иметь спобос автоматического обновления модов, создания сборок, и применение их по ссылке simlink, фильтрации ( уже есть) , уведомление пользователя, дороботать нужно существующий проект. но это могут быть не только моды а просто оснавная суть проекта, это загрузчик файлов, с генератором скраппинга сайтов, созданием коллекций сайтов, оповещением о добавлении изменении элемента на сайте , уневерсальный загрузчик/агрегатор файлов в интернете. пример программ nexus mod manager, mod orgonizer"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Visual Parser Builder with Page Caching (Priority: P1)

**Description**: As a user, I want to create visual parsers for websites by selecting elements in a browser, so that I can extract files and information from any website without writing code.

**Why this priority**: This is the core functionality that differentiates the system. Without visual parser creation, the system cannot aggregate files from websites. Partially implemented in "Constructor" tab - needs completion and enhancement.

**Independent Test**: Can be fully tested by creating a parser for a single website, selecting elements visually, and verifying that the parser configuration is saved and can be executed. Delivers value of enabling non-technical users to create web scrapers.

**Acceptance Scenarios**:

1. **Given** I am on the Constructor tab, **When** I enter a website URL and click "Load", **Then** the page is downloaded locally and displayed in an embedded browser
2. **Given** a page is loaded in the browser, **When** I click on an element, **Then** a selector node is automatically created in the node editor with the CSS selector
3. **Given** I have created multiple nodes in the editor, **When** I connect them to form a parser graph, **Then** the parser logic is validated and can be executed
4. **Given** a parser execution fails due to invalid selector or website structure change, **When** the error occurs, **Then** the system logs the error, displays details to me, and visually marks the problematic node in the editor
5. **Given** I navigate to an internal link on the cached page, **When** I click the link, **Then** the linked page is automatically downloaded and cached locally
6. **Given** multiple versions of a page exist in cache, **When** I select a version from a dropdown, **Then** that version is displayed in the browser for parser development
6. **Given** I have cached page versions, **When** I access cache management UI, **Then** I can view all versions and manually delete selected ones

---

### User Story 2 - Universal File Installation System (Priority: P1)

**Description**: As a user, I want to install files (mods, assets, etc.) for different games and applications through a universal extensible system, so that the system works with any game, not just hardcoded ones.

**Why this priority**: Core value proposition - universal file management. Currently only supports Sims 4, but must support Skyrim, Minecraft, RimWorld, Fallout 4, and be extensible for any future game/application.

**Independent Test**: Can be fully tested by adding a new game type through UI, configuring installation paths and methods, and successfully installing a file for that game. Delivers value of universal file management without code changes.

**Acceptance Scenarios**:

1. **Given** I want to add support for a new game, **When** I use the UI to create a new game profile with installation paths, **Then** the system saves the configuration and can install files for that game
2. **Given** a file is parsed from a website, **When** I click "Install", **Then** the system uses the appropriate installation method for the selected game
3. **Given** I have multiple games configured, **When** I install a file, **Then** I can select which game to install it for
4. **Given** a game requires symlinks for mod management, **When** I install a mod, **Then** the system creates appropriate symlinks automatically

---

### User Story 3 - Automatic File Updates and Notifications (Priority: P2)

**Description**: As a user, I want the system to automatically check for updates to files I've installed and notify me when new versions are available, so that I stay current without manual checking.

**Why this priority**: Enhances user experience significantly by automating maintenance tasks. Users expect modern mod managers to handle updates automatically.

**Independent Test**: Can be fully tested by installing a file, then simulating a new version on the source website, and verifying that the system detects the change and notifies the user. Delivers value of automated maintenance.

**Acceptance Scenarios**:

1. **Given** I have installed files from a website, **When** the system runs its scheduled update check, **Then** it compares current versions with cached versions and detects changes
2. **Given** a new version is detected, **When** the update check completes, **Then** I receive a notification with details about what changed
3. **Given** I receive an update notification, **When** I click "Update", **Then** the new version is downloaded and installed automatically
4. **Given** a website element changes (not just file updates), **When** the change is detected, **Then** I receive a notification about the change

---

### User Story 4 - File Collections (Modpacks) with Symlink Management (Priority: P2)

**Description**: As a user, I want to create collections of files (like modpacks) that can be easily shared and applied, using symlinks to manage file organization without duplicating data.

**Why this priority**: Enables sharing and organization of file sets, similar to Nexus Mod Manager and Mod Organizer. Symlinks provide efficient storage while maintaining flexibility.

**Independent Test**: Can be fully tested by creating a collection, adding files to it, exporting it, and applying it on another system. Delivers value of easy file set management and sharing.

**Acceptance Scenarios**:

1. **Given** I have multiple files installed, **When** I create a new collection and add files to it, **Then** the collection is saved with file references
2. **Given** I have a collection, **When** I export it, **Then** a JSON file is created with metadata and file references
3. **Given** I receive a collection file, **When** I import it, **Then** the system applies the collection using symlinks
4. **Given** I apply a collection, **When** files are installed, **Then** symlinks are created pointing to the actual files

---

### User Story 5 - Site Collection Management and Monitoring (Priority: P3)

**Description**: As a user, I want to organize websites into collections and monitor them for changes, so that I can track multiple sources of files efficiently.

**Why this priority**: Enhances organization for power users who track files from multiple sources. Lower priority than core parsing and installation.

**Independent Test**: Can be fully tested by creating a site collection, adding websites to it, and verifying that change monitoring works for the collection. Delivers value of organized multi-source tracking.

**Acceptance Scenarios**:

1. **Given** I have multiple websites configured, **When** I create a site collection and add websites, **Then** the collection groups them for easier management
2. **Given** I have a site collection, **When** I enable monitoring, **Then** the system checks all sites in the collection for changes
3. **Given** a change is detected on any site in a collection, **When** monitoring runs, **Then** I receive a notification about which site and what changed

---

### Edge Cases

- What happens when a website structure changes and existing selectors no longer work? → **Resolved**: System logs error, shows user details, visually marks problematic node in editor for easy identification and fixing
- How does the system handle websites that require authentication? → **Resolved**: System supports saving encrypted credentials (username/password) for automatic authentication
- What happens when disk space runs out during page caching? → **Resolved**: System stores all versions indefinitely; user must manually manage cache through UI when disk space is needed
- How does the system handle corrupted cached pages?
- What happens when a game installation path is invalid or game is uninstalled?
- How does the system handle symlink creation failures (permissions, unsupported file systems)?
- What happens when multiple users try to install the same file simultaneously?
- How does the system handle websites that block automated access? → **Resolved**: System uses proper User-Agent and delays between requests, shows warning to user when blocking is detected
- What happens when a parser node references a page version that was deleted from cache?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST allow users to load websites in an embedded browser for visual parser development
- **FR-002**: System MUST download and cache web pages locally with versioning support
- **FR-003**: System MUST allow users to select page elements visually and automatically create parser nodes with CSS selectors
- **FR-004**: System MUST provide a visual node editor where users can modify parser logic by connecting nodes
- **FR-005**: System MUST support navigation to internal links within cached pages with automatic caching of linked pages
- **FR-006**: System MUST allow users to select from multiple cached versions of the same page
- **FR-007**: System MUST execute parser graphs to extract files and metadata from websites
- **FR-025**: System MUST log all parser execution errors with detailed information
- **FR-026**: System MUST display parser errors to users with actionable details (which node failed, what selector, error type)
- **FR-027**: System MUST visually mark problematic nodes in the editor when parser execution fails
- **FR-028**: System MUST use proper User-Agent headers and implement delays between requests to respect website rate limits
- **FR-029**: System MUST detect and warn users when websites block automated access (rate limiting, CAPTCHA, etc.)
- **FR-008**: System MUST support universal file installation for multiple games/applications (Sims 4, Skyrim, Minecraft, RimWorld, Fallout 4, and extensible to others)
- **FR-009**: System MUST provide UI for adding new game/application profiles with installation configuration
- **FR-010**: System MUST automatically check for file updates on configured schedules
- **FR-011**: System MUST notify users when file updates or website changes are detected
- **FR-012**: System MUST support automatic installation of file updates
- **FR-013**: System MUST allow users to create file collections (modpacks) that group related files
- **FR-014**: System MUST support exporting and importing file collections for sharing in JSON format with metadata and file references
- **FR-015**: System MUST use symlinks for file collection management to avoid data duplication
- **FR-016**: System MUST support filtering of files (already implemented, maintain and extend)
- **FR-017**: System MUST allow users to organize websites into collections
- **FR-018**: System MUST monitor website collections for changes to elements
- **FR-019**: System MUST work as a universal file aggregator, not limited to game mods
- **FR-020**: System MUST support any file type and source website, not just mod-specific sites
- **FR-021**: System MUST support saving website authentication credentials (username/password) with encryption for automatic authentication during parsing
- **FR-022**: System MUST securely store encrypted credentials and use them automatically when accessing authenticated websites
- **FR-023**: System MUST store all cached page versions indefinitely without automatic deletion
- **FR-024**: System MUST provide UI for users to manually view, select, and delete cached page versions

### Key Entities

- **Parser**: Represents a visual graph of nodes that defines how to extract data from websites. Contains nodes, connections, and execution logic.
- **Parser Node**: Represents a single operation in the parser graph (selector, extract, filter, transform, output). Has type, configuration, and position in graph.
- **Cached Page**: Represents a locally stored version of a web page. Has URL, content, version number, timestamp, and associated site.
- **Site**: Represents a website source for files. Has URL, name, parser configuration, and monitoring settings.
- **File Item**: Represents a file discovered or installed through the system. Has source URL, metadata, version, installation status, and associated game/application.
- **Game Profile**: Represents configuration for a game or application. Has name, installation paths, file organization rules, and installation method.
- **Site Credentials**: Represents authentication information for a website. Has site reference, encrypted username, encrypted password, and last used timestamp.
- **File Collection**: Represents a group of related files (modpack). Has name, file references, symlink configuration, and metadata. Exported as JSON file with metadata and file references (URLs, versions, installation paths).
- **Site Collection**: Represents a group of related websites. Has name, site references, and monitoring settings.
- **Notification**: Represents an alert to the user. Has type (update, change, error), message, timestamp, and read status.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can create a functional parser for a website in under 10 minutes using visual tools
- **SC-002**: System successfully caches and versions at least 1000 pages without performance degradation
- **SC-003**: 95% of parser node connections are valid and executable without errors
- **SC-004**: System supports at least 5 different games/applications out of the box with extensible architecture for unlimited additions
- **SC-005**: Users can add support for a new game through UI in under 5 minutes
- **SC-006**: System detects file updates within 1 hour of publication on source websites (configurable schedule)
- **SC-007**: 90% of file installations complete successfully without manual intervention
- **SC-008**: File collections can be exported and imported with 100% file reference accuracy
- **SC-009**: Symlink-based collections reduce storage usage by at least 50% compared to file duplication
- **SC-010**: System successfully monitors and notifies about changes on at least 100 websites simultaneously
- **SC-011**: Users can filter and find files from collections of 1000+ items in under 2 seconds

## Assumptions

- Users have basic understanding of CSS selectors (or can learn through UI guidance)
- Target file systems support symlinks (Windows, Linux, macOS)
- Users have write permissions to game installation directories
- Websites may change structure, requiring parser updates (expected behavior)
- Network connectivity is available for initial page downloads and update checks
- Cached pages may become stale but system handles this gracefully
- File installation may require game-specific knowledge (handled through game profiles)

## Dependencies

- Existing parser builder component (ParserBuilder.svelte) - needs enhancement
- Existing page viewer component (PageViewer.svelte) - needs enhancement
- Existing filtering system - needs maintenance and extension
- Node editor library (@xyflow/svelte) - already integrated
- Tauri file system plugin - for symlink operations
- Database schema - needs extension for new entities (game profiles, collections, etc.)

## Clarifications

### Session 2025-12-19

- Q: How should the system handle websites that require authentication for accessing files? → A: Support saving credentials (username/password) with encryption for automatic authentication
- Q: How should the system manage cached page versions - when to delete old versions? → A: Store all versions indefinitely, delete only manually by user
- Q: What file format should be used for exporting and importing file collections? → A: JSON file with metadata and file references (human-readable, extensible)
- Q: How should the system handle parser execution errors (invalid selectors, website structure changes, network errors)? → A: Log all errors, show user detailed information, visually mark problematic nodes in editor
- Q: How should the system handle websites that block automated access (rate limiting, CAPTCHA, User-Agent blocking)? → A: Use proper User-Agent and delays between requests, show warning when blocking is detected

## Out of Scope

- Real-time collaborative parser editing
- Cloud synchronization of parsers and collections (local-first architecture)
- Built-in web browser with full JavaScript execution (uses simplified rendering)
- Automatic parser repair when website structure changes
- Payment processing for premium file sources
- Social features (comments, ratings) - focus on file management
