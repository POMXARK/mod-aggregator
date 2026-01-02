<script module>
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import { expect, fn } from '@storybook/test';

  import CollectionViewer from './CollectionViewer.svelte';

  const { Story } = defineMeta({
    component: CollectionViewer,
    args: {
      collectionIds: [1, 2],
      showEvaluation: true,
    },
    argTypes: {
      collectionIds: {
        control: { type: 'object' },
      },
      showEvaluation: {
        control: { type: 'boolean' },
      },
    },
    parameters: {
      // Mock Tauri invoke for stories
      mockData: {
        files: [
          {
            fileId: 1,
            file: {
              id: 1,
              name: 'mod1.dll',
              version: '1.0.0',
              size: 1024000,
              hash: 'abc123',
              createdAt: '2024-01-01T00:00:00Z',
            },
            collections: [
              { collectionId: 1, collectionName: 'Essential Mods', orderIndex: 1 },
            ],
          },
          {
            fileId: 2,
            file: {
              id: 2,
              name: 'texture_pack.zip',
              version: '2.0.0',
              size: 2048000,
              hash: 'def456',
              createdAt: '2024-01-01T00:00:00Z',
            },
            collections: [
              { collectionId: 1, collectionName: 'Essential Mods', orderIndex: 2 },
              { collectionId: 2, collectionName: 'Graphics Mods', orderIndex: 1 },
            ],
          },
        ],
        evaluationResult: {
          collectionId: 1,
          enabledFiles: [1, 2],
          disabledFiles: [],
          evaluationDetails: [
            { fileId: 1, enabled: true, appliedRules: [] },
            { fileId: 2, enabled: true, appliedRules: [] },
          ],
        },
      },
    },
  });
</script>

<script>
  import { Story } from '@storybook/blocks';
</script>

<!-- Default Collection Viewer -->
<Story name="Default" />

<!-- Single Collection -->
<Story
  name="Single Collection"
  args={{
    collectionIds: [1],
  }}
/>

<!-- Multiple Collections -->
<Story
  name="Multiple Collections"
  args={{
    collectionIds: [1, 2, 3],
  }}
/>

<!-- Without Evaluation -->
<Story
  name="Without Evaluation"
  args={{
    showEvaluation: false,
  }}
/>

<!-- Empty Collections -->
<Story
  name="Empty Collections"
  args={{
    collectionIds: [],
  }}
/>

<!-- Loading State -->
<Story
  name="Loading State"
  parameters={{
    mockData: {
      loading: true,
    },
  }}
/>

<!-- Error State -->
<Story
  name="Error State"
  parameters={{
    mockData: {
      error: 'Failed to load collection data',
    },
  }}
/>

<!-- With Evaluation Stats -->
<Story
  name="With Evaluation Stats"
  parameters={{
    mockData: {
      evaluationResult: {
        collectionId: 1,
        enabledFiles: [1],
        disabledFiles: [2],
        evaluationDetails: [
          { fileId: 1, enabled: true, appliedRules: [1] },
          { fileId: 2, enabled: false, appliedRules: [2] },
        ],
      },
    },
  }}
/>

<!-- Large Collection -->
<Story
  name="Large Collection"
  args={{
    collectionIds: [1],
  }}
  parameters={{
    mockData: {
      files: Array.from({ length: 50 }, (_, i) => ({
        fileId: i + 1,
        file: {
          id: i + 1,
          name: `mod${i + 1}.dll`,
          version: '1.0.0',
          size: 1024000,
          hash: `hash${i + 1}`,
          createdAt: '2024-01-01T00:00:00Z',
        },
        collections: [
          { collectionId: 1, collectionName: 'Large Collection', orderIndex: i + 1 },
        ],
      })),
    },
  }}
/>

<!-- Interaction Test: Sorting -->
<Story
  name="Sorting Interaction"
  play={async ({ canvas, userEvent }) => {
    // Test name sorting
    const nameSortButton = canvas.getByRole('button', { name: /по имени|name/i });
    await userEvent.click(nameSortButton);

    // Test order sorting
    const orderSortButton = canvas.getByRole('button', { name: /по порядку|order/i });
    await userEvent.click(orderSortButton);

    // Test collection sorting
    const collectionSortButton = canvas.getByRole('button', { name: /по коллекции|collection/i });
    await userEvent.click(collectionSortButton);
  }}
/>

<!-- Interaction Test: Toggle Disabled Files -->
<Story
  name="Toggle Disabled Files"
  play={async ({ canvas, userEvent }) => {
    const toggle = canvas.getByRole('checkbox', { name: /показать отключенные|show disabled/i });
    await userEvent.click(toggle);
  }}
/>

<!-- Interaction Test: File Actions -->
<Story
  name="File Actions"
  play={async ({ canvas, userEvent }) => {
    // Test file selection
    const fileItem = canvas.getByText('mod1.dll');
    await userEvent.click(fileItem);

    // Test context menu if available
    const fileItems = canvas.getAllByRole('listitem');
    if (fileItems.length > 0) {
      await userEvent.pointer({ keys: '[MouseRight]', target: fileItems[0] });
    }
  }}
/>

<!-- Accessibility Test -->
<Story
  name="Accessibility"
  play={async ({ canvas }) => {
    // Check for proper table structure
    const table = canvas.getByRole('table');
    await expect(table).toBeInTheDocument();

    // Check for table headers
    const headers = canvas.getAllByRole('columnheader');
    await expect(headers.length).toBeGreaterThan(0);

    // Check for proper button labels
    const buttons = canvas.getAllByRole('button');
    buttons.forEach(button => {
      expect(button).toHaveAccessibleName();
    });

    // Check for loading announcements
    if (canvas.queryByText(/загрузка|loading/i)) {
      const loadingElement = canvas.getByText(/загрузка|loading/i);
      await expect(loadingElement).toHaveAttribute('aria-live');
    }
  }}
/>

<!-- Performance Test -->
<Story
  name="Performance Test"
  args={{
    collectionIds: Array.from({ length: 10 }, (_, i) => i + 1),
  }}
  parameters={{
    mockData: {
      files: Array.from({ length: 1000 }, (_, i) => ({
        fileId: i + 1,
        file: {
          id: i + 1,
          name: `mod${i + 1}.dll`,
          version: '1.0.0',
          size: 1024000,
          hash: `hash${i + 1}`,
          createdAt: '2024-01-01T00:00:00Z',
        },
        collections: [
          { collectionId: (i % 10) + 1, collectionName: `Collection ${(i % 10) + 1}`, orderIndex: i + 1 },
        ],
      })),
    },
  }}
/>

