# FerroScope

![Language](https://img.shields.io/badge/built%20with-Rust-orange)
![Status](https://img.shields.io/badge/status-work%20in%20progress-yellow)
![License](https://img.shields.io/badge/license-Apache--2.0-blue)

## Introduction

**FerroScope** is a **lightweight, distributed system monitoring platform written in Rust**. It follows a simple **client–server architecture** and is built to give you real-time visibility into your servers and services while staying resource-efficient, secure, and effortless to set up.

The goal of FerroScope is to offer monitoring that is **simple to install, light on resources, and quick to configure** — without the complexity and infrastructure overhead of traditional enterprise monitoring systems.

FerroScope is built with a focus on:

- Resource-optimized, lightweight architecture
- Minimal system resource usage
- Simple installation and configuration
- Real-time system and service monitoring
- Built-in alerting and notifications
- Minimal attack surface and secure communication

---
> 📦 **Want to deploy FerroScope?**
> - Server & Web UI → **[Deployment Guide](server/SERVER.md)**
> - Monitoring Agent → **[Agent Deployment Guide](agent/AGENT.MD)**

## Notifications

FerroScope doesn't just collect metrics — it tells you the moment something goes wrong. Built-in notifications keep you informed about the health of your infrastructure in real time.

You can get notified for events such as:

- **Node down** — an agent stops reporting or a monitored machine goes offline
- **Node recovered** — a previously down node comes back online
- **Service down** — a monitored service becomes unavailable
- **Service recovered** — a service is back up
- **SSL/TLS certificate expiry** — advance warnings before certificates expire, so you can renew in time

This means you find out about outages and expiring certificates *before* they turn into bigger problems.

---

## Features

FerroScope is designed around simplicity and efficiency, delivering the essentials of monitoring while staying lightweight and easy to operate.

Current and planned features include:

- Distributed, multi-node monitoring
- Lightweight monitoring agents written in Rust
- Real-time system metrics (CPU, RAM, etc.)
- Service availability monitoring
- Real-time metric streaming using Server-Sent Events (SSE)
- Built-in alerting and notifications (node/service downtime, SSL/TLS expiry, and more)
- Simple node registration and management
- Secure token-based authentication
- Minimal configuration requirements
- Low system resource consumption

The project is actively evolving, and additional monitoring features will be added over time.

---

## Architecture

FerroScope follows a **distributed client–server architecture** consisting of two primary components.

### FerroScope Server

The server acts as the **central monitoring hub**.

Responsibilities include:

- Receiving metrics from agents
- Storing monitoring data
- Managing registered nodes
- Tracking service and node health
- Evaluating alert conditions and dispatching notifications
- Providing real-time metric streaming

### Monitoring Agent

Each monitored machine runs a **FerroScope Agent**, the client side of the system.

The agent is responsible for:

- Collecting system metrics
- Monitoring configured services
- Sending monitoring data to the FerroScope server
- Maintaining a lightweight and efficient footprint

The agent is designed around the following principles:

- **Minimal resource usage**
- **Minimal network noise**
- **Low system impact**
- **Simple configuration**
- **Secure communication with the server**

Agents are intentionally built to generate **as little monitoring overhead as possible**, making FerroScope suitable for both small and large deployments.

---

## Why FerroScope?

Many monitoring systems are powerful but come with significant complexity and infrastructure overhead.

FerroScope aims to provide a simpler alternative by focusing on:

- Easy installation and setup
- Resource-optimized, lightweight architecture
- Minimal configuration
- Real-time monitoring capabilities
- Built-in alerting so you're notified the moment something breaks
- Low operational overhead

The result is a monitoring system you can stand up quickly and run without babysitting.

---



## Creator

<a href="https://github.com/Subhodip1307">
  <img src="https://github.com/Subhodip1307.png" width="80px" style="border-radius:50%" alt="Subhodip1307"/>
</a>

---

## Contributors

<table>
<tr>
<td align="center">
<a href="https://github.com/ssdev38">
<img src="https://github.com/ssdev38.png" width="80px" style="border-radius:50%" alt="ssdev38"/>
</a>
</td>
</tr>
<tr>
<td align="center">Web UI Development</td>
</tr>
</table>

---


## License

FerroScope is licensed under the **Apache-2.0 License**. See the [LICENSE](LICENSE) file for details.
