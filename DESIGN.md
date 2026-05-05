---
name: Local AI
description: Civic, utility-first desktop control for local AI services
colors:
  canvas: "#eef2f5"
  surface: "#fcfcfd"
  line: "#d5dce3"
  text: "#1b2430"
  muted: "#5e6a79"
  accent: "#355c7d"
  accent-strong: "#29455f"
  success: "#2f6b57"
  warning: "#8a6431"
  danger: "#9a3d3d"
typography:
  display:
    fontFamily: "Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif"
    fontSize: "32px"
    fontWeight: 600
    lineHeight: 1.15
    letterSpacing: "-0.02em"
  headline:
    fontFamily: "Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif"
    fontSize: "20px"
    fontWeight: 600
    lineHeight: 1.25
    letterSpacing: "-0.01em"
  title:
    fontFamily: "Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif"
    fontSize: "16px"
    fontWeight: 600
    lineHeight: 1.4
    letterSpacing: "0"
  body:
    fontFamily: "Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif"
    fontSize: "14px"
    fontWeight: 400
    lineHeight: 1.55
    letterSpacing: "0"
  label:
    fontFamily: "Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif"
    fontSize: "12px"
    fontWeight: 600
    lineHeight: 1.2
    letterSpacing: "0.12em"
rounded:
  sm: "8px"
  md: "12px"
  lg: "20px"
  xl: "28px"
  pill: "9999px"
spacing:
  xs: "8px"
  sm: "12px"
  md: "16px"
  lg: "24px"
  xl: "32px"
components:
  button-primary:
    backgroundColor: "{colors.accent}"
    textColor: "{colors.surface}"
    rounded: "{rounded.md}"
    padding: "12px 16px"
  button-secondary:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.text}"
    rounded: "{rounded.md}"
    padding: "12px 16px"
  button-danger:
    backgroundColor: "{colors.danger}"
    textColor: "{colors.surface}"
    rounded: "{rounded.md}"
    padding: "12px 16px"
  field:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.text}"
    rounded: "{rounded.md}"
    padding: "12px 12px"
  panel:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.text}"
    rounded: "{rounded.lg}"
    padding: "24px"
  status-badge:
    backgroundColor: "{colors.canvas}"
    textColor: "{colors.accent}"
    rounded: "{rounded.pill}"
    padding: "4px 10px"
  nav-pill-active:
    backgroundColor: "{colors.text}"
    textColor: "{colors.surface}"
    rounded: "{rounded.pill}"
    padding: "10px 16px"
  nav-pill-inactive:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.muted}"
    rounded: "{rounded.pill}"
    padding: "10px 16px"
---

# Design System: Local AI

## Overview

**Creative North Star: "The Civic Counter"**

This redesign is deliberately plain. It should feel like a public-service desk, not a startup cockpit: direct, procedural, calm, and difficult to misunderstand. The system uses one restrained accent, a paper-like neutral field, and standard controls that disappear into the task instead of asking for attention.

The experience must read as administrative and trustworthy at a glance. Simple Mode stays sparse and centered, with enough visual weight to feel stable, but not enough ornament to imply novelty. Developer Mode may become denser, but it should still feel like the same system opened wider, not a different product bolted on afterward.

It explicitly rejects the product anti-references in `PRODUCT.md`: startup dashboards, AI demo apps, crypto terminals, gamer UIs, glossy consumer products, flashy gradients, neon accents, playful mascots, dense analytics panels, over-designed empty states, and overly clever copy. If a screen looks promotional, it has gone wrong.

Key Characteristics:
- Calm, procedural, and intentionally unglamorous
- One accent color used for action and state, not decoration
- Paper-like surfaces over a cool slate canvas
- Standard controls and predictable layouts
- Dense enough for operators, simple enough for end users

## Colors

The palette is restrained and civic, with a cool blue-gray accent and semantic colors that only appear when they mean something.

### Primary
- **Civic Blue** (`#355c7d`): The only active accent in the system. Use it for primary actions, active navigation, and selected state.
- **Deep Civic Blue** (`#29455f`): Hover and emphasis state for the accent, never a decorative wash.

### Neutral
- **Paper Canvas** (`#eef2f5`): The outer application field and large background planes.
- **Document Surface** (`#fcfcfd`): Panels, cards, tables, and control surfaces.
- **Divider Gray** (`#d5dce3`): Borders, table rules, and form outlines.
- **Primary Text** (`#1b2430`): Body copy, headings, and important labels.
- **Secondary Text** (`#5e6a79`): Supporting copy, helper text, and metadata.

### Semantic
- **Service Green** (`#2f6b57`): Success, healthy states, and safe confirmation.
- **Notice Amber** (`#8a6431`): Warnings, waiting states, and partial readiness.
- **Alert Red** (`#9a3d3d`): Errors, failed validation, and attention-required states.

