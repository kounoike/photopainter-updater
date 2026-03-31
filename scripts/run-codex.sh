#!/bin/sh

cd $(dirname "$0")/..

codex \
  --dangerously-bypass-approvals-and-sandbox
