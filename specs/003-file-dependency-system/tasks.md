# Tasks: File Dependency Management and Advanced Collections

**Input**: Design documents from `/specs/003-file-dependency-system/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅

**Tests**: Tests are OPTIONAL per specification for most tasks. However, critical logic (circular dependency detection, validation, collection logic evaluation) MUST have unit tests per Constitution Principle III (TDD). Test tasks T030a, T031a, and T092a are required for these critical components.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Frontend**: `src/components/`, `src/lib/`, `src/types/`
- **Backend**: `src-tauri/src/commands/`, `src-tauri/src/services/`, `src-tauri/src/models/`, `src-tauri/src/database/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [X] T001 Create directory structure for dependencies feature in src/components/dependencies/
- [X] T002 Create directory structure for collections feature in src/components/collections/
- [X] T003 Create directory structure for files feature in src/components/files/
- [X] T004 Create directory structure for import-export feature in src/components/import-export/
- [X] T005 Create directory structure for session feature in src/components/session/
- [X] T006 [P] Create directory structure for lib utilities in src/lib/dependencies/
- [X] T007 [P] Create directory structure for lib utilities in src/lib/collections/
- [X] T008 [P] Create directory structure for lib utilities in src/lib/import-export/
- [X] T009 [P] Create directory structure for lib utilities in src/lib/session/
- [X] T010 [P] Create directory structure for types in src/types/
- [X] T011 [P] Create directory structure for backend commands in src-tauri/src/commands/
- [X] T012 [P] Create directory structure for backend services in src-tauri/src/services/
- [X] T013 [P] Create directory structure for backend models in src-tauri/src/models/
- [X] T014 [P] Create directory structure for database migrations in src-tauri/src/database/migrations/

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T015 Create database migration 003_add_dependencies.sql in src-tauri/src/database/migrations/003_add_dependencies.sql
- [X] T016 Create database migration 004_add_collections.sql in src-tauri/src/database/migrations/004_add_collections.sql
- [X] T017 Create database migration 005_add_session_state.sql in src-tauri/src/database/migrations/005_add_session_state.sql
- [X] T018 [P] Create File model with name@version in src-tauri/src/models/file.rs
- [X] T019 [P] Create FileDependency model in src-tauri/src/models/dependency.rs
- [X] T020 [P] Create Collection model in src-tauri/src/models/collection.rs
- [X] T021 [P] Create CollectionLogicRule model in src-tauri/src/models/collection_logic.rs
- [X] T022 [P] Create SessionState model in src-tauri/src/models/session_state.rs
- [X] T023 [P] Create TypeScript types for File in src/types/file.ts
- [X] T024 [P] Create TypeScript types for FileDependency in src/types/dependency.ts
- [X] T025 [P] Create TypeScript types for Collection in src/types/collection.ts
- [X] T026 [P] Create TypeScript types for CollectionLogicRule in src/types/collection-logic.ts
- [X] T027 [P] Create TypeScript types for SessionState in src/types/session.ts
- [X] T028 Implement database schema extension in src-tauri/src/database/schema.rs
- [X] T029 Update existing database initialization to run new migrations in src-tauri/src/database.rs

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - File Dependency Management (Priority: P1) 🎯 MVP

**Goal**: Enable users to define dependencies between files/mods, track relationships, warn about missing dependencies, and ensure proper installation order.

**Independent Test**: Create a file with dependencies, attempt to use it without dependencies, verify warnings are shown and dependencies can be resolved.

### Implementation for User Story 1

