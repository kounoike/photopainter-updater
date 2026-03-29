# AGENTS.md

## Purpose

This repository uses Kanban as an orchestration layer and Spec Kit as the execution workflow.

All agents must treat Kanban cards as executable units and follow Spec Kit workflow strictly.

---

## Core Principles

* When working on tasks described in a Kanban card, if the work is a feature addition, the Spec Kit workflow must be followed strictly.

---

## Required Execution Spec Kit Workflow

For every implementation task, follow this order:

1. specify
2. clarify (only for local ambiguities)
3. plan
4. tasks
5. analyze
6. implement
7. commit
8. merge

Rules:

* Do NOT skip phases
* Do NOT jump directly to implementation
* Each phase must produce artifacts (spec/plan/tasks/etc.)
* commit & merge only after successful implementation and testing

---

# Codex notes

Use skill commands:

$speckit-specify
$speckit-clarify
$speckit-plan
$speckit-tasks
$speckit-analyze
$speckit-implement


---

# Claude code notes

Use slash commands:

* /speckit.specify
* /speckit.clarify
* /speckit.plan
* /speckit.tasks
* /speckit.analyze
* /speckit.implement

---

## Active Technologies
- Bash (POSIX shell) / Python 3 (実行環境) + Python 標準の HTTP 配信機能 (002-add-run-script)
- files (`server/contents/`) (002-add-run-script)

## Recent Changes
- 002-add-run-script: Added Bash (POSIX shell) / Python 3 (実行環境) + Python 標準の HTTP 配信機能
