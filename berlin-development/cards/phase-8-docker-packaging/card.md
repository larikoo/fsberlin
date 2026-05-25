---
id: 01JBPHASE0800000000000000
title: "Docker container, compose, volume conventions"
type: phase
phase_number: 8
criteria: []
priority: high
assignee: claude-code
skills: [rust, python, architecture]
depends_on: []
created: 2026-05-25
---

Phase 8 of FSBerlin development.


Package FSBerlin as a single container.

## Deliverables
- Dockerfile that builds the Rust binary, includes the Python
  agent runtime venv, and gitleaks.
- docker-compose.yml for local dev with volume mount.
- Healthcheck endpoint.
- Image published to ghcr.io.
- Signed images (cosign).

## Success criteria
- `docker run -v ./project:/data ghcr.io/lari/fsberlin:latest`
  starts the substrate and serves MCP on stdio.
- Image size under 400 MB.
- Cold start under 2 seconds.

## Depends on
- Phases 1-5 minimum.
