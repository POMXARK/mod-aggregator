<script module>
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import { expect, fn } from '@storybook/test';

  import Sidebar from './Sidebar.svelte';

  const { Story } = defineMeta({
    component: Sidebar,
    args: {
      currentPage: 'mods',
      sites: [
        { id: 1, name: 'Test Site 1' },
        { id: 2, name: 'Test Site 2' },
      ],
      selectedSiteId: null,
      onPageChange: fn(),
      onSiteSelect: fn(),
    },
    argTypes: {
      currentPage: {
        control: { type: 'select' },
        options: ['mods', 'sites', 'parser', 'notifications'],
      },
      selectedSiteId: {
        control: { type: 'select' },
        options: [null, 1, 2],
      },
    },
  });
</script>

<script>
  import { Story } from '@storybook/blocks';
</script>

<!-- Default Sidebar -->
<Story name="Default" />

<!-- Mods Page Active -->
<Story
  name="Mods Page Active"
  args={{
    currentPage: 'mods',
  }}
/>

<!-- Sites Page Active -->
<Story
  name="Sites Page Active"
  args={{
    currentPage: 'sites',
  }}
/>

<!-- Parser Page Active -->
<Story
  name="Parser Page Active"
  args={{
    currentPage: 'parser',
  }}
/>

<!-- Notifications Page Active -->
<Story
  name="Notifications Page Active"
  args={{
    currentPage: 'notifications',
  }}
/>

<!-- With Selected Site -->
<Story
  name="With Selected Site"
  args={{
    selectedSiteId: 1,
  }}
/>

<!-- No Sites Available -->
<Story
  name="No Sites Available"
  args={{
    sites: [],
  }}
/>

<!-- Many Sites -->
<Story
  name="Many Sites"
  args={{
    sites: [
      { id: 1, name: 'Site 1' },
      { id: 2, name: 'Site 2' },
      { id: 3, name: 'Site 3' },
      { id: 4, name: 'Site 4' },
      { id: 5, name: 'Site 5' },
    ],
  }}
/>

<!-- Interaction Test: Page Navigation -->
<Story
  name="Page Navigation"
  play={async ({ args, canvas, userEvent }) => {
    // Test navigation to sites page
    const sitesButton = canvas.getByText('Сайты');
    await userEvent.click(sitesButton);

    // Verify callback was called
    await expect(args.onPageChange).toHaveBeenCalledWith('sites');
  }}
/>

<!-- Interaction Test: Site Selection -->
<Story
  name="Site Selection"
  play={async ({ args, canvas, userEvent }) => {
    // Test selecting a site
    const siteButton = canvas.getByText('Test Site 1');
    await userEvent.click(siteButton);

    // Verify callback was called
    await expect(args.onSiteSelect).toHaveBeenCalledWith(1);
  }}
/>

<!-- Accessibility Test -->
<Story
  name="Accessibility"
  play={async ({ canvas }) => {
    // Check for proper navigation role
    const nav = canvas.getByRole('navigation');
    await expect(nav).toBeInTheDocument();

    // Check for proper button roles
    const buttons = canvas.getAllByRole('button');
    await expect(buttons.length).toBeGreaterThan(0);

    // Check for keyboard navigation support
    const firstButton = buttons[0];
    await expect(firstButton).toHaveAttribute('tabindex');
  }}
/>

