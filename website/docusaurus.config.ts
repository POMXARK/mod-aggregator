import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

const config: Config = {
  title: 'Mod Aggregator',
  tagline: 'Десктопное приложение для агрегации модов из различных сайтов',
  favicon: 'img/favicon.ico',

  // Production URL
  url: 'https://your-docs-site.com',
  baseUrl: '/',

  organizationName: 'mod-aggregator',
  projectName: 'mod-aggregator',

  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',

  i18n: {
    defaultLocale: 'ru',
    locales: ['ru'],
  },

  presets: [
    [
      'classic',
      {
        docs: {
          sidebarPath: './sidebars.ts',
          editUrl: 'https://github.com/your-org/mod-aggregator/tree/main/website/',
          routeBasePath: '/',
        },
        blog: false,
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    navbar: {
      title: 'Mod Aggregator',
      logo: {
        alt: 'Mod Aggregator Logo',
        src: 'img/logo.svg',
      },
      items: [
        {
          type: 'docSidebar',
          sidebarId: 'docs',
          position: 'left',
          label: 'Документация',
        },
        {
          href: 'https://github.com/your-org/mod-aggregator',
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'dark',
      links: [
        {
          title: 'Документация',
          items: [
            {
              label: 'Архитектура',
              to: '/architecture/overview',
            },
            {
              label: 'API',
              to: '/api/tauri-commands',
            },
          ],
        },
        {
          title: 'Разработка',
          items: [
            {
              label: 'GitHub',
              href: 'https://github.com/your-org/mod-aggregator',
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} Mod Aggregator. Built with Docusaurus.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
      additionalLanguages: ['rust', 'typescript', 'bash'],
    },
  } satisfies Preset.ThemeConfig,
};

export default config;



































