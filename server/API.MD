# Rust Ferroscope Monitoring API Documentation

Base URL:

```
http://192.168.0.153:900/view
```

All endpoints use the **POST** method.

---

## 1. Login

**Endpoint:**

```
POST /user_login
```

**Description:**
Login User

**Body**

```json
{
  "username":"",
  "password":""
}

```


**Response:** `200 OK`

```json
  {
    
    "token": "node-1"
  }

```


---


## 2. Get Node List

**Endpoint:**

```
POST /get_node_list
```

**Description:**
Returns all available nodes (agents).

**Response:** `200 OK`

```json
[
  {
    "id": 1,
    "name": "node-1"
  }
]
```


---

## 3. Sysinfo/Node info

**Endpoint:**

```
POST /get_node_info?node=<node_id>
```

**Description:**
Gives Sysinfo about a Node (agents).
Takes node id in url as parameter

**Response:** `200 OK`

```json

  {
    "system_name": "String",
    "kernel_version": "String",
    "os_version":"String",
    "uptime": 64,
    "cpu_threads": 16,
    "cpu_vendor":"String",
  }

```


---


## 4. Get Latest CPU Usage

**Endpoint:**

```
POST /get_latest_cpu?node=<node_id>
```

**Description:**
Returns the latest CPU usage of a specific node.

**Query Parameters:**

* `node` (i64) – Node ID

**Response:** `200 OK`

```json
{
  "value": 23.5,
  "date_time": "2026-02-18T10:30:00Z"
}
```


---

## 5. Get Latest RAM Usage

**Endpoint:**

```
POST /get_latest_ram?node=<node_id>
```

**Description:**
Returns the latest RAM usage of a specific node.

**Query Parameters:**

* `node` (i64) – Node ID

**Response:**

* `200 OK` – Returns latest RAM data
* `204 No Content` – If no data is available

```json
{
  "free": "2GB",
  "total": "8GB",
  "timestamp": "2026-02-18T10:30:00Z"
}
```


---

## 6. CPU History (Last 20 Records)

**Endpoint:**

```
POST /cpu_stat?node=<node_id>
```

**Description:**
Returns the last 20 CPU usage records of a node.

**Query Parameters:**

* `node` (i64) – Node ID

**Response:** `200 OK`

```json
[
  {
    "value": 20.1,
    "date_time": "2026-02-18T10:10:00Z"
  }
]
```



---

## 7. RAM History (Last 20 Records)

**Endpoint:**

```
POST /ram_stat?node=<node_id>
```

**Description:**
Returns the last 20 RAM usage records of a node.

**Query Parameters:**

* `node` (i64) – Node ID

**Response:** `200 OK`

```json
[
  {
    "free": "2GB",
    "total": "8GB",
    "timestamp": "2026-02-18T10:10:00Z"
  }
]
```


---

## 8. Get All Services of a Node

**Endpoint:**

```
POST /node_services?node=<node_id>
```

**Description:**
Returns all services running on a specific node.

**Query Parameters:**

* `node` (i64) – Node ID

**Response:** `200 OK`

```json
[
  {
    "service_name": "nginx"
  }
]
```


---

## 9. Get All Services Current Status

**Endpoint:**

```
POST /service_current_stat?node=<node_id>
```

**Description:**
Returns the current status of all service on a node.

**Query Parameters:**

* `node` (i64) – Node ID

**Response:**

* `200 OK` – Service status found
* `204 No Content` – If no data found

```json
[{
  "service_name": "nginx",
  "status": "up",
  "service_status":"Reachable",
  "error_msg":"",
}]
```
---

## 10. Get Service Current Status

**Endpoint:**

```
POST /single_service_current_stat
```

**Description:**
Returns the current status of specific service on a node.

**Request Body:**

```json
{
  "node": 1,
   "service_name":"<service_name>"
}
```

**Response:**

* `200 OK` – Service status found
* `204 No Content` – If no data found

```json
{
  "error_msg":"",
  "status": "up",
  "service_status":"Reachable"
}
```
---

# Summary

* Base URL: `http://192.168.0.153:900/view`
* All routes use POST method
* Query parameter for node-based endpoints: `?node=<node_id>`
* History endpoints return last 20 records only
* Some endpoints may return `204 No Content` when data is unavailable
