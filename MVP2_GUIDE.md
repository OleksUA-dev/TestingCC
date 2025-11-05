# CoreCRM-R MVP 2 Guide

## 🚀 What's New in MVP 2

MVP 2 introduces the **API Gateway** with dynamic CRUD operations and JWT authentication:

1. **JWT Authentication** - Secure user authentication and authorization
2. **Dynamic CRUD API** - Auto-generated REST endpoints for custom entities
3. **User Management** - Register, login, and manage users
4. **Metadata-Driven Operations** - CRUD operations based on entity metadata

## 🎯 Key Features

### Authentication & Authorization

- **JWT-based authentication** with configurable expiration
- **Role-based access control** (user vs admin)
- **Secure password hashing** with bcrypt
- **Protected endpoints** with middleware

### Dynamic CRUD

- **Automatic endpoint generation** for any entity created via metadata service
- **Type-safe operations** based on entity metadata
- **Validation** of required fields
- **Audit trails** (created_by, updated_by, created_at, updated_at)

## 📋 Prerequisites

- Rust 1.75+
- Docker and Docker Compose
- PostgreSQL 16+

## 🚀 Quick Start

### 1. Start All Services

```bash
# Start all services with Docker Compose
docker-compose up -d

# Or build from source
docker-compose up --build
```

This will start:
- **PostgreSQL** on port 5432 (with 2 databases: corecrm_metadata, corecrm_gateway)
- **Metadata Service** on port 8001
- **API Gateway** on port 8000

### 2. Verify Services

```bash
# Check metadata service
curl http://localhost:8001/health

# Check API gateway
curl http://localhost:8000/health
```

## 📚 API Reference

### Base URLs

- **API Gateway**: `http://localhost:8000`
- **Metadata Service**: `http://localhost:8001`

---

## Authentication API

### Register a New User

```bash
POST /api/v1/auth/register
Content-Type: application/json

{
  "username": "john_doe",
  "email": "john@example.com",
  "password": "securepassword123"
}
```

**Response** (201 Created):
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "john_doe",
  "email": "john@example.com",
  "is_active": true,
  "is_admin": false,
  "created_at": "2024-01-02T10:00:00Z"
}
```

### Login

```bash
POST /api/v1/auth/login
Content-Type: application/json

{
  "username": "john_doe",
  "password": "securepassword123"
}
```

**Response** (200 OK):
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 86400,
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "john_doe",
    "email": "john@example.com",
    "is_active": true,
    "is_admin": false,
    "created_at": "2024-01-02T10:00:00Z"
  }
}
```

**Save the `access_token` - you'll need it for authenticated requests!**

### Get Current User Info

```bash
GET /api/v1/auth/me
Authorization: Bearer <your-access-token>
```

**Response** (200 OK):
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "john_doe",
  "email": "john@example.com",
  "is_active": true,
  "is_admin": false,
  "created_at": "2024-01-02T10:00:00Z"
}
```

### Default Admin User

A default admin user is created during migration:

- **Username**: `admin`
- **Password**: `admin123`
- **Email**: `admin@corecrm.local`

**⚠️ Change this password in production!**

---

## Dynamic CRUD API

All dynamic CRUD endpoints require authentication. Include the JWT token in the `Authorization` header.

### General Pattern

```
POST   /api/v1/data/{entity_name}           - Create record
GET    /api/v1/data/{entity_name}           - List records
GET    /api/v1/data/{entity_name}/{id}      - Get record by ID
PUT    /api/v1/data/{entity_name}/{id}      - Update record
DELETE /api/v1/data/{entity_name}/{id}      - Delete record
```

---

## Complete Example Workflow

### Step 1: Create Entity (Metadata Service)

First, create an entity using the metadata service:

```bash
curl -X POST http://localhost:8001/api/v1/entities \
  -H "Content-Type: application/json" \
  -d '{
    "system_name": "contact",
    "display_name": "Контакт",
    "display_name_plural": "Контакти",
    "description": "Фізична особа"
  }'
```

Save the `entity_id` from the response.

### Step 2: Add Attributes

```bash
ENTITY_ID="<your-entity-id>"

# First Name
curl -X POST http://localhost:8001/api/v1/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$ENTITY_ID\",
    \"system_name\": \"first_name\",
    \"display_name\": \"Ім'я\",
    \"data_type\": \"string\",
    \"is_required\": true
  }"

# Last Name
curl -X POST http://localhost:8001/api/v1/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$ENTITY_ID\",
    \"system_name\": \"last_name\",
    \"display_name\": \"Прізвище\",
    \"data_type\": \"string\",
    \"is_required\": true
  }"