- [X] T030 [P] [US1] Create dependency graph service in src-tauri/src/services/dependency_service/graph.rs
- [X] T031 [P] [US1] Create dependency validator service in src-tauri/src/services/dependency_service/validator.rs
- [X] T032 [P] [US1] Create version resolver service in src-tauri/src/services/dependency_service/resolver.rs
- [X] T030a [P] [US1] Unit test for circular dependency detection in tests/backend/services/dependency_service/test_validator.rs
- [X] T031a [P] [US1] Unit test for dependency graph algorithms in tests/backend/services/dependency_service/test_graph.rs
- [X] T033 [US1] Implement get_file_dependencies command in src-tauri/src/commands/dependencies.rs
- [X] T034 [US1] Implement add_file_dependency command in src-tauri/src/commands/dependencies.rs
- [X] T035 [US1] Implement remove_file_dependency command in src-tauri/src/commands/dependencies.rs
- [X] T036 [US1] Implement check_dependencies command in src-tauri/src/commands/dependencies.rs
- [X] T037 [US1] Implement check_circular_dependencies command in src-tauri/src/commands/dependencies.rs
- [X] T038 [US1] Implement get_dependent_files command in src-tauri/src/commands/dependencies.rs
- [X] T039 [US1] Implement resolve_dependency_version command in src-tauri/src/commands/dependencies.rs
- [X] T040 [US1] Implement get_dependency_graph command in src-tauri/src/commands/dependencies.rs
- [X] T041 [US1] Implement get_installation_order command in src-tauri/src/commands/dependencies.rs
- [X] T042 [P] [US1] Create dependency graph utility in src/lib/dependencies/dependency-graph.ts
- [X] T043 [P] [US1] Create dependency validator utility in src/lib/dependencies/dependency-validator.ts
- [X] T044 [P] [US1] Create version resolver utility in src/lib/dependencies/version-resolver.ts
- [X] T045 [P] [US1] Create DependencyGraph component in src/components/dependencies/DependencyGraph.svelte
- [X] T046 [P] [US1] Create DependencyEditor component in src/components/dependencies/DependencyEditor.svelte
- [X] T047 [P] [US1] Create DependencyWarning component in src/components/dependencies/DependencyWarning.svelte
- [X] T048 [P] [US1] Create VersionSelector component in src/components/dependencies/VersionSelector.svelte
- [X] T049 [US1] Integrate dependency commands with Tauri app state in src-tauri/src/main.rs
- [X] T050 [US1] Add error handling and logging for dependency operations in src-tauri/src/commands/dependencies.rs

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Manual File Addition and Editing (Priority: P1)

**Goal**: Enable users to manually add files/mods through a form, specify all parameters including dependencies, and edit any parameter later.

**Independent Test**: Manually add a file through the form, specify all parameters including dependencies, then edit those parameters.

### Implementation for User Story 2

- [X] T051 [US2] Implement create_file command in src-tauri/src/commands/files.rs
- [X] T052 [US2] Implement update_file command in src-tauri/src/commands/files.rs
- [X] T053 [US2] Implement delete_file command with dependency check in src-tauri/src/commands/files.rs
- [X] T054 [US2] Implement upload_file_version command in src-tauri/src/commands/files.rs
- [X] T055 [US2] Implement get_file_by_name_version command in src-tauri/src/commands/files.rs
- [X] T056 [US2] Implement get_file_versions command in src-tauri/src/commands/files.rs
- [X] T057 [P] [US2] Create FileForm component in src/components/files/FileForm.svelte
- [X] T058 [US2] Add dependency editing to FileForm in src/components/files/FileForm.svelte
- [X] T059 [US2] Add validation for name@version uniqueness in src-tauri/src/commands/files.rs
- [X] T060 [US2] Add validation for file parameters in src/components/files/FileForm.svelte
- [X] T061 [US2] Integrate FileForm with dependency management in src/components/files/FileForm.svelte
- [X] T062 [US2] Add error handling for file operations in src-tauri/src/commands/files.rs

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - File Import and Export (Priority: P1)

**Goal**: Enable users to import and export individual files/mods and collections for sharing, backup, and transfer between systems.

**Independent Test**: Export a file or collection, then import it on another system or after deletion, verify all parameters and dependencies are preserved.

### Implementation for User Story 3

