import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import App from '../App.svelte';
import * as tauriApi from '@tauri-apps/api/core';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('App Integration Tests', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // Mock invoke to return empty array for get_sites
    vi.mocked(tauriApi.invoke).mockResolvedValue([]);
  });

  it('renders ModsList by default', async () => {
    render(App);
    
    // Wait for component to mount and load
    await waitFor(() => {
      // Check that ModsList content is visible (heading in main content)
      const main = screen.getByRole('main');
      const modsHeading = main.querySelector('h2');
      expect(modsHeading).toHaveTextContent('Моды');
    });
  });

  it('switches to Sites page when "Сайты" button is clicked', async () => {
    const user = userEvent.setup();
    render(App);
    
    // Wait for initial render
    await waitFor(() => {
      const main = screen.getByRole('main');
      expect(main.querySelector('h2')).toHaveTextContent('Моды');
    });

    // Find and click "Сайты" button in navigation
    const nav = screen.getByRole('navigation');
    const sitesButton = Array.from(nav.querySelectorAll('button')).find(
      (btn) => btn.textContent?.includes('Сайты')
    );
    expect(sitesButton).toBeInTheDocument();

    await user.click(sitesButton!);

    // Wait for content to change
    await waitFor(() => {
      // Check that SitesManager content is visible
      expect(screen.getByText('Управление сайтами')).toBeInTheDocument();
      
      // Verify ModsList is no longer visible
      const main = screen.getByRole('main');
      const modsHeading = main.querySelector('h2');
      expect(modsHeading).not.toHaveTextContent('Моды');
    }, { timeout: 2000 });
  });

  it('switches to Parser page when "Конструктор" button is clicked', async () => {
    const user = userEvent.setup();
    render(App);
    
    await waitFor(() => {
      const main = screen.getByRole('main');
      expect(main.querySelector('h2')).toHaveTextContent('Моды');
    });

    const parserButton = screen.getByText('Конструктор').closest('button');
    expect(parserButton).toBeInTheDocument();

    await user.click(parserButton!);

    // Wait for ParserBuilder to appear (check for select element)
    await waitFor(() => {
      // ParserBuilder has a select for choosing sites
      const siteSelect = screen.queryByText('Выберите сайт для редактирования');
      expect(siteSelect).toBeInTheDocument();
      
      // ModsList should be gone
      const main = screen.getByRole('main');
      const modsHeading = main.querySelector('h2');
      expect(modsHeading).not.toHaveTextContent('Моды');
    }, { timeout: 2000 });
  });

  it('switches to Notifications page when "Уведомления" button is clicked', async () => {
    const user = userEvent.setup();
    render(App);
    
    await waitFor(() => {
      const main = screen.getByRole('main');
      expect(main.querySelector('h2')).toHaveTextContent('Моды');
    });

    const notificationsButton = screen.getByText('Уведомления').closest('button');
    expect(notificationsButton).toBeInTheDocument();

    await user.click(notificationsButton!);

    // Wait for NotificationsPanel to appear
    await waitFor(() => {
      const notificationsHeading = screen.getByRole('heading', { name: /уведомления/i });
      expect(notificationsHeading).toBeInTheDocument();
      
      // ModsList should be gone
      const main = screen.getByRole('main');
      const modsHeading = main.querySelector('h2');
      expect(modsHeading).not.toHaveTextContent('Моды');
    }, { timeout: 2000 });
  });

  it('switches back to Mods page when "Моды" button is clicked', async () => {
    const user = userEvent.setup();
    render(App);
    
    await waitFor(() => {
      const main = screen.getByRole('main');
      expect(main.querySelector('h2')).toHaveTextContent('Моды');
    });

    // First switch to Sites
    const nav = screen.getByRole('navigation');
    const sitesButton = Array.from(nav.querySelectorAll('button')).find(
      (btn) => btn.textContent?.includes('Сайты')
    );
    await user.click(sitesButton!);

    await waitFor(() => {
      expect(screen.getByText('Управление сайтами')).toBeInTheDocument();
    });

    // Then switch back to Mods - find button in navigation
    const modsButton = Array.from(nav.querySelectorAll('button')).find(
      (btn) => btn.textContent?.includes('Моды')
    );
    expect(modsButton).toBeInTheDocument();
    await user.click(modsButton!);

    // Wait for ModsList to appear again
    await waitFor(() => {
      const main = screen.getByRole('main');
      const modsHeading = main.querySelector('h2');
      expect(modsHeading).toHaveTextContent('Моды');
    }, { timeout: 2000 });

    // Verify SitesManager is no longer visible
    expect(screen.queryByText('Управление сайтами')).not.toBeInTheDocument();
  });

  it('updates active state of navigation buttons when page changes', async () => {
    const user = userEvent.setup();
    render(App);
    
    await waitFor(() => {
      const main = screen.getByRole('main');
      expect(main.querySelector('h2')).toHaveTextContent('Моды');
    });

    const nav = screen.getByRole('navigation');
    
    // Initially, Mods button should be active
    const modsButton = Array.from(nav.querySelectorAll('button')).find(
      (btn) => btn.textContent?.includes('Моды')
    );
    expect(modsButton).toHaveClass('active');

    // Click Sites button
    const sitesButton = Array.from(nav.querySelectorAll('button')).find(
      (btn) => btn.textContent?.includes('Сайты')
    );
    await user.click(sitesButton!);

    // Wait for state to update
    await waitFor(() => {
      // Sites button should now be active
      const updatedSitesButton = Array.from(nav.querySelectorAll('button')).find(
        (btn) => btn.textContent?.includes('Сайты')
      );
      expect(updatedSitesButton).toHaveClass('active');
      
      // Mods button should no longer be active
      const updatedModsButton = Array.from(nav.querySelectorAll('button')).find(
        (btn) => btn.textContent?.includes('Моды')
      );
      expect(updatedModsButton).not.toHaveClass('active');
    }, { timeout: 2000 });
  });
});

