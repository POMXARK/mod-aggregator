describe('Refresh Button Test', () => {
  beforeEach(() => {
    // Переходим на страницу конструктора парсеров
    cy.visit('/')
    
    // Ждем загрузки приложения
    cy.wait(2000)
  })

  it('should display refresh button in recent URLs dropdown', () => {
    // Находим поле ввода URL
    cy.get('.url-input', { timeout: 10000 }).should('be.visible')
    
    // Кликаем на поле ввода, чтобы открыть список последних открытых
    cy.get('.url-input').click()
    
    // Ждем появления dropdown (до 5 секунд)
    cy.get('.recent-urls-dropdown', { timeout: 5000 }).should('be.visible')
    
    // Проверяем наличие заголовка
    cy.get('.recent-urls-header').should('contain', 'Последние открытые')
    
    // Проверяем наличие элементов списка (до 5 секунд)
    cy.get('.recent-url-item', { timeout: 5000 }).should('have.length.at.least', 1)
    
    // Для каждого элемента списка проверяем наличие кнопки обновления
    cy.get('.recent-url-item').each(($item, index) => {
      cy.wrap($item).within(() => {
        // Проверяем наличие контейнера с кнопками (до 5 секунд)
        cy.get('.recent-url-actions', { timeout: 5000 })
          .should('exist')
          .should('be.visible')
        
        // Проверяем наличие кнопки обновления (до 5 секунд)
        cy.get('.recent-url-refresh', { timeout: 5000 })
          .should('exist')
          .should('be.visible')
          .should('have.attr', 'aria-label', 'Обновить')
        
        // Проверяем наличие SVG иконки внутри кнопки (до 5 секунд)
        cy.get('.recent-url-refresh svg', { timeout: 5000 })
          .should('exist')
          .should('be.visible')
        
        // Проверяем наличие кнопки удаления
        cy.get('.recent-url-remove').should('be.visible')
      })
      
      // Делаем скриншот для каждого элемента
      cy.screenshot(`refresh-button-item-${index}`)
    })
    
    // Делаем скриншот для визуальной проверки
    cy.screenshot('refresh-button-visible')
  })
  
  it('should have correct structure for recent URL items', () => {
    cy.get('.url-input').click()
    cy.get('.recent-urls-dropdown', { timeout: 5000 }).should('be.visible')
    
    // Проверяем структуру каждого элемента
    cy.get('.recent-url-item').first().within(() => {
      // Должна быть ссылка
      cy.get('.recent-url-link').should('be.visible')
      
      // Должен быть контейнер с действиями
      cy.get('.recent-url-actions', { timeout: 5000 }).should('be.visible')
      
      // В контейнере должны быть обе кнопки
      cy.get('.recent-url-actions').within(() => {
        cy.get('.recent-url-refresh', { timeout: 5000 }).should('exist').should('be.visible')
        cy.get('.recent-url-remove').should('exist').should('be.visible')
      })
    })
  })
})
