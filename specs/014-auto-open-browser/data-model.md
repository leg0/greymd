# Data Model: Auto-Open Browser on Startup

**Feature**: 014-auto-open-browser  
**Date**: 2026-03-09

## Overview

This feature introduces no new data entities, persistent state, or data relationships. It is a behavioral change to the startup flow.

## State

The feature adds one piece of ephemeral startup state:

- **`no_browser` flag** (`bool`): Parsed from CLI arguments at startup. `true` if `--no-browser` was passed, `false` otherwise. Not persisted. Consumed once during startup to decide whether to invoke the browser-opening function.

## Interactions

```
CLI args → parse --no-browser → bool
                                  │
TcpListener::bind() → addr ──────┤
                                  ▼
                          if !no_browser:
                            browser::open(url)
                                  │
                                  ▼
                          server::start(listener)
```

No database, files, or external state are read or written by this feature.
