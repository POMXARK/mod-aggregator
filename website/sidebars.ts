import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

const sidebars: SidebarsConfig = {
  docs: [
    'intro',
    {
      type: 'category',
      label: 'Архитектура',
      items: [
        'architecture/overview',
        'architecture/data-flow',
      ],
    },
    {
      type: 'category',
      label: 'API',
      items: [
        'api/tauri-commands',
      ],
    },
    {
      type: 'category',
      label: 'Руководства',
      items: [
        'guides/development',
        'guides/testing',
      ],
    },
    {
      type: 'category',
      label: 'Компоненты',
      items: [
        'components/parser-builder',
        'components/page-viewer',
      ],
    },
    {
      type: 'category',
      label: 'Руководство пользователя',
      items: [
        'user-guide/README',
        'user-guide/getting-started',
        'user-guide/files',
      ],
    },
  ],
};

export default sidebars;

