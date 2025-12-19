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
  ],
};

export default sidebars;

