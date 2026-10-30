import './styles.css'

// Theme toggle — the initial theme is set by an inline script in index.html.
const root = document.documentElement
const toggle = document.getElementById('theme-toggle')
const syncLabel = () => {
  const next = root.dataset.theme === 'light' ? 'dark' : 'light'
  toggle?.setAttribute('aria-label', `Switch to ${next} theme`)
  toggle?.setAttribute('title', `Switch to ${next} theme`)
}
syncLabel()
toggle?.addEventListener('click', () => {
  root.dataset.theme = root.dataset.theme === 'dark' ? 'light' : 'dark'
  try {
    localStorage.setItem('theme', root.dataset.theme)
  } catch {
    // Storage can be unavailable (private mode); the toggle still works.
  }
  syncLabel()
})

// Code tabs.
for (const group of document.querySelectorAll<HTMLElement>('[data-tabs]')) {
  const tabs = [...group.querySelectorAll<HTMLButtonElement>('[role="tab"]')]
  const select = (index: number) => {
    tabs.forEach((tab, i) => {
      const active = i === index
      tab.classList.toggle('active', active)
      tab.setAttribute('aria-selected', String(active))
      const panel = document.getElementById(tab.getAttribute('aria-controls') ?? '')
      if (panel) panel.hidden = !active
    })
  }
  tabs.forEach((tab, i) => {
    tab.addEventListener('click', () => select(i))
    tab.addEventListener('keydown', (e) => {
      const step = e.key === 'ArrowRight' ? 1 : e.key === 'ArrowLeft' ? -1 : 0
      if (!step) return
      const next = (i + step + tabs.length) % tabs.length
      select(next)
      tabs[next]?.focus()
    })
  })
}
