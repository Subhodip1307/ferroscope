# Ferroscope Streaming API

Ferroscope provides **real-time metric streaming** using **Server-Sent Events (SSE)**.
These endpoints allow applications to subscribe to live updates instead of repeatedly polling the REST API.

Streaming is ideal for:

* Live monitoring dashboards
* Real-time alerting systems
* Custom monitoring interfaces
* Mobile or web dashboards

If you only need the **latest metric snapshot**, see the standard API endpoints in the main API documentation.

---

# Index

* [Base Path](#base-path)
<!-- * [Authentication](#authentication) -->
* [How Streaming Works](#how-streaming-works)
* [Stream CPU Metrics](#stream-cpu-metrics)
* [Stream RAM Metrics](#stream-ram-metrics)
* [Example: JavaScript Client](#example-javascript-client)
* [Example: cURL](#example-curl)
* [Notes](#notes)

---

# Base Path

```
/stream
```

---



# How Streaming Works

Ferroscope streaming APIs use **Server-Sent Events (SSE)**.

When a client connects:

1. The HTTP connection remains open.
2. The server continuously sends new metric updates.
3. Each update is delivered as a JSON event.

This allows applications to receive **live monitoring data without polling the API repeatedly**.

---

# Stream CPU Metrics

Streams **live CPU usage metrics** for a specific node.

## Endpoint

```
GET /stream/cpu
```

## Query Parameter

```
node=<node_id>
```

Example request:

```
/stream/cpu?node=1
```

## Example Event

Each event contains the latest CPU usage data.

```json
{
  "value": 42.8,
  "date_time": "2026-03-15T11:10:21Z"
}
```

### Possible Error

If the node is not currently streaming metrics:

```
503 Service Unavailable
```

---

# Stream RAM Metrics

Streams **live RAM usage metrics** for a specific node.

## Endpoint

```
GET /stream/ram
```

## Query Parameter

```
node=<node_id>
```

Example request:

```
/stream/ram?node=1
```

## Example Event

```json
{
  "free": "9243000",
  "total": "16000000",
  "timestamp": "2026-03-15T11:10:21Z"
}
```

### Possible Error

If the node stream does not exist:

```
503 Service Unavailable
```

---

# Example: JavaScript Client

```javascript
const source = new EventSource(
  "<server-url>/stream/cpu?node=1"
);

source.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log("CPU Usage:", data.value);
};
```

---

# Example: cURL

```
curl -N \
-H "Authorization: <token>" \
"<server-url>/stream/cpu?node=1"
```

The `-N` flag disables buffering so the stream is displayed in real time.

---

# Notes

* Streaming uses **HTTP Server-Sent Events (SSE)**.
* Connections remain open until the client disconnects.
* Clients should implement **automatic reconnection** if the connection drops.
* These endpoints are optimized for **real-time monitoring dashboards**.

---
