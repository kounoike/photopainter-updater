# AGENTS.md — Kanban + Spec Kit Execution Rules

## Purpose

This repository uses Kanban as an orchestration layer and Spec Kit as the execution workflow.

All agents must treat Kanban cards as executable units and follow Spec Kit workflow strictly.

---

## Core Principles

* A Kanban card is NOT a note. It is an executable specification.
* Every implementation MUST follow Spec Kit workflow.
* Never skip steps. Never guess across boundaries.
* Prefer stopping with a BLOCKER over making incorrect assumptions.

---

## Card Requirements

When creating or modifying a Kanban card, ALWAYS include:

* Goal
* Done Criteria
* Shared Specs
* Allowed Scope
* Forbidden Scope
* Dependencies
* Execution Rules

If information is missing:

* Write `TODO:` explicitly
* If unsafe → write `BLOCKER:` and stop

Never create incomplete or minimal cards.

---

## Required Execution Workflow

For every implementation task, follow this order:

1. specify
2. clarify (only for local ambiguities)
3. plan
4. tasks
5. analyze
6. implement

Rules:

* Do NOT skip phases
* Do NOT jump directly to implementation
* Each phase must produce artifacts (spec/plan/tasks/etc.)

---

## Scope Control

* Only modify files inside **Allowed Scope**
* Never modify **Forbidden Scope**
* If required change is outside scope:
  → STOP and emit BLOCKER

---

## Shared Spec Compliance

Always read and follow:

* specs/_shared/product.md
* specs/_shared/architecture.md
* specs/_shared/conventions.md
* specs/_shared/test-strategy.md

Never redefine global architecture.

---

## Uncertainty Handling

If unclear:

1. Check Shared Specs
2. Check existing code patterns
3. If still unclear → BLOCKER

Never guess cross-feature behavior.

---

## Completion Requirements

At the end of execution, always report:

* spec changes
* plan/tasks generated
* files changed
* tests executed
* remaining risks or limitations

---

## Strict Prohibitions

Agents MUST NOT:

* Skip Spec Kit workflow
* Create plain Kanban cards
* Modify unrelated areas
* Introduce global design changes
* Ignore missing information

---

## Final Rule

If a task cannot be executed safely under these rules:

→ STOP
→ REPORT BLOCKER
→ DO NOT PROCEED