- [X] T063 [US3] Implement export_file command in src-tauri/src/commands/import_export.rs
- [x] T064 [US3] Implement export_collection command in src-tauri/src/commands/import_export.rs
- [x] T065 [US3] Implement export_build command in src-tauri/src/commands/import_export.rs
- [X] T066 [US3] Implement import_file command in src-tauri/src/commands/import_export.rs
- [x] T067 [US3] Implement import_collection command in src-tauri/src/commands/import_export.rs
- [x] T068 [US3] Implement import_build command in src-tauri/src/commands/import_export.rs
- [X] T069 [US3] Implement validate_import_json command in src-tauri/src/commands/import_export.rs
- [X] T070 [P] [US3] Create JSON serializer utility in src/lib/import-export/json-serializer.ts
- [X] T071 [P] [US3] Create JSON serializer service in src-tauri/src/services/import_export_service.rs
- [X] T072 [P] [US3] Create ImportDialog component in src/components/import-export/ImportDialog.svelte
- [X] T073 [P] [US3] Create ExportDialog component in src/components/import-export/ExportDialog.svelte
- [X] T074 [US3] Add JSON schema validation for import in src-tauri/src/services/import_export_service.rs
- [X] T075 [US3] Add dependency resolution for imports in src-tauri/src/commands/import_export.rs
- [X] T076 [US3] Add error handling for import/export operations in src-tauri/src/commands/import_export.rs

**Checkpoint**: At this point, User Stories 1, 2, AND 3 should all work independently

---

## Phase 6: User Story 4 - Advanced Collection Logic and Composition (Priority: P2)

**Goal**: Enable users to create collections with custom enable/disable logic for files, combine multiple collections, and select files from multiple collections.

**Independent Test**: Create a collection with enable/disable logic, combine it with another collection, verify the logic works correctly.

### Implementation for User Story 4

- [x] T077 [US4] Implement get_collections command in src-tauri/src/commands/collections.rs
- [x] T078 [US4] Implement create_collection command in src-tauri/src/commands/collections.rs
- [x] T079 [US4] Implement update_collection command in src-tauri/src/commands/collections.rs
- [x] T080 [US4] Implement delete_collection command in src-tauri/src/commands/collections.rs
- [x] T081 [US4] Implement get_collection_files command in src-tauri/src/commands/collections.rs
- [x] T082 [US4] Implement add_file_to_collection command in src-tauri/src/commands/collections.rs
- [x] T083 [US4] Implement remove_file_from_collection command in src-tauri/src/commands/collections.rs
- [x] T084 [US4] Implement reorder_collection_files command in src-tauri/src/commands/collections.rs
- [x] T085 [US4] Implement get_collection_logic_rules command in src-tauri/src/commands/collections.rs
- [x] T086 [US4] Implement create_collection_logic_rule command in src-tauri/src/commands/collections.rs
- [x] T087 [US4] Implement update_collection_logic_rule command in src-tauri/src/commands/collections.rs
- [x] T088 [US4] Implement delete_collection_logic_rule command in src-tauri/src/commands/collections.rs
- [x] T089 [US4] Implement evaluate_collection_logic command in src-tauri/src/commands/collections.rs
- [x] T090 [US4] Implement combine_collections command in src-tauri/src/commands/collections.rs
- [x] T091 [US4] Implement get_files_from_multiple_collections command in src-tauri/src/commands/collections.rs
- [x] T092 [P] [US4] Create collection logic evaluator service in src-tauri/src/services/collection_service/logic_evaluator.rs
- [x] T092a [P] [US4] Unit test for collection logic evaluation in tests/backend/services/collection_service/test_logic_evaluator.rs
- [x] T093 [P] [US4] Create collection logic utility in src/lib/collections/collection-logic.ts
- [x] T094 [P] [US4] Create collection composer utility in src/lib/collections/collection-composer.ts
- [x] T095 [P] [US4] Create CollectionLogicBuilder component in src/components/collections/CollectionLogicBuilder.svelte
- [x] T096 [P] [US4] Create CollectionComposer component in src/components/collections/CollectionComposer.svelte
- [x] T097 [P] [US4] Create CollectionViewer component in src/components/collections/CollectionViewer.svelte
- [x] T098 [US4] Add UI constructor for boolean conditions in src/components/collections/CollectionLogicBuilder.svelte
- [x] T099 [US4] Add UI constructor for collection_check conditions in src/components/collections/CollectionLogicBuilder.svelte
- [x] T100 [US4] Add UI constructor for file_check conditions in src/components/collections/CollectionLogicBuilder.svelte
- [x] T101 [US4] Add UI constructor for and/or conditions in src/components/collections/CollectionLogicBuilder.svelte
- [x] T102 [US4] Add validation for collection logic rules in src-tauri/src/commands/collections.rs
- [x] T103 [US4] Integrate collection commands with Tauri app state in src-tauri/src/main.rs

