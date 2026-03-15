# Ferroscope Authentication API

Ferroscope uses **token-based authentication** to protect API endpoints.  
Clients must first authenticate using their **username and password** to obtain an API token.

This token must then be included in requests to all protected endpoints.

---

# Index

- [Authentication Flow](#authentication-flow)
- [Login](#login)
- [Using the Authentication Token](#using-the-authentication-token)
- [Unauthorized Requests](#unauthorized-requests)
- [Changing User Password](#changing-user-password)
- [Getting User Information](#getting-user-information)
- [Security Notes](#security-notes)

---

# Authentication Flow

1. The user logs in using their **username and password**.
2. The server validates the credentials.
3. The server returns an **authentication token**.
4. The token must be included in all protected API requests.

---

# Login

Authenticates a user and returns a token that can be used to access protected endpoints.

## Endpoint

```
POST /auth/user_login
```

## Request Body

```json
{
  "username": "admin",
  "password": "secure_password"
}
```

## Successful Response

```json
{
  "token": "b2a0fba6-7a5b-4f8c-9a71-32f8b9b5c1d1"
}
```

The returned token must be stored by the client and used for future requests.

---

# Using the Authentication Token

All protected endpoints require the token to be sent in the **Authorization header**.

Example:

```
Authorization: <token>
```

Example request using curl:

```
curl -X POST http://<server-url>/view/get_node_list \
-H "Authorization: b2a0fba6-7a5b-4f8c-9a71-32f8b9b5c1d1"
```

If the token is valid, the request will be processed normally.

---

# Unauthorized Requests

If the token is missing or invalid, the server will return:

```
401 Unauthorized
```

Possible reasons:

* Authorization header not provided
* Token is invalid
* Token does not exist in the server database

---

# Changing User Password

Users can change their password using the following endpoint.

## Endpoint

```
POST /view/change_password
```

## Request Body

```json
{
  "username": "admin",
  "password": "new_secure_password"
}
```

## Response

```
200 OK
```

If authentication fails:

```
401 Unauthorized
```

---

# Getting User Information

Returns the authenticated user's information.

## Endpoint

```
POST /view/get_userdetails
```

## Headers

```
Authorization: <token>
```

## Example Response

```json
{
  "user_id": 1,
  "username": "admin"
}
```

---

# Security Notes

* Always keep authentication tokens private.
* Do not expose tokens in client-side code for public applications.
* Rotate tokens if a security breach is suspected.
* Use HTTPS when deploying Ferroscope in production.

---
