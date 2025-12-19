---
sidebar_position: 2
---

# Руководство по тестированию

Руководство по написанию и запуску тестов в проекте.

## Типы тестов

### Unit тесты

Тестирование отдельных функций и компонентов.

**Frontend (Vitest):**

```typescript
import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import SiteList from './SiteList.svelte';

describe('SiteList', () => {
  it('should render list of sites', () => {
    const sites = [
      { id: 1, name: 'Site 1', url: 'https://site1.com' }
    ];
    render(SiteList, { props: { sites } });
    expect(screen.getByText('Site 1')).toBeInTheDocument();
  });
});
```

**Backend (Rust):**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_url() {
        let url = "https://example.com/page";
        let parsed = parse_url(url).unwrap();
        assert_eq!(parsed.host(), Some("example.com"));
    }
}
```

### E2E тесты

Тестирование полных пользовательских сценариев с Cypress.

```typescript
describe('Sites Management', () => {
  it('should add new site', () => {
    cy.visit('/');
    cy.get('[data-testid="add-site-button"]').click();
    cy.get('[data-testid="site-name-input"]').type('Test Site');
    cy.get('[data-testid="save-site-button"]').click();
    cy.get('[data-testid="sites-list"]').should('contain', 'Test Site');
  });
});
```

## Запуск тестов

```bash
# Все тесты
npm test

# Только unit тесты
npm run test:run

# E2E тесты
npm run cypress:run

# Rust тесты
cd src-tauri && cargo test
```

## Покрытие кода

Стремитесь к минимум 70% покрытия для критичных модулей.

```bash
# Проверка покрытия (TypeScript)
npm run test:coverage

# Проверка покрытия (Rust)
cd src-tauri && cargo tarpaulin
```

