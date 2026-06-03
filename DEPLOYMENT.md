# FerroScope — Deployment Guide

This guide walks through deploying the FerroScope **server** and **web UI** with Docker, and putting **nginx** in front of them as a reverse proxy.

**Contents**

1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Deploying with Docker](#deploying-with-docker)
   - [Option A — Everything in one Compose file](#option-a--everything-in-one-compose-file)
   - [Option B — Server and UI in separate files](#option-b--server-and-ui-in-separate-files)
   - [Environment variables](#environment-variables)
4. [Configuring nginx](#configuring-nginx)
5. [Enabling HTTPS](#enabling-https)
6. [Creating and managing users](#creating-and-managing-users)

---

## Overview

FerroScope is published as two Docker images:

| Component | Image | Internal port |
| --- | --- | --- |
| Server (backend) | `subhodip1307/ferroscope-server` | `8000` |
| Web UI (frontend) | `subhodip1307/ferroscope-ui` | `3000` |

The server also needs a **PostgreSQL** database (port `5432`).

A typical production layout puts nginx on the host, terminates TLS, and reverse-proxies to the containers, which only listen on localhost:

```
                  ┌─────────────────────────── Host ───────────────────────────┐
   Browser ──▶ nginx (443) ─┬─▶ 127.0.0.1:3000   FerroScope UI  (Next.js)
                            └─▶ 127.0.0.1:8000   FerroScope Server (Axum) ─▶ PostgreSQL
```

The browser talks to the server **directly** over the public API URL (set via `NEXT_PUBLIC_BASE_URL`), so the two are exposed on separate hostnames (e.g. `ferroscope.example.com` for the UI and `api.ferroscope.example.com` for the server).

---

## Prerequisites

- A Linux host with **Docker** and the **Docker Compose plugin** installed
- A domain (or two subdomains) pointing to the host
- **nginx** installed on the host
- The data directory for PostgreSQL created in advance:

```bash
sudo mkdir -p /srv/ferroscope_server
```

---

## Deploying with Docker

You can run the UI and the backend together in a single Compose file, or split them into separate files (useful when the UI runs on a different machine from the server).

### Option A — Everything in one Compose file

Create `docker-compose.yml`:

```yaml
services:
  database:
    image: postgres:18
    shm_size: 512mb
    restart: unless-stopped
    environment:
      POSTGRES_USER: myuser
      POSTGRES_PASSWORD: mypassword
      POSTGRES_DB: mydatabase
      PGDATA: /var/lib/postgresql/data
    volumes:
      - /srv/ferroscope_server:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U myuser -d mydatabase"]
      interval: 5s
      timeout: 5s
      retries: 10

  project:
    image: subhodip1307/ferroscope-server
    restart: unless-stopped
    depends_on:
      database:
        condition: service_healthy
    environment:
      CORS: "https://ferroscope.example.com"
      PSQL_URL: "postgres://myuser:mypassword@database:5432/mydatabase"
      EMAIL_HOST_USER: ""
      EMAIL_HOST_PASSWORD: ""
      EMAIL_HOST: ""
      # Optional. Defaults to 0.0.0.0:8000 if unset.
      # HOST: "0.0.0.0:8000"
    user: "scope:scope"
    ports:
      - "127.0.0.1:8000:8000"

  ui:
    image: subhodip1307/ferroscope-ui
    restart: unless-stopped
    environment:
      NEXT_PUBLIC_BASE_URL: "https://api.ferroscope.example.com"
    ports:
      - "127.0.0.1:3000:3000"
```

Start the stack:

```bash
docker compose up -d
docker compose logs -f          # watch startup
docker compose ps               # check status
```

> **Why `127.0.0.1:` in the port mappings?**
> It binds the containers to localhost only, so they aren't reachable from the public internet directly — all external traffic must go through nginx. The database has no published port at all; only the server reaches it, internally, over the Compose network (which is why `PSQL_URL` uses the hostname `database`, the service name).

### Option B — Server and UI in separate files

Run the database + server on one host (or stack) and the UI on another. The two communicate only through the public API URL in the browser, so they don't need to share a Docker network.

**`server-compose.yml`** (database + server):

```yaml
services:
  database:
    image: postgres:18
    shm_size: 512mb
    restart: unless-stopped
    environment:
      POSTGRES_USER: myuser
      POSTGRES_PASSWORD: mypassword
      POSTGRES_DB: mydatabase
      PGDATA: /var/lib/postgresql/data
    volumes:
      - /srv/ferroscope_server:/var/lib/postgresql/data

  project:
    image: subhodip1307/ferroscope-server
    restart: unless-stopped
    depends_on:
      database:
        condition: service_healthy
    environment:
      CORS: "https://ferroscope.example.com"
      PSQL_URL: "postgres://myuser:mypassword@database:5432/mydatabase"
      EMAIL_HOST_USER: ""
      EMAIL_HOST_PASSWORD: ""
      EMAIL_HOST: ""
    user: "scope:scope"
    ports:
      - "127.0.0.1:8000:8000"
```

**`ui-compose.yml`** (UI only):

```yaml
services:
  ui:
    image: subhodip1307/ferroscope-ui
    restart: unless-stopped
    environment:
      NEXT_PUBLIC_BASE_URL: "https://api.ferroscope.example.com"
    ports:
      - "127.0.0.1:3000:3000"
```

Bring each up independently:

```bash
docker compose -f server-compose.yml up -d
docker compose -f ui-compose.yml up -d
```

### Environment variables

**Database (`database`)**

| Variable | Description |
| --- | --- |
| `POSTGRES_USER` | Database username. **Change from the default.** |
| `POSTGRES_PASSWORD` | Database password. **Change from the default.** |
| `POSTGRES_DB` | Database name. |
| `PGDATA` | Data directory inside the container (kept as is). |

**Server (`project`)**

| Variable | Description |
| --- | --- |
| `CORS` | Comma-separated list of allowed frontend origins, e.g. `https://ferroscope.example.com` or `https://a.example.com,https://b.example.com`. This **must** include the UI's public URL, or the browser will block requests. |
| `PSQL_URL` | PostgreSQL connection string. The host part (`database`) must match the database service name when they share a Compose network. |
| `EMAIL_HOST` | SMTP server host used to send notification emails (node/service down, SSL/TLS expiry, etc.). |
| `EMAIL_HOST_USER` | SMTP username. |
| `EMAIL_HOST_PASSWORD` | SMTP password. |
| `HOST` | *(Optional)* Bind address for the server. Defaults to `0.0.0.0:8000`. If you change the port here, update the `ports` mapping to match. |

**UI (`ui`)**

| Variable | Description |
| --- | --- |
| `NEXT_PUBLIC_BASE_URL` | The **public** URL where the browser reaches the FerroScope server (e.g. `https://api.ferroscope.example.com`). This is used client-side, so it must be the externally reachable address — not the internal Docker hostname. |

> **The two settings that trip people up:** `NEXT_PUBLIC_BASE_URL` must point at the server's public URL, and the server's `CORS` must list the UI's public URL. They are a matched pair.

---

## Configuring nginx

This setup uses two server names: one for the UI, one for the API. Create `/etc/nginx/sites-available/ferroscope`:

```nginx
# ---- FerroScope Web UI ----
server {
    listen 80;
    server_name ferroscope.example.com;

    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_http_version 1.1;
        proxy_set_header Host              $host;
        proxy_set_header X-Real-IP         $remote_addr;
        proxy_set_header X-Forwarded-For   $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}

# ---- FerroScope Server (API + live streaming) ----
server {
    listen 80;
    server_name api.ferroscope.example.com;

    location / {
        proxy_pass http://127.0.0.1:8000;
        proxy_http_version 1.1;
        proxy_set_header Host              $host;
        proxy_set_header X-Real-IP         $remote_addr;
        proxy_set_header X-Forwarded-For   $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        # Required for Server-Sent Events (live metric streaming).
        # Without these, nginx buffers the stream and live metrics stall.
        proxy_set_header Connection        '';
        proxy_buffering            off;
        proxy_cache                off;
        proxy_read_timeout         24h;
        chunked_transfer_encoding  off;
    }
}
```

Enable the site and reload:

```bash
sudo ln -s /etc/nginx/sites-available/ferroscope /etc/nginx/sites-enabled/
sudo nginx -t          # test the config
sudo systemctl reload nginx
```

> The SSE block on the API server is important. FerroScope streams live metrics over Server-Sent Events; if nginx buffers the response (the default), the dashboard won't update in real time and long-lived connections may be cut. `proxy_buffering off` plus a long `proxy_read_timeout` keeps the stream open and flowing.

**Single-domain alternative:** if you'd rather serve everything from one hostname, you can route the UI at `/` and the API under a path prefix in the same `server` block — but this only works if the server's routes live under that prefix. The two-subdomain layout above works with the app as-is and is the recommended approach.

---

## Enabling HTTPS

Because FerroScope can monitor SSL/TLS certificate expiry, you'll almost certainly want it served over HTTPS itself. The quickest route is Certbot with the nginx plugin:

```bash
sudo apt install certbot python3-certbot-nginx
sudo certbot --nginx -d ferroscope.example.com -d api.ferroscope.example.com
```

Certbot rewrites the server blocks to listen on `443` and sets up automatic renewal. After enabling HTTPS, make sure your URLs use `https://`:

- `NEXT_PUBLIC_BASE_URL` → `https://api.ferroscope.example.com`
- `CORS` → `https://ferroscope.example.com`

Re-create the affected containers so the new values take effect:

```bash
docker compose up -d
```

---

## Creating and Managing Users

User accounts are managed with the **`fero`** command-line tool, which ships inside the server image. Use it to create the account you'll log in to the web UI with, and to reset passwords later.

First, open a shell inside the running server container (the `project` service):

```bash
docker compose exec project sh
```

> On older Docker installs still using Compose v1, the command is `docker-compose exec project sh`.

Once inside the container, use `fero`:

**Create a user**

```sh
fero createuser <username> <password>
```

**Change a password**

```sh
fero changepassword <username> <password>
```

| Argument | Description |
| --- | --- |
| `<username>` | The account's login name. |
| `<password>` | The account's password. |

For example:

```sh
fero createuser admin "s3cur3-p@ss"
fero changepassword admin "n3w-p@ss"
```

When you're done, leave the container shell:

```sh
exit
```

> **Tip — skip the interactive shell.** You can call `fero` directly from the host in one line:
> ```bash
> docker compose exec project fero createuser <username> <password>
> ```

> **Security note:** passwords passed as command-line arguments may be saved in your shell history and briefly visible in the container's process list. On a shared host, clear the relevant history afterward and use a strong, unique password.