### Named Rules
**The One Accent Rule.** Civic Blue is reserved for actions, selection, and active state. If it starts decorating surfaces, the system has drifted into UI theater.

**The Paper-First Rule.** Surfaces stay quiet and light, with borders doing more of the structural work than shadows.

## Typography

**Display Font:** Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif  
**Body Font:** Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif  
**Label Font:** Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif

**Character:** One sans family carries the whole interface. That keeps the product quiet, familiar, and efficient. Weight and scale do the hierarchy work, not decorative font switching.

### Hierarchy
- **Display** (600, 32px, 1.15): Reserved for the main Simple Mode statement and rare top-level page titles.
- **Headline** (600, 20px, 1.25): Section headers, dashboard titles, and developer page headings.
- **Title** (600, 16px, 1.4): Card titles, control group labels, and short prompts.
- **Body** (400, 14px, 1.55): Explanatory copy and status text. Keep prose at 65 to 75ch where possible.
- **Label** (600, 12px, 0.12em, uppercase): Field labels, table heads, and metadata tags.

### Named Rules
**The Single Sans Rule.** Never introduce a second family just to make the file look more designed. This product needs clarity, not typographic commentary.

**The Fixed Scale Rule.** Keep the scale step changes modest and predictable. Product UI should not lurch between sizes.

## Elevation

This system is almost flat. Depth comes from tonal separation, borders, and one soft panel shadow, not from layered drama. The effect should feel like stacked paper on a desk, not floating glass or a glossy dashboard.

### Shadow Vocabulary
- **Panel Lift** (`0 16px 40px rgba(16, 24, 40, 0.08)`): Primary panels and the main status card.
- **Hover Lift** (`0 8px 20px rgba(16, 24, 40, 0.06)`): Optional hover treatment for clickable surfaces.

### Named Rules
**The Flat-By-Default Rule.** If a surface is not interactive or primary, it should stay flat.

**The Shadow-Is-Structure Rule.** Shadow is for hierarchy, not decoration. If the page still reads without it, the shadow is doing its job.

## Components

The component language is standard and familiar. Controls should be immediately legible, with no novelty in shape or behavior.

### Buttons
- **Shape:** Gently rounded rectangles (`12px` radius).
- **Primary:** Civic Blue fill with white text, medium padding, used only for the main action in a group.
- **Hover / Focus:** Darken slightly on hover, show a visible focus ring, and keep transitions short and functional.
- **Secondary / Ghost / Tertiary:** White or transparent surfaces with a line border and muted text, used for utility actions.

### Panels / Cards
- **Corner Style:** Softly rounded, not pill-shaped (`20px` radius).
- **Background:** Document Surface over the Paper Canvas.
- **Shadow Strategy:** One soft lift shadow for primary panels only.
- **Border:** One-pixel Divider Gray outline for structure.
- **Internal Padding:** Generous but not spacious, usually `24px`.

### Status Badge
- **Style:** Small pill badge with semantic tone color, compact padding, and strong label weight.
- **State:** Neutral for idle, amber for starting or partial readiness, green for ready, red for needs attention.

### Inputs / Fields
- **Style:** White fill, Divider Gray stroke, soft rounding, no decorative chrome.
- **Focus:** Border shifts to Civic Blue with a subtle ring, never a glow effect.
- **Error / Disabled:** Red border and muted text only, no animation flourish.

### Navigation
- **Style:** Rounded pills or simple list items with clear active state.
- **Typography:** Small but readable, same sans family as the rest of the app.
- **Default / Hover / Active:** Inactive items stay neutral; active items use Civic Blue or dark text inversion.
- **Mobile Treatment:** Collapse density before inventing a new pattern.

### Simple Status Card
- **Style:** Centered, restrained, and almost ceremonial in its simplicity.
- **Header:** The app mark is compact and functional, not logo-like theater.
- **Content:** One state line, one supporting sentence, and one action cluster.

## Do's and Don'ts

### Do:
- **Do** keep Simple Mode visually spare, centered, and easy to scan.
- **Do** use Civic Blue only for primary actions, selected state, and active navigation.
- **Do** let borders and spacing carry structure before adding more shadow.
- **Do** keep messages matter-of-fact, especially when the app needs attention.
- **Do** make Developer Mode denser, but keep it recognizably the same system.

### Don't:
- **Don't** make it look like a startup dashboard.
- **Don't** make it look like an AI demo app.
- **Don't** make it look like a crypto terminal.
- **Don't** make it look like a gamer UI.
- **Don't** make it look like a glossy consumer product.
- **Don't** use flashy gradients.
- **Don't** use neon accents.
- **Don't** use playful mascots.
- **Don't** use dense analytics panels as decoration.
- **Don't** build over-designed empty states.
- **Don't** write overly clever copy.
- **Don't** make simple service control feel experimental.
