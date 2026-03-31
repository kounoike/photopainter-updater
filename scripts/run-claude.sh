#!/bin/sh

cd $(dirname "$0")/..

claude \
  --allow-dangerously-skip-permissions
