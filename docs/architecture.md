# Vanguard Landing Page - Architecture & Design Decisions

> Documentation of architectural choices and design rationale.

---

## 1. Technology Choice: Leptos (Rust → WASM)

### Why Not React/Next.js?
- **Brand Alignment**: Portfolio should demonstrate systems-level expertise
- **Differentiation**: Few portfolios are built in Rust+WASM
- **Interview Talking Point**: "I built my portfolio in pure Rust, zero JavaScript"

### Why Leptos Specifically?
| Framework | Pros | Cons |
|-----------|------|------|
| **Leptos** ✓ | Signals-based, mature, SSR support | Newer ecosystem |
| Yew | Large community | Class-based components (older pattern) |
| Dioxus | Multi-platform | More complex for simple sites |

**Decision**: Leptos 0.6 with CSR (Client-Side Rendering) for simplicity.

---

## 2. Design Philosophy

### Terminal Theme Rationale
- **Target Audience**: Engineering hiring managers, technical recruiters
- **Message**: "I understand systems, not just frontends"
- **Aesthetic**: Professional but distinctive (not the same Tailwind template everyone uses)

### Interactive Terminal
The terminal is **fully functional**, not decorative:
- Real text input with keyboard handling
- Command parsing and response generation
- History log that persists during session
- Easter eggs to show personality

| Command Category | Examples |
|-----------------|----------|
| **Navigation** | `projects`, `skills`, `contact` |
| **Info** | `help`, `about` |
| **Utility** | `clear` |
| **Easter Eggs** | `ls`, `whoami`, `sudo hire me`, `rm -rf /` |

### Color Palette
| Color | Hex | Usage |
|-------|-----|-------|
| Background | `#0a0a0f` | Primary dark |
| Gold | `#D4AF37` | Accents, CTAs, brand |
| Green | `#00ff88` | Terminal commands |
| Text | `#e8e8e8` | Primary text |
| Dim | `#7a7a8a` | Secondary text |

### Typography
- **Headers/Body**: Inter (Google Fonts)
- **Terminal/Code**: SF Mono, Fira Code, Consolas (system stack)

---

## 3. Component Architecture

```
App
├── Hero (Terminal window)
│   ├── TerminalHeader (buttons, title)
│   └── TerminalBody (ASCII logo, commands)
├── Projects
│   ├── FilterTabs (category selection)
│   └── ProjectGrid
│       └── ProjectCard (repeated)
├── TechStack
│   └── TechItem (repeated)
└── Footer
```

### State Management
- **Filter State**: `create_signal("All")` — Leptos signal for selected category
- **Project Data**: Static `Vec<Project>` defined in code
- **No External State**: Everything is derived from props or local signals

---

## 4. Scalability Design

### Adding Projects
Projects are defined as a `Vec<Project>` in `get_projects()`. To add:
1. Append to vector
2. Assign a category
3. Rebuild

No database needed — source of truth is code.

### Adding Categories
1. Add to `categories` vec in `Projects` component
2. Use new category string in project definitions

### Future Considerations
- **Multiple Pages**: Leptos Router can be added for `/about`, `/contact`
- **CMS Integration**: Could fetch from JSON/API if project count grows significantly
- **i18n**: Leptos has localization crates if needed

---

## 5. Performance Decisions

### WASM Optimization
```toml
[profile.release]
opt-level = "z"    # Optimize for size
lto = true         # Link-time optimization
codegen-units = 1  # Single codegen unit for better optimization
```

### Asset Loading
- **Fonts**: Google Fonts with `preconnect` for faster loading
- **No Images**: Emoji icons instead of asset files (fewer HTTP requests)
- **CSS**: Single file, no framework overhead

### Bundle Size Target
- WASM: < 200KB gzipped
- CSS: < 10KB
- Total: < 250KB (vs 500KB+ typical React bundle)

---

## 6. Deployment Strategy

### GitHub Pages
1. `trunk build --release --public-url "/vanguard-landing/"`
2. Push `dist/` to `gh-pages` branch
3. Configure repo settings

### Why Not Vercel/Netlify?
- GitHub Pages is simpler for static WASM
- No build time needed (pre-built)
- Free, fast CDN

### Custom Domain (Optional)
- Add CNAME file to `dist/`
- Configure DNS

---

## 7. Accessibility Notes

### Current Status
- ✅ Semantic HTML (`section`, `footer`, `h1`-`h3`)
- ✅ Focus styles on buttons/links
- ✅ Color contrast passes WCAG AA
- ⚠️ ASCII art has no alt text (decorative only)

### Future Improvements
- Add `aria-label` to icon-only buttons
- Screen reader testing with NVDA/VoiceOver

---

## 8. File Reference

| File | Lines | Purpose |
|------|-------|---------|
| `main.rs` | ~10 | Entry point, mounts App |
| `app.rs` | ~210 | All components |
| `lib.rs` | ~1 | Module exports |
| `main.css` | ~350 | All styling |
| `index.html` | ~15 | HTML shell with Trunk hooks |
| `Cargo.toml` | ~15 | Dependencies |
| `Trunk.toml` | ~10 | Build config |

---

*Last Updated: 2026-01-03*
