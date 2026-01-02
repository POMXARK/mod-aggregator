<script module>
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import { expect, fn } from '@storybook/test';

  import ModCard from './ModCard.svelte';

  const { Story } = defineMeta({
    component: ModCard,
    args: {
      mod: {
        id: 1,
        name: 'Test Mod',
        description: 'A test mod for demonstration',
        version: '1.0.0',
        author: 'Test Author',
        downloadUrl: 'https://example.com/download',
        imageUrl: 'https://via.placeholder.com/300x200',
        tags: ['utility', 'graphics'],
        dependencies: [],
        createdAt: '2024-01-01T00:00:00Z',
        updatedAt: '2024-01-15T00:00:00Z',
      },
      onSelect: fn(),
      onDownload: fn(),
      onViewDetails: fn(),
    },
    argTypes: {
      mod: {
        control: { type: 'object' },
      },
    },
  });
</script>

<script>
  import { Story } from '@storybook/blocks';
</script>

<!-- Default Mod Card -->
<Story name="Default" />

<!-- Mod Without Image -->
<Story
  name="Without Image"
  args={{
    mod: {
      id: 2,
      name: 'Mod Without Image',
      description: 'This mod has no image',
      version: '2.0.0',
      author: 'No Image Author',
      downloadUrl: 'https://example.com/download2',
      tags: ['essential'],
      dependencies: [],
      createdAt: '2024-01-01T00:00:00Z',
      updatedAt: '2024-01-15T00:00:00Z',
    },
  }}
/>

<!-- Mod With Long Description -->
<Story
  name="Long Description"
  args={{
    mod: {
      id: 3,
      name: 'Mod With Very Long Description',
      description: 'This is a very long description that should test how the component handles text overflow and wrapping. It contains multiple sentences and should demonstrate proper text truncation or expansion behavior in the UI.',
      version: '1.5.0',
      author: 'Long Description Author',
      downloadUrl: 'https://example.com/download3',
      imageUrl: 'https://via.placeholder.com/300x200',
      tags: ['long', 'description', 'test'],
      dependencies: [],
      createdAt: '2024-01-01T00:00:00Z',
      updatedAt: '2024-01-15T00:00:00Z',
    },
  }}
/>

<!-- Mod With Many Tags -->
<Story
  name="Many Tags"
  args={{
    mod: {
      id: 4,
      name: 'Mod With Many Tags',
      description: 'This mod has many tags',
      version: '1.0.0',
      author: 'Tag Author',
      downloadUrl: 'https://example.com/download4',
      imageUrl: 'https://via.placeholder.com/300x200',
      tags: ['tag1', 'tag2', 'tag3', 'tag4', 'tag5', 'tag6', 'tag7', 'tag8', 'tag9', 'tag10'],
      dependencies: [],
      createdAt: '2024-01-01T00:00:00Z',
      updatedAt: '2024-01-15T00:00:00Z',
    },
  }}
/>

<!-- Recently Updated Mod -->
<Story
  name="Recently Updated"
  args={{
    mod: {
      id: 5,
      name: 'Recently Updated Mod',
      description: 'This mod was updated recently',
      version: '1.0.0',
      author: 'Recent Author',
      downloadUrl: 'https://example.com/download5',
      imageUrl: 'https://via.placeholder.com/300x200',
      tags: ['recent', 'update'],
      dependencies: [],
      createdAt: '2024-01-01T00:00:00Z',
      updatedAt: new Date().toISOString(), // Today
    },
  }}
/>

<!-- Mod With Dependencies -->
<Story
  name="With Dependencies"
  args={{
    mod: {
      id: 6,
      name: 'Mod With Dependencies',
      description: 'This mod requires other mods',
      version: '1.0.0',
      author: 'Dependency Author',
      downloadUrl: 'https://example.com/download6',
      imageUrl: 'https://via.placeholder.com/300x200',
      tags: ['dependencies'],
      dependencies: [
        { id: 1, name: 'Required Mod 1', version: '1.0.0' },
        { id: 2, name: 'Required Mod 2', version: '2.0.0' },
      ],
      createdAt: '2024-01-01T00:00:00Z',
      updatedAt: '2024-01-15T00:00:00Z',
    },
  }}
/>

<!-- Interaction Test: Select Mod -->
<Story
  name="Select Interaction"
  play={async ({ args, canvas, userEvent }) => {
    const card = canvas.getByRole('button', { name: /Test Mod/ });
    await userEvent.click(card);

    await expect(args.onSelect).toHaveBeenCalledWith(args.mod);
  }}
/>

<!-- Interaction Test: Download Mod -->
<Story
  name="Download Interaction"
  play={async ({ args, canvas, userEvent }) => {
    const downloadButton = canvas.getByRole('button', { name: /скачать|download/i });
    await userEvent.click(downloadButton);

    await expect(args.onDownload).toHaveBeenCalledWith(args.mod);
  }}
/>

<!-- Interaction Test: View Details -->
<Story
  name="View Details Interaction"
  play={async ({ args, canvas, userEvent }) => {
    const detailsButton = canvas.getByRole('button', { name: /подробнее|details/i });
    await userEvent.click(detailsButton);

    await expect(args.onViewDetails).toHaveBeenCalledWith(args.mod);
  }}
/>

<!-- Loading State -->
<Story
  name="Loading State"
  args={{
    isLoading: true,
  }}
/>

<!-- Error State -->
<Story
  name="Error State"
  args={{
    error: 'Failed to load mod information',
  }}
/>

<!-- Accessibility Test -->
<Story
  name="Accessibility"
  play={async ({ canvas }) => {
    // Check for proper heading structure
    const heading = canvas.getByRole('heading', { level: 3 });
    await expect(heading).toBeInTheDocument();

    // Check for image alt text
    const image = canvas.getByRole('img');
    await expect(image).toHaveAttribute('alt');

    // Check for button accessibility
    const buttons = canvas.getAllByRole('button');
    await expect(buttons.length).toBeGreaterThan(0);

    // Check for proper ARIA labels if needed
    buttons.forEach(button => {
      expect(button).toHaveAttribute('aria-label');
    });
  }}
/>

