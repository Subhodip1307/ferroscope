# Ferroscope API – Node & Metrics Endpoints

These endpoints allow applications to query **nodes, system information, metrics, and monitored services** from a Ferroscope server.

These APIs are intended for developers building:

* Custom dashboards
* Mobile monitoring apps
* Web monitoring interfaces
* Automation systems

All endpoints in this section require **authentication**.

See the [Authentication Guide](authentication.md) for details on how to obtain an API token.

---

# Authentication

All `/view/*` endpoints require a valid **Authorization token**.

Include the token in the request header:

```
Authorization: <token>
```

If the token is missing or invalid the server will return:

```
401 Unauthorized
```

---

# Base Path

```
/view
```

---

# Get Node List

Returns all registered nodes connected to the Ferroscope server.

### Endpoint

```
POST /view/get_node_list
```

### Request

No request body required.

### Response

```
200 OK
```

```json
[
  {
    "id": 1,
    "name": "production-server"
  },
  {
    "id": 2,
    "name": "backup-node"
  }
]
```

---

# Get Node System Information

Returns system information for a specific node.

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

If the node has no data:

```
204 No Content
```

---

# Get Latest CPU Usage

Returns the **latest CPU usage metric** for a node.

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

```json
{
  "value": 43.7,
  "date_time": "2026-03-15T10:45:33Z"
}
```

If no data exists:

```
204 No Content
```


For real-time CPU updates, refer to the [Streaming API](streaming.md).

---

# Get Latest RAM Usage

Returns the most recent memory statistics for a node.

### Endpoint

```
POST /view/get_latest_ram
```

### Request

Query parameter:

```
node=<node_id>
```

Example

```
/view/get_latest_ram?node=1
```

### Response

```json
{
  "total": "16000000",
  "free": "9200000",
  "timestamp": "2026-03-15T10:45:33Z"
}
```

For real-time RAM updates, refer to the [Streaming API](streaming.md).


---

# CPU Usage History

Returns the **latest 20 CPU usage records**.

### Endpoint

```
POST /view/cpu_stat
```

### Request

Query parameter

```
node=<node_id>
```

### Response

```json
[
  {
    "value": 41.2,
    "date_time": "2026-03-15T10:45:33Z"
  },
  {
    "value": 38.7,
    "date_time": "2026-03-15T10:44:33Z"
  }
]
```

---

# RAM Usage History

Returns the **latest 20 RAM usage records**.

### Endpoint

```
POST /view/ram_stat
```

### Request

Query parameter

```
node=<node_id>
```

### Response

```json
[
  {
    "free": "9200000",
    "total": "16000000",
    "timestamp": "2026-03-15T10:45:33Z"
  },
  {
    "free": "9150000",
    "total": "16000000",
    "timestamp": "2026-03-15T10:44:33Z"
  }
]
```

---

# List Services on a Node

Returns all monitored services running on a node.

### Endpoint

```
POST /view/node_services
```

### Request

Query parameter

```
node=<node_id>
```

### Response

```json
[
  {
    "service_name": "nginx",
    "category":"<Host/Web>"
  },
  {
    "service_name": "postgres",
    "category":"<Host/Web>"
  }
]
```

---

# Get Status of a Single Service

Returns the current status of a specific service.

### Endpoint

```
POST /view/single_service_current_stat
```

### Request Body

```json
{
  "node": 1,
  "service_name": "nginx",
}
```

### Response

```json
{
  "status": "running",
  "service_status": "Reachable",
  "error_msg": "",
  "category":"<Host/Web>",
  "ssl_exp":"null_or_utc_time"
}
```

Possible service status values:

```
Reachable
Unreachable
```

If the service is not found:

```
204 No Content
```

---

# Get Status of All Services

Returns the status of every monitored service on a node.

### Endpoint

```
POST /view/service_current_stat
```

### Request

Query parameter

```
node=<node_id>
```

### Response

```json
[
  {
    "service_name": "nginx",
    "status": "running",
    "service_status": "Reachable",
    "error_msg": "",
    "category":"Host",
    "ssl_exp":null
  },
  {
    "service_name": "postgres",
    "status": "running",
    "service_status": "Reachable",
    "error_msg": "",
    "category":"Web",
    "ssl_exp":"utc_time"
  }
]
```

---

# Create Node

Registers a new node in the Ferroscope server.

The server returns an authentication token that the node should use when sending metrics.

### Endpoint

```
POST /view/create_nodes
```

### Request Body

```json
{
  "name": "production-node"
}
```

### Response

```json
{
  "token": "e3c6c9b3-49d0-4b8e-a45e-7f6b0e7d0a8b"
}
```

If the node creation fails:

```
409 Conflict
```

---
