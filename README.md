# FerroScope

## Introduction

**FerroScope** is a **distributed system monitoring platform written in Rust** designed to provide real-time visibility into servers and services while remaining lightweight, secure, and easy to operate.

The primary goal of FerroScope is to offer a monitoring solution that is **simple to install, minimal in resource usage, and easy to integrate**, without the complexity of traditional enterprise monitoring systems.

FerroScope is built with a focus on:

- Lightweight architecture
- Minimal system resource usage
- Simple installation and configuration
- Real-time system and service monitoring
- Minimal attack surface
- API-first design for custom dashboards and integrations

The Ferroscope server exposes a **public API**, allowing developers to build their own monitoring dashboards, automation tools, or integrations.

For API usage and integration details, see the **[API Documentation](API-DOCS/README.md)**.

---

## Project Status

🚧 **Work in Progress**

FerroScope is currently under heavy development and active testing.  
Core features are being implemented and refined, and the architecture may evolve as the project matures.

Planned improvements include:

- Additional system metrics
- More service monitoring capabilities
- Improved distributed node communication
- Performance optimizations
- Web-based monitoring interface
- Alerting and notification support

---


## Features

FerroScope is designed with simplicity and efficiency in mind. The system focuses on delivering essential monitoring capabilities while remaining lightweight and easy to operate.

Current and planned features include:

- Distributed node monitoring
- Lightweight monitoring agents
- Real-time system metrics (CPU, RAM, etc.)
- Service availability monitoring
- Live metric streaming using Server-Sent Events (SSE)
- API-first architecture for custom dashboards
- Simple node registration and management
- Minimal configuration requirements
- Low system resource consumption
- Secure token-based authentication

The project is actively evolving and additional monitoring features will be added over time.

---

## Architecture

FerroScope follows a **distributed monitoring architecture** consisting of two primary components:

### Ferroscope Server

The server acts as the **central monitoring hub**.

Responsibilities include:

- Receiving metrics from nodes
- Storing monitoring data
- Managing registered nodes
- Tracking service health
- Exposing APIs for dashboards and integrations
- Providing real-time metric streaming

The server exposes a **REST API and streaming endpoints**, allowing developers to build their own monitoring dashboards, automation tools, or integrations.

---

### Monitoring Agent

Each monitored machine runs a **FerroScope Agent**.

The agent is responsible for:

- Collecting system metrics
- Monitoring configured services
- Sending monitoring data to the Ferroscope server
- Maintaining a lightweight and efficient footprint

The agent is designed with the following principles:

- **Minimal resource usage**
- **Minimal network noise**
- **Low system impact**
- **Simple configuration**
- **Secure communication with the server**

Agents are intentionally built to generate **as little monitoring overhead as possible**, making FerroScope suitable for both small and large deployments.

---

## Why FerroScope?

Many monitoring systems are powerful but often come with significant complexity and infrastructure overhead.

FerroScope aims to provide a simpler alternative by focusing on:

- Easy installation
- Lightweight architecture
- Minimal configuration
- Developer-friendly APIs
- Real-time monitoring capabilities
- Low operational overhead

The project is designed to be **developer-friendly**, allowing teams to easily build custom monitoring dashboards or integrations on top of the Ferroscope API.


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