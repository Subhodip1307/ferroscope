# Ferroscope — Read / View API

These endpoints let you **read data** from nodes — system info, CPU/RAM metrics, and service statuses.

Intended for developers building dashboards, monitoring apps, or automation tools on top of Ferroscope.

---

## Index

- [Authentication](#authentication)
- [Base Path](#base-path)
- [Get Node List](#get-node-list)
- [Get Node System Info](#get-node-system-info)
- [Get Latest CPU Usage](#get-latest-cpu-usage)
- [Get Latest RAM Usage](#get-latest-ram-usage)
- [CPU Usage History](#cpu-usage-history)
- [RAM Usage History](#ram-usage-history)
- [List Services on a Node](#list-services-on-a-node)
- [Get Status of a Single Service](#get-status-of-a-single-service)
- [Get Status of All Services](#get-status-of-all-services)

---

## Authentication

**Every endpoint in this section requires authentication**, regardless of whether it reads sensitive data or not.

Include your token in every request header:

```
Authorization: <your-token>
```

If the token is missing or invalid:

```
401 Unauthorized
```

See the [Authentication Guide](auth.md) for how to obtain a token.

---

## Base Path

```
/view
```

All endpoints below are relative to this base path.

---

## Get Node List

Returns all nodes registered in the Ferroscope server.

### Endpoint

```
POST /view/get_node_list
```

### Request

No body or query parameters required.

### Response

**Status:** `200 OK`

```json
[
  { "id": 1, "name": "production-server" },
  { "id": 2, "name": "backup-node" }
]
```

| Field | Type | Description |
|-------|------|-------------|
| `id` | integer | The node's unique numeric ID |
| `name` | string | The node's human-readable name |

---

## Get Node System Info

Returns hardware and OS details for a specific node.

### Endpoint

```
POST /view/get_node_info
```

### Request

Query parameter:

```
node=<node_id>
```

Example:

```
/view/get_node_info?node=1
```

### Response

**Status:** `200 OK`

```json
{
  "system_name": "Linux",
  "kernel_version": "6.8.0",
  "os_version": "Ubuntu 22.04",
  "uptime": 938473,
  "cpu_threads": 16,
  "cpu_vendor": "Intel"
}
```

| Field | Type | Description |
|-------|------|-------------|
| `system_name` | string | OS family (e.g. `"Linux"`, `"Windows"`) |
| `kernel_version` | string | Kernel or OS build version |
| `os_version` | string | Full OS version string |
| `uptime` | integer | System uptime in seconds |
| `cpu_threads` | integer | Number of logical CPU threads |
| `cpu_vendor` | string | CPU manufacturer (e.g. `"Intel"`, `"AMD"`) |

### Error Response

**Status:** `204 No Content` — The node exists but has not reported system info yet.

---

## Get Latest CPU Usage

Returns the single most recent CPU usage reading for a node.

### Endpoint

```
POST /view/get_latest_cpu
```

### Request

Query parameter:

```
node=<node_id>
```

Example:

```
/view/get_latest_cpu?node=1
```

### Response

**Status:** `200 OK`

```json
{
  "value": 43.7,
  "date_time": "2026-03-15T10:45:33Z"
}
```

| Field | Type | Description |
|-------|------|-------------|
| `value` | float | CPU usage percentage at the time of recording |
| `date_time` | string (UTC) | Timestamp of the reading |

### Error Response

**Status:** `204 No Content` — No CPU data available for this node yet.

---

## Get Latest RAM Usage

Returns the single most recent RAM usage reading for a node.

### Endpoint

```
POST /view/get_latest_ram
```

### Request

Query parameter:

```
node=<node_id>
```

Example:

```
/view/get_latest_ram?node=1
```

### Response

**Status:** `200 OK`

```json
{
  "total": "16000000",
  "free": "9200000",
  "timestamp": "2026-03-15T10:45:33Z"
}
```

| Field | Type | Description |
|-------|------|-------------|
| `total` | string | Total RAM on the node in kilobytes |
| `free` | string | Free/available RAM in kilobytes |
| `timestamp` | string (UTC) | Timestamp of the reading |

### Error Response

**Status:** `204 No Content` — No RAM data available for this node yet.

---

## CPU Usage History

Returns the **last 20 CPU usage records** for a node, ordered from newest to oldest.

### Endpoint

```
POST /view/cpu_stat
```

### Request

Query parameter:

```
node=<node_id>
```

### Response

**Status:** `200 OK`

```json
[
  { "value": 41.2, "date_time": "2026-03-15T10:45:33Z" },
  { "value": 38.7, "date_time": "2026-03-15T10:44:33Z" }
]
```

Returns an empty array `[]` if no records exist yet.

---

## RAM Usage History

Returns the **last 20 RAM usage records** for a node, ordered from newest to oldest.

### Endpoint

```
POST /view/ram_stat
```

### Request

Query parameter:

```
node=<node_id>
```

### Response

**Status:** `200 OK`

```json
[
  { "total": "16000000", "free": "9200000", "timestamp": "2026-03-15T10:45:33Z" },
  { "total": "16000000", "free": "9150000", "timestamp": "2026-03-15T10:44:33Z" }
]
```

Returns an empty array `[]` if no records exist yet.

---

## List Services on a Node

Returns all monitored services on a node, grouped by their category.

### Endpoint

```
POST /view/node_services
```

### Request

Query parameter:

```
node=<node_id>
```

### Response

**Status:** `200 OK`

Services are returned as a map where each **key is a category name** and the value is a list of services in that category.

```json
{
  "Web": [
    {
      "service_name": "nginx",
      "category": "Web",
      "ssl_exp": "2026-12-01T00:00:00Z"
    }
  ],
  "Host": [
    {
      "service_name": "postgres",
      "category": "Host",
      "ssl_exp": null
    }
  ]
}
```

| Field | Type | Description |
|-------|------|-------------|
| `service_name` | string | Name of the monitored service |
| `category` | string | Category the service belongs to (`"Web"`, `"Host"`, etc.) |
| `ssl_exp` | string (UTC) or `null` | SSL certificate expiry time. `null` if not applicable |

---

## Get Status of a Single Service

Returns the current status of one specific service on a node.

### Endpoint

```
POST /view/single_service_current_stat
```

### Request

**JSON body:**

```json
{
  "node": 1,
  "service_name": "nginx"
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `node` | integer | ✅ Yes | The node's numeric ID |
| `service_name` | string | ✅ Yes | The exact name of the service |

### Response

**Status:** `200 OK`

```json
{
  "status": "up",
  "error_msg": "",
  "category": "Web",
  "ssl_exp": "2026-12-01T00:00:00Z"
}
```

| Field | Type | Description |
|-------|------|-------------|
| `status` | string | Current status of the service (e.g. `"up"`, `"down"`) |
| `error_msg` | string | Error details if the service is down. Empty string `""` if healthy |
| `category` | string | The service's category (`"Web"`, `"Host"`, etc.) |
| `ssl_exp` | string (UTC) or `null` | SSL certificate expiry. `null` if not applicable |

### Error Response

**Status:** `204 No Content` — No service matching that name was found on that node.

---

## Get Status of All Services

Returns the current status of every monitored service on a node, grouped by category.

### Endpoint

```
POST /view/service_current_stat
```

### Request

Query parameter:

```
node=<node_id>
```

### Response

**Status:** `200 OK`

```json
{
  "Web": [
    {
      "service_name": "nginx",
      "status": "up",
      "error_msg": "",
      "category": "Web",
      "ssl_exp": "2026-12-01T00:00:00Z"
    }
  ],
  "Host": [
    {
      "service_name": "postgres",
      "status": "down",
      "error_msg": "connection refused",
      "category": "Host",
      "ssl_exp": null
    }
  ]
}
```

| Field | Type | Description |
|-------|------|-------------|
| `service_name` | string | Name of the service |
| `status` | string | Current status (`"up"`, `"down"`) |
| `error_msg` | string | Error details if down. Empty string `""` if healthy |
| `category` | string | The service's category |
| `ssl_exp` | string (UTC) or `null` | SSL certificate expiry. `null` if not applicable |

Returns an empty object `{}` if the node has no monitored services.

---
