# Ferroscope API Documentation

This directory contains the **official API documentation for the Ferroscope server**.

Ferroscope is designed with an **API-first architecture**, allowing developers to build their own tools, dashboards, or integrations on top of a Ferroscope server.

The APIs make it possible to retrieve monitoring data, manage nodes, stream live metrics, and interact with the monitoring system programmatically.

---

# Why These APIs Exist

Ferroscope intentionally separates the **monitoring backend** from the **user interface**.

This allows developers and organizations to build their own interfaces depending on their needs.

Using the Ferroscope API, you can build:

* Custom monitoring dashboards
* Mobile monitoring applications (Android / iOS)
* Web-based monitoring panels
* CLI tools
* Automation systems
* Internal monitoring platforms
* Integration with other monitoring or alerting systems

The Ferroscope server acts as the **central monitoring engine**, while the API allows any external application to interact with it.

---

# Design Philosophy

The API is designed with the following goals:

* Simple and predictable endpoints
* Lightweight responses
* Easy integration with any programming language
* Real-time monitoring capabilities
* Minimal complexity for developers

The API documentation focuses on **clarity and practical usage**, making it easier to build external tools on top of Ferroscope.

---

# API Documentation Index

The API documentation is organized into multiple sections.

## Authentication

Authentication endpoints used for logging in and managing user credentials.

* [Authentication API](authentication.md)

---

## View APIs

Endpoints used to retrieve monitoring data such as:

* Node list
* System information
* CPU metrics
* RAM metrics
* Service status

These endpoints are primarily used by dashboards and monitoring tools.

* [View API](user_view.md)

---

## Streaming APIs

Streaming endpoints provide **real-time monitoring data** using Server-Sent Events (SSE).

These APIs allow applications to receive live updates for:

* CPU usage
* RAM usage

Streaming APIs are typically used by **live dashboards and monitoring interfaces**.

* [Streaming API](streaming.md)

---

# Getting Started

1. Authenticate using the login endpoint.
2. Include the returned token in the `Authorization` header.
3. Use the View APIs to retrieve monitoring data.
4. Use the Streaming APIs to receive real-time updates.

For authentication details, see:

* [Authentication Guide](authentication.md)

---