**Checkpoint**: At this point, User Stories 1-4 should all work independently

---

## Phase 7: User Story 5 - Multi-Selection and Batch Operations (Priority: P2)

**Goal**: Enable users to select multiple files with mouse and perform batch operations for efficient management of large numbers of files.

**Independent Test**: Select multiple files, perform a batch operation (delete, move, change properties), verify the operation applies to all selected files.

### Implementation for User Story 5

- [x] T103a [US5] Create batch_operations.rs command file structure in src-tauri/src/commands/batch_operations.rs
- [x] T104 [US5] Implement batch_delete_files command in src-tauri/src/commands/batch_operations.rs
- [x] T105 [US5] Implement batch_move_files_to_collection command in src-tauri/src/commands/batch_operations.rs
- [x] T106 [US5] Implement batch_update_file_properties command in src-tauri/src/commands/batch_operations.rs
- [x] T107 [US5] Implement batch_add_dependencies command in src-tauri/src/commands/batch_operations.rs
- [x] T108 [US5] Implement batch_check_dependencies command in src-tauri/src/commands/batch_operations.rs
- [x] T109 [US5] Implement batch_export_files command in src-tauri/src/commands/batch_operations.rs
- [x] T110 [US5] Implement validate_batch_operation command in src-tauri/src/commands/batch_operations.rs
- [x] T111 [P] [US5] Create FileList component with multi-selection in src/components/files/FileList.svelte
- [x] T112 [P] [US5] Create BatchOperations component in src/components/files/BatchOperations.svelte
- [x] T113 [US5] Add click selection (single, Ctrl+Click, Shift+Click) to FileList in src/components/files/FileList.svelte
- [x] T114 [US5] Add drag selection to FileList in src/components/files/FileList.svelte
- [x] T115 [US5] Add batch operation menu to BatchOperations in src/components/files/BatchOperations.svelte
- [x] T116 [US5] Add dependency warnings for batch operations in src/components/files/BatchOperations.svelte
- [x] T117 [US5] Add error handling for batch operations in src-tauri/src/commands/batch_operations.rs
- [x] T118 [US5] Integrate batch operations with dependency checking in src-tauri/src/commands/batch_operations.rs

**Checkpoint**: At this point, User Stories 1-5 should all work independently

---

## Phase 8: User Story 6 - State Persistence and Session Recovery (Priority: P2)

**Goal**: Enable the system to remember file order, view preferences, and recent actions, restoring workspace when reopening the application.

**Independent Test**: Customize file order and view, close and reopen the application, verify all preferences and order are restored.

### Implementation for User Story 6

- [x] T119 [US6] Implement get_session_state command in src-tauri/src/commands/session_state.rs
- [x] T120 [US6] Implement update_file_order command in src-tauri/src/commands/session_state.rs
- [x] T121 [US6] Implement update_ui_preferences command in src-tauri/src/commands/session_state.rs
- [x] T122 [US6] Implement update_open_collections command in src-tauri/src/commands/session_state.rs
- [x] T123 [US6] Implement update_selected_files command in src-tauri/src/commands/session_state.rs
- [x] T124 [US6] Implement add_recent_action command in src-tauri/src/commands/session_state.rs
- [x] T125 [US6] Implement get_recent_actions command in src-tauri/src/commands/session_state.rs
- [x] T126 [US6] Implement clear_recent_actions command in src-tauri/src/commands/session_state.rs
- [x] T127 [US6] Implement restore_session command in src-tauri/src/commands/session_state.rs
- [x] T128 [US6] Implement reset_session_state command in src-tauri/src/commands/session_state.rs
- [x] T129 [P] [US6] Create session state utility in src/lib/session/session-state.ts
- [x] T130 [P] [US6] Create SessionRestore component in src/components/session/SessionRestore.svelte
- [x] T131 [US6] Create FileDragDrop component for reordering in src/components/files/FileDragDrop.svelte
- [x] T132 [US6] Add auto-save for file order changes in src/components/files/FileDragDrop.svelte
- [x] T133 [US6] Add auto-save for UI preferences changes in src/components/session/SessionRestore.svelte
- [x] T134 [US6] Add session restoration on app startup in src/App.svelte
- [x] T135 [US6] Add validation for restored session state in src-tauri/src/commands/session_state.rs
- [x] T136 [US6] Add limit enforcement for recent_actions (max 50) in src-tauri/src/commands/session_state.rs

