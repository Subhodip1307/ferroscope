# Ferroscope — Authentication API

Ferroscope uses **token-based authentication** to protect its API endpoints.  
You first log in with your **username and password** to get a token, then include that token in every protected request.

---

## Index

- [How Authentication Works](#how-authentication-works)
- [Login](#login)
- [Using Your Token](#using-your-token)
- [Unauthorized Requests](#unauthorized-requests)
- [Get Your User Info](#get-your-user-info)
- [Change Username or Password](#change-username-or-password)
- [Security Notes](#security-notes)

---

## How Authentication Works

1. You call `/auth/user_login` with your username and password.
2. The server checks your credentials.
3. If valid, it returns a **token** (a unique string).
4. You include that token in the `Authorization` header for all further requests.

> Each time you log in, your old token is deleted and a fresh one is issued. Only one active token per user exists at a time.

---

## Login

Validates your credentials and returns an auth token.

### Endpoint

```
POST /auth/user_login
```

### Request Body

```json
{
  "username": "admin",
  "password": "your_password"
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `username` | string | ✅ Yes | Your account username |
| `password` | string | ✅ Yes | Your account password |

### Success Response

**Status:** `200 OK`

```json
{
  "token": "b2a0fba6-7a5b-4f8c-9a71-32f8b9b5c1d1"
}
```

**Save this token.** You will need it for every protected request.

### Error Response

**Status:** `200` with error body — returned when username is not found or password is wrong:

```json
{
  "msg": "no user found"
}
```

> ℹ️ Both "user not found" and "wrong password" return the same message intentionally — this avoids revealing which accounts exist.

---

## Using Your Token

Include the token in the `Authorization` header on every protected request.

```
Authorization: <your-token>
```

**Example using curl:**

```bash
curl -X POST http://<server-url>/view/get_node_list \
  -H "Authorization: b2a0fba6-7a5b-4f8c-9a71-32f8b9b5c1d1"
```

---

## Unauthorized Requests

If the token is missing or invalid, the server returns:

```
401 Unauthorized
```

Common reasons:

- The `Authorization` header was not included in the request
- The token is incorrect or malformed
- The token does not exist (e.g., user logged in again and a new token was issued)

---

## Get Your User Info

Returns the currently authenticated user's ID and username. Requires a valid token.

### Endpoint

```
POST /auth/get_userdetails
```

### Headers

```
Authorization: <your-token>
```

### Success Response

**Status:** `200 OK`

```json
{
  "user_id": 1,
  "username": "admin"
}
```

| Field | Type | Description |
|-------|------|-------------|
| `user_id` | integer | Your internal numeric user ID |
| `username` | string | Your current username |

---

## Change Username or Password

Updates your username and/or password. Requires a valid token. Both fields must be provided even if you are only changing one of them.

### Endpoint

```
POST /auth/change_password
```

### Headers

```
Authorization: <your-token>
```

### Request Body

```json
{
  "username": "new_username",
  "password": "new_password"
}
```

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `username` | string | ✅ Yes | Your new (or existing) username |
| `password` | string | ✅ Yes | Your new (or existing) password |

> ℹ️ If you only want to change your password, pass your current username in the `username` field. Same applies in reverse.

### Success Response

**Status:** `200 OK` — Update was successful. No response body.

### Error Response

**Status:** `409 Conflict` — The update failed, likely because the new username is already taken.

---

## Security Notes

- Keep your token private — treat it like a password.
- Do not hardcode tokens in frontend/client-side code for public-facing apps.
- If you suspect your token has been compromised, log in again — this invalidates the old token and issues a new one.
- Always use **HTTPS** in production to prevent tokens from being intercepted.