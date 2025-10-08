# Changelog

## 2025-10-08 - Product Vision Update & Phase 0 Complete

### Major Product Vision Changes
- **New Positioning**: "Automation for Everyone" - targeting non-programmers
- **Voice-Driven Creation**: Users talk through automation, AI writes Markdown scripts
- **Markdown Scripts**: Replaced YAML with human-readable Markdown format (inspired by Claude agents)
- **Local-First AI**: 100% on-device inference by default, optional cloud with auto-obfuscation
- **Privacy Protection**: Multi-layer data obfuscation (text + images) before any cloud transmission
- **GTM Strategy**: Tesla approach - launch on macOS for discerning users first

### Added
- **COMPETITION.md** - Comprehensive competitive analysis with Tesla GTM strategy
- **SCRIPT_FORMAT.md** - Markdown script specification with voice creation examples
- **SAFETY_PRIVACY.md** - Local-first AI and data obfuscation architecture

### Phase 0 Complete ✅
- **Chrome DevTools Protocol (CDP)** via spider_chrome
- **Auto-download Chrome** - Zero manual setup required (~150MB, cached)
- **Headless mode** and CI/CD support
- **E2E tests** with visible window (5 second delay) and CI mode
- **CLI tool** with protocol-optional URLs

## 2025-10-08 - Documentation Consolidation

### Added
- **Phase 0: CLI Prototype** - New prototyping phase before desktop app
  - 2-3 day timeline
  - Validates browser automation approach
  - Creates reusable library crate
- **PHASE_0_TASKS.md** - Detailed task breakdown for CLI implementation
- **Cargo workspace structure** - Multi-crate organization:
  - `robert-webdriver` - Core automation library
  - `robert-cli` - CLI tool binary
  - `robert-app` - Tauri desktop app (future)

### Changed
- **Renamed documents** for clarity:
  - `browser-automation-prd-revised.md` → `PRD.md`
  - `browser-automation-revised-plan.md` → `IMPLEMENTATION_PLAN.md`
- **Frontend framework**: Confirmed Svelte + TypeScript + Tailwind CSS
- **Memory footprint spec**: Clarified to <100MB (app only), <1.5GB (app + browser)
- **Success criteria**: Changed checkboxes from `[x]` to `[ ]` (planning phase)
- **Editor choice**: CodeMirror instead of Monaco (lighter weight)

### Fixed
- React → Svelte references in PRD (2 locations)
- Inconsistent memory footprint specifications
- Success criteria marked as complete when still in planning

### Moved to Archive
- `browser-automation-prd.md` (original Windows-focused PRD)
- `browser-automation-implementation-plan.md` (original Windows plan)
- `consistency-review.md` (review document, no longer needed)

### Updated
- **README.md** - Reflects current project status and Phase 0 focus
- **PRD.md** - Fixed React references, clarified memory specs
- **IMPLEMENTATION_PLAN.md** - Added Phase 0, fixed Svelte references

---

## Project Status

**Phase**: Phase 0 Complete ✅ → Phase 1 Starting

**Next Action**: Begin Phase 1 - Tauri desktop app with voice-driven Markdown script creation

---

## Document Structure (Current)

```
robert/
├── PRD.md                    # Product Requirements
├── IMPLEMENTATION_PLAN.md    # Full roadmap
├── COMPETITION.md            # Competitive analysis & GTM strategy
├── SCRIPT_FORMAT.md          # Markdown script specification
├── SAFETY_PRIVACY.md         # Privacy & obfuscation architecture
├── PHASE_0_TASKS.md          # CLI prototype tasks (reference)
├── CHANGELOG.md              # This file
├── README.md                 # Project overview
├── archive/                  # Deprecated docs
└── crates/                   # Phase 0 complete ✅
    ├── robert-webdriver/     # Core automation library (CDP)
    ├── robert-cli/           # CLI tool
    └── robert-app/           # Tauri app (placeholder)
```

---

## Technology Stack (Confirmed)

- **Desktop Framework**: Tauri 2.0 (planned for Phase 1)
- **Frontend**: Svelte + TypeScript + Tailwind CSS
- **Backend**: Rust 1.70+
- **Browser Automation**: spider_chrome (Chrome DevTools Protocol)
- **Chrome Management**: Auto-download via spider_chromiumoxide_fetcher
- **Script Format**: Markdown with YAML frontmatter (inspired by Claude agents)
- **AI Integration**: Local-first inference + optional cloud
- **Target Platform**: macOS first (Tesla GTM strategy)

---

## Key Decisions

1. **Voice-Driven over Manual Coding**: Talk through automation, AI writes scripts
2. **Markdown over YAML**: Human-readable format, no scary syntax
3. **Local-First AI**: 100% on-device by default, privacy-first approach
4. **CDP over WebDriver**: Direct Chrome control via spider_chrome
5. **Auto-download Chrome**: Zero manual setup required
6. **macOS First (Tesla GTM)**: Target discerning users, signal quality
7. **Svelte over React**: Smaller bundle size, better performance
8. **Multi-crate Workspace**: Separation of concerns, reusable library

---

## Timeline

- **Phase 0**: Complete ✅ (CLI prototype with CDP and auto-download)
- **Phase 1-7**: 7 weeks (macOS desktop app with voice-driven creation)
- **v1.5**: Months 3-4 (Linux headless + Docker)
- **v2.0+**: Months 6+ (Multi-browser, Windows, visual builder)