# Email
curl -X POST http://localhost:8001/api/v1/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$ENTITY_ID\",
    \"system_name\": \"email\",
    \"display_name\": \"Email\",
    \"data_type\": \"string\",
    \"is_required\": true,
    \"is_unique\": true
  }"

# Phone
curl -X POST http://localhost:8001/api/v1/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$ENTITY_ID\",
    \"system_name\": \"phone\",
    \"display_name\": \"Телефон\",
    \"data_type\": \"string\"
  }"
```

### Step 3: Generate Database Schema

```bash
curl -X POST http://localhost:8001/api/v1/entities/$ENTITY_ID/schema
```

### Step 4: Login to API Gateway

```bash
curl -X POST http://localhost:8000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "password": "admin123"
  }'
```

Save the `access_token` from the response.

### Step 5: Create Contact Record

```bash
TOKEN="<your-access-token>"

curl -X POST http://localhost:8000/api/v1/data/contact \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "first_name": "Іван",
    "last_name": "Петренко",
    "email": "ivan@example.com",
    "phone": "+380501234567"
  }'
```

**Response** (201 Created):
```json
{
  "id": "7c9e6679-7425-40de-944b-e07fc1f90ae7",
  "first_name": "Іван",
  "last_name": "Петренко",
  "email": "ivan@example.com",
  "phone": "+380501234567",
  "created_at": "2024-01-02T10:30:00Z",
  "updated_at": "2024-01-02T10:30:00Z"
}
```

### Step 6: List Contacts

```bash
curl http://localhost:8000/api/v1/data/contact \
  -H "Authorization: Bearer $TOKEN"
```

**Query Parameters**:
- `limit` - Number of records to return (default: 100)
- `offset` - Number of records to skip (default: 0)

Example:
```bash
curl "http://localhost:8000/api/v1/data/contact?limit=10&offset=0" \
  -H "Authorization: Bearer $TOKEN"
```

### Step 7: Get Contact by ID

```bash
CONTACT_ID="<contact-id-from-create-response>"

curl http://localhost:8000/api/v1/data/contact/$CONTACT_ID \
  -H "Authorization: Bearer $TOKEN"
```

### Step 8: Update Contact

```bash
curl -X PUT http://localhost:8000/api/v1/data/contact/$CONTACT_ID \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "phone": "+380509999999"
  }'
```

### Step 9: Delete Contact

```bash
curl -X DELETE http://localhost:8000/api/v1/data/contact/$CONTACT_ID \
  -H "Authorization: Bearer $TOKEN"
```

---

## Complete Bash Script Example

```bash
#!/bin/bash

# Configuration
METADATA_URL="http://localhost:8001"
GATEWAY_URL="http://localhost:8000"

echo "=== CoreCRM-R MVP 2 Demo ==="

# 1. Create Entity
echo -e "\n1. Creating 'contact' entity..."
ENTITY_RESPONSE=$(curl -s -X POST $METADATA_URL/api/v1/entities \
  -H "Content-Type: application/json" \
  -d '{
    "system_name": "contact",
    "display_name": "Контакт",
    "display_name_plural": "Контакти",
    "description": "Фізична особа"
  }')

ENTITY_ID=$(echo $ENTITY_RESPONSE | jq -r '.id')
echo "Entity ID: $ENTITY_ID"

# 2. Add Attributes
echo -e "\n2. Adding attributes..."

curl -s -X POST $METADATA_URL/api/v1/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$ENTITY_ID\",
    \"system_name\": \"first_name\",
    \"display_name\": \"Ім'я\",
    \"data_type\": \"string\",
    \"is_required\": true
  }" > /dev/null

curl -s -X POST $METADATA_URL/api/v1/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$ENTITY_ID\",
    \"system_name\": \"email\",
    \"display_name\": \"Email\",
    \"data_type\": \"string\",
    \"is_required\": true,
    \"is_unique\": true
  }" > /dev/null

echo "Attributes created"

# 3. Generate Schema
echo -e "\n3. Generating database schema..."
curl -s -X POST $METADATA_URL/api/v1/entities/$ENTITY_ID/schema | jq

# 4. Login
echo -e "\n4. Logging in..."
LOGIN_RESPONSE=$(curl -s -X POST $GATEWAY_URL/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "password": "admin123"
  }')

TOKEN=$(echo $LOGIN_RESPONSE | jq -r '.access_token')
echo "Access Token: ${TOKEN:0:50}..."

# 5. Create Contact
echo -e "\n5. Creating contact record..."
CONTACT_RESPONSE=$(curl -s -X POST $GATEWAY_URL/api/v1/data/contact \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "first_name": "Іван",
    "email": "ivan@example.com"
  }')

echo $CONTACT_RESPONSE | jq

CONTACT_ID=$(echo $CONTACT_RESPONSE | jq -r '.id')