**Checkpoint**: At this point, all User Stories 1-6 should be fully functional

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T137 [P] Add JSDoc documentation to all TypeScript utilities in src/lib/
- [x] T138 [P] Add Rust documentation to all public functions in src-tauri/src/
- [x] T139 [P] Create README for dependencies feature in src/components/dependencies/README.md
- [x] T140 [P] Create README for collections feature in src/components/collections/README.md
- [x] T141 [P] Create README for files feature in src/components/files/README.md
- [x] T142 [P] Create README for import-export feature in src/components/import-export/README.md
- [x] T143 [P] Create README for session feature in src/components/session/README.md
- [x] T144 Code cleanup and refactoring across all new modules
- [x] T145 Performance optimization for dependency graph operations
- [x] T146 Performance optimization for collection logic evaluation
- [x] T146a Performance optimization for multi-collection file viewing (SC-009: <500ms response time, maintain UI responsiveness) in src/components/collections/CollectionViewer.svelte and src-tauri/src/commands/collections.rs
- [x] T147 Add comprehensive error messages for all user-facing operations
- [ ] T148 Run quickstart.md validation scenarios (Manual testing required)
- [x] T149 Add logging for all critical operations
- [ ] T150 Security review for file operations and dependency validation (Manual review required)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-8)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2)
- **Polish (Phase 9)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Can start after Foundational (Phase 2) - Uses US1 dependency system
- **User Story 3 (P1)**: Can start after Foundational (Phase 2) - Uses US1 and US2
- **User Story 4 (P2)**: Can start after Foundational (Phase 2) - Uses US2 files
- **User Story 5 (P2)**: Can start after Foundational (Phase 2) - Uses US1, US2
- **User Story 6 (P2)**: Can start after Foundational (Phase 2) - Uses US2 files

### Within Each User Story

- Models before services
- Services before commands
- Commands before UI components
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- Models within a story marked [P] can run in parallel
- UI components within a story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch all models for User Story 1 together:
Task: "Create dependency graph service in src-tauri/src/services/dependency_service/graph.rs"
Task: "Create dependency validator service in src-tauri/src/services/dependency_service/validator.rs"
Task: "Create version resolver service in src-tauri/src/services/dependency_service/resolver.rs"

# Launch all UI components for User Story 1 together:
Task: "Create DependencyGraph component in src/components/dependencies/DependencyGraph.svelte"
Task: "Create DependencyEditor component in src/components/dependencies/DependencyEditor.svelte"
Task: "Create DependencyWarning component in src/components/dependencies/DependencyWarning.svelte"
Task: "Create VersionSelector component in src/components/dependencies/VersionSelector.svelte"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (File Dependency Management)
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Add User Stories 4-6 (P2) → Test independently → Deploy/Demo
6. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (P1)
   - Developer B: User Story 2 (P1)
   - Developer C: User Story 3 (P1)
3. After P1 stories complete:
   - Developer A: User Story 4 (P2)
   - Developer B: User Story 5 (P2)
   - Developer C: User Story 6 (P2)
4. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
- All file paths are relative to repository root
- Backend commands must be registered in src-tauri/src/main.rs
- Frontend components should use Svelte 5 runes ($state, $derived, $effect)
- All Tauri commands must return Result<T, String> for error handling

