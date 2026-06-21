# Ferroscope Agent

**Contents**

1. [Overview](#overview)
2. [Architecture Principles](#architecture-principles)
3. [Core Capabilities](#core-capabilities)
   - [System Telemetry](#system-telemetry)
4. [Installation & Deployment](#installation--deployment)
   - [Step 1 — Run the installer](#step-1--run-the-installer)
   - [Step 2 — Create a node in the web UI](#step-2--create-a-node-in-the-web-ui)
   - [Step 3 — Configure the agent](#step-3--configure-the-agent)
   - [Step 4 — Start the agent](#step-4--start-the-agent)
   - [Tuning the agent](#tuning-the-agent-optional)
5. [Updating the Agent](#updating-the-agent)
6. [Task Execution System](#task-execution-system)
7. [Configuration](#configuration)
   - [Directory Structure](#directory-structure)
   - [Web Checking Configuration](#web-checking-configuration)
   - [Host Checking Configuration](#host-checking-configuration)
   - [Important Notes](#important-notes)
8. [Security Considerations](#security-considerations)
9. [Roadmap](#roadmap)

---

## Overview

Ferroscope Agent is a  Linux-based system monitoring agent written entirely in Rust. It is designed for performance, reliability, and minimal resource overhead.

The agent runs directly on target infrastructure and reports system-level telemetry and task execution results to the Ferroscope backend.

> **Platform Support:** Linux (x86_64). Additional platforms are planned.

---

## Architecture Principles

Ferroscope Agent is built with the following goals:

* **Performance-first design** — Low CPU and memory footprint.
* **Memory safety** — Leveraging Rust’s ownership model to eliminate runtime memory vulnerabilities.
* **Concurrency without compromise** — Efficient async execution model.
* **Extensibility** — Task system designed to support future monitoring capabilities.
* **Deterministic behavior** — Predictable execution intervals and structured reporting.

---

## Core Capabilities

The agent collects and reports the following system metrics:

### System Telemetry

* CPU usage
* RAM usage
* Kernel version
* Operating system name
* System uptime
* CPU thread count
* CPU vendor information

These metrics provide a lightweight but actionable overview of system health and runtime environment.

Additional telemetry modules will be introduced in upcoming releases.

---

## Installation & Deployment

The agent runs on every machine you want to monitor. Installation is handled by a single script — no manual user or service setup required.

### Step 1 — Run the installer

On the target Linux machine, run:

```bash
curl -fsSL https://raw.githubusercontent.com/Subhodip1307/ferroscope/main/scripts/install.sh | sudo bash
```

The script handles the full setup. It will:

* Install and set up the agent on the system
* Create a dedicated least-privilege user to run it
* Generate the configuration file
* **Enable** the systemd service — but **not start it yet** (you'll start it once it's configured)

### Step 2 — Create a node in the web UI

Log in to the Ferroscope web UI and click the **Create Node** button. Enter a name for the node and submit. The UI will return two values:

* an **auth token**
* a **server (backend) URL**

Copy both — you'll need them in the next step.

### Step 3 — Configure the agent

On the machine where you installed the agent, open the config file at `/etc/ferroscope_agent/agent.toml`:

```bash
sudo nano /etc/ferroscope_agent/agent.toml     # or: sudo vim ...
```

Fill in the two values you copied from the web UI:

```toml
agent_auth_token = "<token from the web UI>"
agent_server_url = "<server url from the web UI>"
```

Save and exit.

### Step 4 — Start the agent

The installer only *enabled* the service, so you need to start it after configuring:

```bash
sudo systemctl start ferr
```

Check that it's running:

```bash
systemctl status ferr
journalctl -u ferr -f     # follow the logs
```

Now check the web UI — the node should start reporting its stats. If nothing shows up right away, click the refresh button in the UI (or reload the page). Because the service was enabled during install, it will also start automatically on every boot.

### Tuning the agent (optional)

`agent.toml` also contains a few timing options you can adjust. All values are in **seconds**.

| Option | Default | What it does |
| ----------------- | ----- | --------------------------------------------------------------------------- |
| `api_time_out`    | `5`   | How long the agent waits for the server to respond before the request times out. |
| `cpu_interval`    | `60`  | How often CPU usage is collected and sent — every 60s by default.           |
| `ram_interval`    | `60`  | How often RAM usage is collected and sent — every 60s by default.           |
| `uptime_interval` | `600` | How often system uptime is reported — every 10 minutes by default.          |
| `web_interval`    | `60`  | How often the agent runs the web service checks you've configured in `CONF/Web/` — every 60s by default. |
| `host_interval`   | `50`  | How often the agent runs the host checks you've configured in `CONF/Host/` — every 50s by default. |

---

## Updating the Agent

To update an installed agent to the latest release, run the one-line updater on the target machine:

```bash
curl -fsSL https://raw.githubusercontent.com/Subhodip1307/ferroscope/main/scripts/update.sh | sudo bash
```

This downloads the latest binary, stops the agent, swaps in the new build, and starts it again. Your existing configuration in `/etc/ferroscope_agent/agent.toml` is left untouched.

---

## Task Execution System

Beyond telemetry, the Ferroscope Agent supports a configurable task system.

Tasks are defined via a configuration file and executed locally by the agent.

### Currently Supported Task Type

#### 1. Service Tracking (Host Monitoring)

The agent can monitor a host or service at a fixed interval.

When enabled, the agent will:

1. Read tracking rules from the configuration file.
2. Perform health checks at a defined interval.
3. Determine whether the service is alive or unreachable.
4. Report status results to the backend.

This capability is especially valuable for:

* Monitoring services running inside VPNs or private networks.
* Observing internal infrastructure without exposing it publicly.
* Detecting downtime in restricted environments.

More task types (process monitoring, port checks, advanced health validation, etc.) are planned.

---

# Configuration

Ferroscope Agent uses TOML-based configuration files to define tracking tasks.

Configuration files must follow a strict directory structure relative to the agent binary.

### Directory Structure

The configuration directory must be placed in the **same directory where the agent binary resides**.

Required structure:

```
<binary_directory>/
│
├── ferroscope-agent
├── CONF/
│   ├── Web/
│   └── Host/
```

* `CONF/` — Root configuration directory (must be created manually).
* `Web/` — Contains web tracking configuration files.
* `Host/` — Contains host tracking configuration files.

The agent reads configurations dynamically from these directories.

---

## Web Checking Configuration

Web tracking allows monitoring HTTP/HTTPS endpoints.

Each web tracking configuration must:

* Be a `.toml` file
* Be placed inside `CONF/Web/`

### Required Fields

| Field         | Type    | Description                                                                    |
| ------------- | ------- | ------------------------------------------------------------------------------ |
| `name`        | String  | Logical identifier for the check                                               |
| `url`         | String  | Full URL including protocol (e.g., [https://example.com](https://example.com)) |
| `status_code` | Integer | Expected HTTP status code                                                      |

### Example

```toml
name = "example-home"
url = "https://example.com"
status_code = 200
```

### Behavior

The agent will:

* Send an HTTP request to the specified URL
* Compare the returned status code with the expected value
* Mark the check as failed if the status does not match
* Report the result to the backend

---

## Host Checking Configuration

Host tracking allows monitoring TCP connectivity to a specific host and port.

Each host tracking configuration must:

* Be a `.toml` file
* Be placed inside `CONF/Host/`

### Required Fields

| Field  | Type    | Description                                           |
| ------ | ------- | ----------------------------------------------------- |
| `name` | String  | Logical identifier for the check                      |
| `host` | String  | Target IPv4 address (IPv6 not supported at this time) |
| `port` | Integer | Port number to test connectivity                      |

### Example

```toml
name = "internal-api"
host = "192.168.1.10"
port = 8080
```

### Behavior

The agent will:

* Attempt to establish a TCP connection to the specified host and port
* Mark the check as successful if connection succeeds
* Mark the check as failed if connection cannot be established
* Report the result to the backend

---
## Reloading Configuration
 
The agent supports **runtime configuration reloading** — configuration files can be added, modified, or removed without restarting the agent or interrupting checks that are already in progress.
 
**Configuration changes do not take effect automatically.** After adding, modifying, or removing any file under `CONF/`, a reload must be triggered for the changes to apply.
 
After writing a configuration, run:
 
```bash
sudo systemctl reload ferr.service
```
 
The agent then re-scans the `CONF/Web/` and `CONF/Host/` directories and updates its active configuration set.
 
### Reload Behavior
 
* The reload replaces the in-memory configuration set atomically — running checks are not interrupted.
* New configuration files are picked up, and removed files are dropped from the active set.
* Multiple reloads triggered in quick succession may be merged into a single reload. This is expected behavior; the final on-disk state is what takes effect.
* A restart of the agent is **not** required for configuration changes.
---

### Important Notes

* Configuration files must be valid TOML.
* Files placed outside the required directory structure will be ignored.
* IPv6 is not supported for host checks at this stage.
* Misconfigured files may cause the check to fail validation.

This configuration model is intentionally simple and deterministic. More advanced options (timeouts, retries, intervals, alert rules) will be introduced in future releases.

---

## Security Considerations

* No unnecessary ports are exposed.
* Communication with backend services should be secured using TLS.
* Sensitive configuration values should be protected at the filesystem level.

---
