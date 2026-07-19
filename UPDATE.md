# FerroScope — Update Guide

This guide covers how to keep a FerroScope deployment up to date. There are three moving parts:

- the **Server** (runs in Docker),
- the **Web UI** (runs in Docker), and
- the **Agent** (installed on each monitored machine).

The two Docker components are updated by pulling the newest images and recreating their containers. The agent is updated with a single command.

**Contents**

1. [Overview](#overview)
2. [Updating the Server and Web UI (Docker)](#updating-the-server-and-web-ui-docker)
   - [Option A — single Compose file](#option-a--single-compose-file)
   - [Option B — separate Compose files](#option-b--separate-compose-files)
   - [Clean up old images](#clean-up-old-images-optional)
   - [Pinning a specific version](#pinning-a-specific-version-optional)
3. [Updating the Agent](#updating-the-agent)
4. [After updating](#after-updating)

---

## Overview

| Component | Runs as | Update method |
| --- | --- | --- |
| Server | Docker container (`subhodip1307/ferroscope-server`) | Pull new image + recreate |
| Web UI | Docker container (`subhodip1307/ferroscope-ui`) | Pull new image + recreate |
| Agent | systemd service on each host | One-line update script |

Your data is safe across updates. PostgreSQL keeps its data in the `/srv/ferroscope_server` volume, and the agent's configuration in `/etc/ferroscope_agent/agent.toml` is left untouched. Updating only swaps out the application containers and binary — not your data.

---

## Updating the Server and Web UI (Docker)

The Compose files reference the images **without a version tag**:

```yaml
image: subhodip1307/ferroscope-server
image: subhodip1307/ferroscope-ui
```

An untagged image means Docker uses the `latest` tag — but Docker does **not** re-download `latest` on its own. To pick up a new release you have to pull the newest images and recreate the containers.

### Option A — single Compose file

If you deployed everything from one `docker-compose.yml`:

```bash
docker compose pull      # download the newest server + UI images
docker compose up -d     # recreate any container whose image changed
```

`docker compose up -d` only recreates the containers that actually changed, so your database keeps running untouched.

### Option B — separate Compose files

If your server and UI live in separate files:

```bash
# Server (and database)
docker compose -f server-compose.yml pull
docker compose -f server-compose.yml up -d

# Web UI
docker compose -f ui-compose.yml pull
docker compose -f ui-compose.yml up -d
```

### Clean up old images (optional)

After updating, the previous images stay on disk. Remove the ones no longer in use:

```bash
docker image prune -f
```

### Pinning a specific version (optional)

If you'd rather run a fixed release instead of tracking `latest`, set the tag explicitly in the Compose file:

```yaml
image: subhodip1307/ferroscope-server:1.2.0   # example tag
image: subhodip1307/ferroscope-ui:1.2.0       # example tag
```

Then apply it:

```bash
docker compose pull
docker compose up -d
```

To move to another version later, change the tag and run the two commands again.

> **Tip:** pinning a version gives you repeatable deployments and makes rollbacks easy — set the tag back to the previous version and re-run `pull` + `up -d`.

---

## Updating the Agent

On each monitored machine, run the one-line updater:

```bash
curl -fsSL https://raw.githubusercontent.com/Subhodip1307/ferroscope/main/scripts/update.sh | sudo bash
```

This downloads the latest agent binary, stops the running service, swaps in the new build, and starts it again. Your configuration at `/etc/ferroscope_agent/agent.toml` and everything under `CONF/` is left untouched.

Repeat this on every host that runs an agent.

---

## After updating

Confirm everything came back up cleanly.

**Docker (server + UI):**

```bash
docker compose ps          # all services should be "running" / healthy
docker compose logs -f     # watch for startup errors
```

**Agent (on each host):**

```bash
systemctl status ferr
journalctl -u ferr -f      # follow the logs
```

Then open the web UI and confirm your nodes are still reporting. If a node looks stale, give it a metric interval or two and refresh the page.