# Browser Automation (Experimental - V3)

**Status**: Experimental / Future Feature
**Target Release**: v3.0+ (2027+)
**Priority**: Low (focus is on ContextOS + Memory Layer for v1.0-v2.0)

## Overview

This directory contains documentation for **browser automation and agentic workflow capabilities**—features that are planned for future versions of Robert but are **not part of the core v1.0-v2.0 product**.

The initial product vision positioned Robert as a browser automation tool. **This direction has been superseded** by the **ContextOS / Memory Layer for AI** positioning. Browser automation capabilities have been moved to experimental status and may be reintroduced as part of **agentic workflows in v3.0+**.

## Why This Moved to V3

Robert's core value proposition is:
1. **ContextOS**: Sophisticated AI memory control
2. **Universal Indexing**: Cross-tool knowledge graphs (Jira, Slack, Figma, etc.)
3. **Agentic Execution**: Domain-specific agents that execute workflows

Browser automation is a **means to an end** for agentic execution, not the core product. In v1.0-v2.0, we focus on:
- Memory management and context control
- Knowledge graph construction (GraphRAG)
- Multi-provider reasoning
- Team collaboration and context sharding

**In v3.0+**, we may introduce browser/desktop automation as **execution capabilities for agents**:
- Research agents scraping competitor websites
- Meeting prep agents pulling data from web apps
- Workflow agents automating multi-step tasks across tools

## Documents in This Directory

### Core Browser Automation
- **script_format.md** - CDP-based browser automation script format
- **chat_ui.md** - Injected chat interface for browser automation
- **profiles.md** - User profiles and browser profile management
- **screenshot_testing.md** - Visual regression testing for browser automation

### Workflow Learning System
- **step_frame_implementation.md** - Agent workflow learning and frame capture
- **visualdom_format.md** - Visual DOM representation for agents

### Privacy & Security
- **user_profiles_privacy.md** - Privacy architecture for browser automation profiles

## Relationship to Current Product

While these features are experimental, they inform the **v1.5 domain agents** architecture:

### V1.5 Domain Agents (Memory + Simple Execution)
- **Pricing Agent**: Generates quotes using GraphRAG (no browser automation needed)
- **Research Agent**: Summarizes competitor data (user provides data, agent processes)
- **Meeting Prep Agent**: Reads calendar + docs, generates briefing (no scraping)
- **Writing Agent**: Drafts reports using brand voice + company data (no automation)

### V3.0 Advanced Agentic Workflows (Memory + Browser Automation)
- **Research Agent**: Autonomously scrapes competitor websites, monitors changes
- **Data Collection Agent**: Pulls data from web apps without APIs
- **Workflow Agent**: Executes multi-step tasks across browser-based tools
- **Testing Agent**: Visual regression testing, screenshot comparison

## Technical Notes

The browser automation architecture uses:
- **Chrome DevTools Protocol (CDP)** via `chromiumoxide`
- **Ephemeral browser profiles** for privacy
- **AI-generated CDP scripts** (Claude writes automation)
- **Workflow learning system** (agents learn navigation paths)

This infrastructure remains valuable for **v3.0+ execution capabilities** but is not critical path for **v1.0-v2.0 memory layer**.

## For Developers

If you're working on Robert v1.0-v2.0:
- **Ignore this directory** - focus on ContextOS and GraphRAG
- **Do not implement** browser automation features
- **Reference only** if designing agent execution architecture for v3.0

If you're exploring v3.0+ agentic workflows:
- **Review these specs** as foundation for execution layer
- **Update as needed** based on v1.0-v2.0 learnings
- **Integrate with** ContextOS and domain agents architecture

## Status Updates

- **2025-12-02**: Moved browser automation docs to experimental directory
- **Target v3.0 (2027+)**: Re-evaluate browser automation as execution layer for agents

---

**For questions**: See main [prd.md](../prd.md) and [pitch.md](../pitch.md) for current product direction.
