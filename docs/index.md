---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "Learning Rust"
  text: "From The Book"
  tagline: With Mouli Bheemaneti...
  actions:
    - theme: brand
      text: Get Started
      link: /get-started
    - theme: alt
      text: Official Site
      link: https://doc.rust-lang.org/stable/book/

---

## Project Overview 🚀

- 📖 I'll document my learnings as I read through the [Official Rust Programming Book](https://doc.rust-lang.org/stable/book/).
- ✍️ Concise notes, examples, and key points.
- 💡 Includes explanations, definitions, keypoints, and code samples.
- 📝 New terms and concepts noted

<script lang="ts" setup>
import {
  VPTeamPage,
  VPTeamPageTitle,
  VPTeamMembers
} from 'vitepress/theme'

const members = [
  {
    avatar: 'https://www.github.com/moulibheemaneti.png',
    name: 'Mouli Bheemaneti',
    title: 'Creator',
    links: [
      { icon: 'github', link: 'https://github.com/moulibheemaneti' },
      { icon: 'linkedin', link: 'https://linkedin.com/in/moulibheemaneti' }
    ]
  },
]
</script>

<VPTeamPage>
  <VPTeamPageTitle>
    <template #title>
      Core Team
    </template>
  </VPTeamPageTitle>
  <VPTeamMembers :members />
</VPTeamPage>
