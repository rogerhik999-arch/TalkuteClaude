<!--
Sync Impact Report:
- Version: [NEW] → 1.0.0
- Modified principles: N/A (initial creation)
- Added sections: All core principles, Technology Stack, Performance Standards, Development Workflow
- Removed sections: N/A
- Templates requiring updates:
  ✅ plan-template.md - Constitution Check section aligns with principles
  ✅ spec-template.md - Requirements align with AI-native and cross-platform principles
  ✅ tasks-template.md - Task organization supports modular architecture
- Follow-up TODOs: None
-->

# TalkuteClaude Constitution
<!-- AI-Powered Context-Aware Input Enhancement System -->

## Core Principles

### I. Rust-First Core (NON-NEGOTIABLE)
All core logic, performance-critical components, and system integrations MUST be implemented in Rust.

**Rules**:
- Business logic, AI integration, context detection, and input processing in Rust
- Rust libraries MUST be self-contained, independently testable, and documented
- FFI bindings for Flutter MUST be minimal, type-safe, and well-documented
- Zero-cost abstractions preferred; unsafe code requires explicit justification

**Rationale**: Rust provides memory safety, fearless concurrency, and native performance required for low-overhead cross-platform operation. This ensures consistent behavior across all target platforms (Windows, Mac, Linux, iOS, Android) without runtime overhead.

### II. Flutter Cross-Platform UI (NON-NEGOTIABLE)
All user interface components MUST be implemented in Flutter for unified cross-platform experience.

**Rules**:
- Single Flutter codebase for mobile (iOS, Android) and desktop (Windows, Mac, Linux)
- Platform-specific adaptations via conditional rendering, not separate codebases
- UI state management MUST be reactive and testable
- Accessibility compliance (WCAG 2.1 AA) required for all UI components

**Rationale**: Flutter enables true write-once-run-anywhere UI with native performance and consistent user experience. This minimizes maintenance burden and ensures feature parity across all platforms.

### III. AI-Native Architecture (NON-NEGOTIABLE)
LLM integration is a first-class architectural concern, not an afterthought.

**Rules**:
- Prompt engineering MUST be versioned, tested, and optimized
- LLM calls MUST be asynchronous, cancellable, and timeout-protected
- Fallback strategies required for offline/degraded AI service scenarios
- Context window management MUST be efficient (token counting, truncation strategies)
- AI model selection MUST be configurable (local models, cloud APIs, hybrid)

**Rationale**: AI is the core value proposition. Treating it as infrastructure ensures reliability, performance, and maintainability as models evolve.

### IV. Context-Aware Intelligence
System MUST recognize application context and user intent to optimize input enhancement.

**Rules**:
- Application detection via OS-level APIs (window title, process name, accessibility tree)
- Context classification (code editor, email, chat, document, terminal, etc.)
- User input pattern recognition (command, prose, code, data entry)
- Context-specific prompt templates for optimal LLM performance
- Privacy-first: No sensitive data logging; context detection local-only

**Rationale**: Generic input enhancement is insufficient. Context-aware optimization delivers 10x better results by tailoring AI behavior to user intent and application requirements.

### V. Minimal & Elegant Design (NON-NEGOTIABLE)
Start simple, add complexity only when justified. YAGNI principles strictly enforced.

**Rules**:
- Maximum 5 core modules: UI, Core Logic, AI Integration, Context Detection, Platform Adapters
- No premature abstractions; three instances before extracting pattern
- Configuration over code for extensibility
- Dependencies MUST be justified; prefer standard library solutions
- Code files <800 lines; functions <50 lines; nesting <4 levels

**Rationale**: Complexity is the enemy of maintainability. Minimal architecture reduces cognitive load, simplifies testing, and accelerates development.

### VI. Test-First Development (NON-NEGOTIABLE)
TDD mandatory for all core functionality. Tests written before implementation.

**Rules**:
- Red-Green-Refactor cycle strictly enforced
- Minimum 80% code coverage for Rust core
- Unit tests for all business logic
- Integration tests for AI pipeline and context detection
- E2E tests for critical user flows (input → enhancement → output)
- Tests MUST be fast (<100ms unit, <1s integration, <10s E2E)

**Rationale**: Test-first ensures correctness, enables fearless refactoring, and serves as living documentation. High-quality tests are non-negotiable for cross-platform reliability.

## Technology Stack Requirements

### Mandatory Technologies
- **Core**: Rust 1.75+ (stable channel)
- **UI**: Flutter 3.16+ (stable channel)
- **FFI**: flutter_rust_bridge for Rust-Flutter integration
- **AI**: Anthropic Claude API (primary), OpenAI API (fallback), local models (optional)
- **Build**: Cargo (Rust), Flutter build tools
- **Testing**: cargo test (Rust), flutter test (Flutter), integration_test (E2E)

### Platform-Specific Requirements
- **Windows**: Win32 API for context detection
- **Mac**: Cocoa/AppKit for context detection
- **Linux**: X11/Wayland for context detection
- **iOS**: UIKit accessibility APIs
- **Android**: AccessibilityService APIs

### Prohibited Technologies
- No Electron (violates performance requirements)
- No web-based UI frameworks (violates native performance requirement)
- No interpreted languages for core logic (violates performance requirement)

## Performance Standards

### Response Time Requirements
- Context detection: <50ms (p95)
- AI prompt preparation: <20ms (p95)
- LLM API call: <2s (p95, network-dependent)
- UI rendering: 60fps minimum, 120fps target
- Memory footprint: <100MB idle, <300MB active

### Resource Constraints
- CPU: <5% idle, <30% during AI processing
- Network: Efficient token usage, request batching where possible
- Battery: Minimal background activity, aggressive power management on mobile
- Disk: <50MB installation size (excluding AI models)

### Scalability Targets
- Support 1000+ input enhancements per day per user
- Handle 10+ concurrent context switches without degradation
- Graceful degradation under resource constraints

## Development Workflow

### Code Review Requirements
- All code MUST pass automated tests before review
- Security review required for: FFI boundaries, AI prompt injection risks, context data handling
- Performance review required for: Core Rust logic, AI integration, context detection
- Accessibility review required for: All UI components

### Quality Gates
- No compiler warnings (Rust: clippy, Flutter: analyzer)
- No security vulnerabilities (cargo audit, Flutter security scan)
- Test coverage ≥80% (Rust core)
- Documentation coverage 100% (public APIs)

### Commit Standards
- Conventional commits format: `<type>: <description>`
- Types: feat, fix, refactor, docs, test, chore, perf
- Atomic commits: one logical change per commit
- Commit messages in English

### Branching Strategy
- Main branch: production-ready code only
- Feature branches: `###-feature-name` format
- No direct commits to main; all changes via PR
- PR requires: passing tests, code review approval, constitution compliance check

## Governance

This constitution supersedes all other development practices and guidelines. Any deviation MUST be explicitly justified and documented.

### Amendment Process
1. Propose amendment with rationale and impact analysis
2. Review against project goals and technical constraints
3. Update dependent templates and documentation
4. Increment version according to semantic versioning
5. Document in Sync Impact Report

### Compliance Verification
- All PRs MUST include constitution compliance checklist
- Quarterly architecture review to verify adherence
- Complexity violations require explicit justification in plan.md

### Version Control
- **MAJOR**: Backward-incompatible principle changes or removals
- **MINOR**: New principles or materially expanded guidance
- **PATCH**: Clarifications, wording improvements, non-semantic changes

**Version**: 1.0.0 | **Ratified**: 2026-03-04 | **Last Amended**: 2026-03-04
