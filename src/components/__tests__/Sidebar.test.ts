import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import Sidebar from '../Sidebar.svelte';

describe('Sidebar', () => {
  const mockSites = [
    { id: 1, name: 'Test Site 1' },
    { id: 2, name: 'Test Site 2' },
  ];

  let mockOnPageChange: ReturnType<typeof vi.fn>;
  let mockOnSiteSelect: ReturnType<typeof vi.fn>;

  const defaultProps = () => ({
    currentPage: 'mods',
    sites: mockSites,
    selectedSiteId: null,
    onPageChange: mockOnPageChange,
    onSiteSelect: mockOnSiteSelect,
  });

  beforeEach(() => {
    mockOnPageChange = vi.fn();
    mockOnSiteSelect = vi.fn();
  });

  it('renders all navigation buttons', () => {
    render(Sidebar, { props: defaultProps() });

    expect(screen.getByText('Моды')).toBeInTheDocument();
    // Use getAllByText for "Сайты" since it appears in both button and h3
    const sitesElements = screen.getAllByText('Сайты');
    expect(sitesElements.length).toBeGreaterThan(0);
    expect(screen.getByText('Конструктор')).toBeInTheDocument();
    expect(screen.getByText('Уведомления')).toBeInTheDocument();
  });

  it('calls onPageChange when "Моды" button is clicked', async () => {
    const user = userEvent.setup();
    render(Sidebar, { props: defaultProps() });

    const modsButton = screen.getByText('Моды').closest('button');
    expect(modsButton).toBeInTheDocument();

    await user.click(modsButton!);
    expect(mockOnPageChange).toHaveBeenCalledWith('mods');
    expect(mockOnPageChange).toHaveBeenCalledTimes(1);
  });

  it('calls onPageChange when "Сайты" button is clicked', async () => {
    const user = userEvent.setup();
    render(Sidebar, { props: defaultProps() });

    // Find the button by role and text within nav
    const nav = screen.getByRole('navigation');
    const sitesButton = Array.from(nav.querySelectorAll('button')).find(
      (btn) => btn.textContent?.includes('Сайты')
    );
    expect(sitesButton).toBeInTheDocument();

    await user.click(sitesButton!);
    expect(mockOnPageChange).toHaveBeenCalledWith('sites');
    expect(mockOnPageChange).toHaveBeenCalledTimes(1);
  });

  it('calls onPageChange when "Конструктор" button is clicked', async () => {
    const user = userEvent.setup();
    render(Sidebar, { props: defaultProps() });

    const parserButton = screen.getByText('Конструктор').closest('button');
    expect(parserButton).toBeInTheDocument();

    await user.click(parserButton!);
    expect(mockOnPageChange).toHaveBeenCalledWith('parser');
    expect(mockOnPageChange).toHaveBeenCalledTimes(1);
  });

  it('calls onPageChange when "Уведомления" button is clicked', async () => {
    const user = userEvent.setup();
    render(Sidebar, { props: defaultProps() });

    const notificationsButton = screen.getByText('Уведомления').closest('button');
    expect(notificationsButton).toBeInTheDocument();

    await user.click(notificationsButton!);
    expect(mockOnPageChange).toHaveBeenCalledWith('notifications');
    expect(mockOnPageChange).toHaveBeenCalledTimes(1);
  });

  it('calls onSiteSelect when "Все сайты" button is clicked', async () => {
    const user = userEvent.setup();
    render(Sidebar, { props: defaultProps() });

    const allSitesButton = screen.getByText('Все сайты');
    expect(allSitesButton).toBeInTheDocument();

    await user.click(allSitesButton);
    expect(mockOnSiteSelect).toHaveBeenCalledWith(null);
    expect(mockOnSiteSelect).toHaveBeenCalledTimes(1);
  });

  it('calls onSiteSelect when a site button is clicked', async () => {
    const user = userEvent.setup();
    render(Sidebar, { props: defaultProps() });

    const siteButton = screen.getByText('Test Site 1');
    expect(siteButton).toBeInTheDocument();

    await user.click(siteButton);
    expect(mockOnSiteSelect).toHaveBeenCalledWith(1);
    expect(mockOnSiteSelect).toHaveBeenCalledTimes(1);
  });

  it('shows active state for current page', () => {
    const { container } = render(Sidebar, { 
      props: { ...defaultProps(), currentPage: 'sites' } 
    });

    // Find the button by role and text within nav
    const nav = screen.getByRole('navigation');
    const sitesButton = Array.from(nav.querySelectorAll('button')).find(
      (btn) => btn.textContent?.includes('Сайты')
    );
    expect(sitesButton).toHaveClass('active');
  });

  it('shows active state for selected site', () => {
    const { container } = render(Sidebar, { 
      props: { ...defaultProps(), selectedSiteId: 1 } 
    });

    const siteButton = screen.getByText('Test Site 1').closest('button');
    expect(siteButton).toHaveClass('active');
  });

  it('updates active state when currentPage prop changes', () => {
    const { component } = render(Sidebar, { 
      props: defaultProps() 
    });

    // Initially mods should be active
    const modsButton = screen.getByText('Моды').closest('button');
    expect(modsButton).toHaveClass('active');

    // Update to sites
    component.$set({ currentPage: 'sites' });

    // Sites should now be active
    const nav = screen.getByRole('navigation');
    const sitesButton = Array.from(nav.querySelectorAll('button')).find(
      (btn) => btn.textContent?.includes('Сайты')
    );
    expect(sitesButton).toHaveClass('active');
    
    // Mods should no longer be active
    const updatedModsButton = screen.getByText('Моды').closest('button');
    expect(updatedModsButton).not.toHaveClass('active');
  });

  it('updates active state when selectedSiteId prop changes', () => {
    const { component } = render(Sidebar, { 
      props: defaultProps() 
    });

    // Initially "Все сайты" should be active
    const allSitesButton = screen.getByText('Все сайты');
    expect(allSitesButton).toHaveClass('active');

    // Update to select site 1
    component.$set({ selectedSiteId: 1 });

    // Site 1 should now be active
    const site1Button = screen.getByText('Test Site 1').closest('button');
    expect(site1Button).toHaveClass('active');
    
    // "Все сайты" should no longer be active
    const updatedAllSitesButton = screen.getByText('Все сайты');
    expect(updatedAllSitesButton).not.toHaveClass('active');
  });
});

