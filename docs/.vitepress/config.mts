import { defineConfig } from 'vitepress';

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "Learning Rust",
  description: "A docs site for learning rust, from the book.",
  base: '/learn-rust/',

  // Enable clean URLs (remove .html from generated links)
  cleanUrls: true,

  // Rewrites map source files -> output paths (use relative file paths, no leading slash)
  rewrites: {
    "chapter00/foreword.md": "foreword.md",
    "chapter00/introduction.md": "introduction.md",
    "chapter01/11-installation.md": "chapter/1/installation.md",
    "chapter01/12-hello-world.md": "chapter/1/hello-world.md",
    "chapter01/13-hello-cargo.md": "chapter/1/hello-cargo.md",
    "chapter02/guessing-game.md": "chapter/2/guessing-game.md",
  },

  themeConfig: {
    search: {
      provider: 'local'
    },
    footer: {
      message: 'Created by Mouli Bheemaneti',
      copyright: 'Made with ❤️ for Rust 🦀 — Happy Learning!'
    },
    nav: [
      { text: 'Home', link: '/' },
      // removed .html from this link
      { text: 'Chapters', link: '/chapter/1/installation' },
      { text: 'Official Book', link: 'https://doc.rust-lang.org/stable/book/' },
    ],

    sidebar: [
      {
        items: [
          { text: 'Get Started!', link: '/get-started' },
          { text: 'Foreword', link: '/foreword' },
          { text: 'Introduction', link: '/introduction' },
          {
            text: 'Chapter 1',
            collapsible: true,
            collapsed: false,
            items: [
              { text: '1.1. Installation', link: '/chapter/1/installation' },
              { text: '1.2. Hello, World!', link: '/chapter/1/hello-world' },
              { text: '1.3. Hello, Cargo!', link: '/chapter/1/hello-cargo' },
            ]
          },
          { text: 'Chapter 2', link: '/chapter/2/guessing-game' },
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/vuejs/vitepress' }
    ]
  }
})