# 6. List Contacts
echo -e "\n6. Listing contacts..."
curl -s $GATEWAY_URL/api/v1/data/contact \
  -H "Authorization: Bearer $TOKEN" | jq

# 7. Update Contact
echo -e "\n7. Updating contact..."
curl -s -X PUT $GATEWAY_URL/api/v1/data/contact/$CONTACT_ID \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "first_name": "Іван (оновлено)"
  }' | jq

echo -e "\n=== Demo Complete! ==="
```

Save this as `demo-mvp2.sh`, make it executable (`chmod +x demo-mvp2.sh`), and run it!

---

## Error Handling

All errors return a JSON response with an `error` field:

```json
{
  "error": "Error message here"
}
```

Common HTTP status codes:

- `400 Bad Request` - Invalid input or validation error
- `401 Unauthorized` - Missing or invalid authentication token
- `403 Forbidden` - Insufficient permissions
- `404 Not Found` - Resource not found
- `500 Internal Server Error` - Server error

---

## Security Best Practices

### For Development

The default configuration is suitable for development. Default admin credentials are provided for convenience.

### For Production

1. **Change JWT Secret**:
   ```bash
   export GATEWAY__JWT__SECRET="your-very-long-random-secret-key-here"
   ```

2. **Change Admin Password**:
   ```sql
   -- Connect to corecrm_gateway database
   UPDATE users
   SET password_hash = '$2b$12$...'  -- Generate new hash
   WHERE username = 'admin';
   ```

3. **Use HTTPS**:
   - Deploy behind a reverse proxy (nginx, Traefik)
   - Configure TLS certificates

4. **Environment Variables**:
   - Never commit `.env` files
   - Use secrets management (Kubernetes secrets, AWS Secrets Manager, etc.)

5. **Database Security**:
   - Use strong passwords
   - Limit network access
   - Enable SSL/TLS for database connections

---

## Architecture Overview

```
┌──────────────────────────────────────────────────┐
│                    Client                        │
└─────────────────────┬────────────────────────────┘
                      │
                      │ HTTP + JWT
                      │
                      ▼
┌──────────────────────────────────────────────────┐
│              API Gateway (8000)                   │
│  ┌─────────────────────────────────────────────┐ │
│  │  Authentication & Authorization Middleware  │ │
│  └─────────────────────────────────────────────┘ │
│  ┌─────────────────────────────────────────────┐ │
│  │      Dynamic CRUD Service                   │ │
│  │  • Fetches metadata from Metadata Service  │ │
│  │  • Validates requests                       │ │
│  │  • Executes dynamic SQL                     │ │
│  └─────────────────────────────────────────────┘ │
└────────┬──────────────────────┬──────────────────┘
         │                      │
         │                      │ HTTP
         │                      │
         │                      ▼
         │          ┌──────────────────────────────┐
         │          │  Metadata Service (8001)     │
         │          │  • Entity definitions        │
         │          │  • Attribute definitions     │
         │          │  • Schema generation         │
         │          └──────────────────────────────┘
         │                      │
         │                      │
         ▼                      ▼
┌──────────────────────────────────────────────────┐
│              PostgreSQL (5432)                    │
│  ┌─────────────────┐  ┌──────────────────────┐  │
│  │ corecrm_gateway │  │  corecrm_metadata    │  │
│  │  • users        │  │  • entities          │  │
│  │  • entity_*     │  │  • attributes        │  │
│  └─────────────────┘  └──────────────────────┘  │
└──────────────────────────────────────────────────┘
```

---

## Troubleshooting

### Service won't start

Check logs:
```bash
docker-compose logs api-gateway
docker-compose logs metadata-service
```

### Authentication fails

1. Check JWT secret is configured
2. Verify token hasn't expired
3. Ensure user exists and is active

### Dynamic CRUD fails

1. Verify entity exists in metadata service
2. Check that schema was generated (`POST /api/v1/entities/{id}/schema`)
3. Verify database table exists:
   ```bash
   docker exec -it corecrm-postgres psql -U corecrm -d corecrm_gateway \
     -c "\dt entity_*"
   ```

### Database connection issues

1. Check PostgreSQL is running:
   ```bash
   docker-compose ps postgres
   ```

2. Verify connection strings in `.env`

3. Check database exists:
   ```bash
   docker exec -it corecrm-postgres psql -U corecrm -l
   ```

---

## Next Steps (MVP 3)

MVP 3 will introduce:

1. **UI Engine** - Dynamic form and grid rendering
2. **Frontend Application** - React/Vue/Svelte client
3. **Enhanced RBAC** - Field-level permissions
4. **Relationships** - Many-to-many support

---

**Happy Building! 🚀**
