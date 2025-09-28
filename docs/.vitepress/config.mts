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
    "chapter03/31-variables-and-mutability.md": "chapter/3/variables-and-mutability.md",
    "chapter03/32-data-types.md": "chapter/3/data-types.md",
    "chapter03/33-functions.md": "chapter/3/functions.md",
    "chapter03/34-comments.md": "chapter/3/comments.md",
    "chapter04/41-ownership.md": "chapter/4/ownership.md",
    "chapter04/42-references-and-borrowing.md": "chapter/4/references-and-borrowing.md",
    "chapter04/43-slice-type.md": "chapter/4/slice-type.md",
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
          {
            text: 'Chapter 3',
            collapsible: true,
            collapsed: false,
            items: [
              { text: '3.1. Variables and Mutability', link: '/chapter/3/variables-and-mutability' },
              { text: '3.2. Data Types', link: '/chapter/3/data-types' },
              { text: '3.3. Functions', link: '/chapter/3/functions' },
              { text: '3.4. Comments', link: '/chapter/3/comments' },
            ]
          },
          {
            text: 'Chapter 4',
            collapsible: true,
            collapsed: false,
            items: [
              { text: '4.1. Ownership', link: '/chapter/4/ownership' },
              { text: '4.2. References and Borrowing', link: '/chapter/4/references-and-borrowing' },
              { text: '4.3. Slice Type', link: '/chapter/4/slice-type' },
            ]
          },
          { text: 'Miscellaneous', link: '/miscellaneous' },
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/vuejs/vitepress' }
    ]
  }
})
