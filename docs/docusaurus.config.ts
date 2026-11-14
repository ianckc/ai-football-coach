import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

const config: Config = {
  title: 'AI Football Coach',
  tagline:
    'Use AI to help football coaches plan training sessions and improve their teams.',
  favicon: 'img/favicon.ico',

  future: {
    v4: true,
  },

  url: 'http://localhost:3001',
  baseUrl: '/',

  organizationName: 'ai-football-coach',
  projectName: 'ai-football-coach',

  onBrokenLinks: 'throw',

  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      'classic',
      {
        docs: {
          sidebarPath: './sidebars.ts',
        },
        blog: false,
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    image: 'img/docusaurus-social-card.jpg',
    colorMode: {
      respectPrefersColorScheme: true,
    },
    navbar: {
      title: 'AI Football Coach',
      logo: {
        alt: 'AI Football Coach Logo',
        src: 'img/logo.svg',
      },
      items: [
        {
          type: 'docSidebar',
          sidebarId: 'docsSidebar',
          position: 'left',
          label: 'Docs',
        },
      ],
    },
    footer: {
      style: 'dark',
      links: [
        {
          title: 'Docs',
          items: [
            {
              label: 'Introduction',
              to: '/docs/intro',
            },
            {
              label: 'Software factory',
              to: '/docs/software-factory',
            },
            {
              label: 'Backend API',
              to: '/docs/backend-api',
            },
            {
              label: 'Frontend',
              to: '/docs/frontend',
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} AI Football Coach. Built with Docusaurus.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
