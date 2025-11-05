# CoreCRM-R API Examples

This document provides practical examples for using the CoreCRM-R Metadata Service API.

## Prerequisites

Make sure the metadata service is running:
```bash
make dev
# or
cargo run --package metadata-service
```

The service will be available at: `http://localhost:8001`

## Example 1: Creating a Simple "Contact" Entity

### Step 1: Create the Entity

```bash
curl -X POST http://localhost:8001/api/v1/entities \
  -H "Content-Type: application/json" \
  -d '{
    "system_name": "contact",
    "display_name": "Контакт",
    "display_name_plural": "Контакти",
    "description": "Фізична особа - клієнт або партнер",
    "icon": "user"
  }'
```

**Response**:
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "system_name": "contact",
  "display_name": "Контакт",
  "display_name_plural": "Контакти",
  "description": "Фізична особа - клієнт або партнер",
  "is_system": false,
  "is_active": true,
  "icon": "user",
  "created_at": "2024-01-01T10:00:00Z",
  "updated_at": "2024-01-01T10:00:00Z",
  "created_by": null,
  "updated_by": null
}
```

**Save the `id` field - you'll need it for the next steps!**

### Step 2: Add Attributes to the Entity

Let's add several fields to our Contact entity:

#### First Name (String, Required)
```bash
ENTITY_ID="550e8400-e29b-41d4-a716-446655440000"  # Replace with your entity ID

curl -X POST http://localhost:8001/api/v1/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$ENTITY_ID\",
    \"system_name\": \"first_name\",
    \"display_name\": \"Ім'я\",
    \"description\": \"Ім'я контакту\",
    \"data_type\": \"string\",
    \"is_required\": true,
    \"is_unique\": false
  }"
```

#### Last Name (String, Required)
```bash
curl -X POST http://localhost:8001/api/v1/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$ENTITY_ID\",
    \"system_name\": \"last_name\",
    \"display_name\": \"Прізвище\",
    \"data_type\": \"string\",
    \"is_required\": true
  }"
```

#### Email (String, Required, Unique)
```bash
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
```

#### Phone (String, Optional)
```bash
curl -X POST http://localhost:8001/api/v1/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$ENTITY_ID\",
    \"system_name\": \"phone\",
    \"display_name\": \"Телефон\",
    \"data_type\": \"string\",
    \"is_required\": false
  }"
```

#### Birth Date (Date, Optional)
```bash
curl -X POST http://localhost:8001/api/v1/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$ENTITY_ID\",
    \"system_name\": \"birth_date\",
    \"display_name\": \"Дата народження\",
    \"data_type\": \"date\",
    \"is_required\": false
  }"
```

#### Is Active (Boolean, Default: true)
```bash
curl -X POST http://localhost:8001/api/v1/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$ENTITY_ID\",
    \"system_name\": \"is_active\",
    \"display_name\": \"Активний\",
    \"data_type\": \"boolean\",
    \"is_required\": true,
    \"default_value\": true
  }"
```

### Step 3: Generate Database Schema

```bash
curl -X POST http://localhost:8001/api/v1/entities/$ENTITY_ID/schema
```

**Response**:
```json
{
  "table_name": "entity_contact",
  "sql": "CREATE TABLE IF NOT EXISTS entity_contact (\n    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),\n    first_name VARCHAR(255) NOT NULL,\n    last_name VARCHAR(255) NOT NULL,\n    email VARCHAR(255) NOT NULL UNIQUE,\n    phone VARCHAR(255),\n    birth_date DATE,\n    is_active BOOLEAN NOT NULL DEFAULT true,\n    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),\n    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),\n    created_by UUID,\n    updated_by UUID\n);",
  "success": true,
  "message": "Table 'entity_contact' created successfully"
}
```

### Step 4: Verify Table Creation

```bash
docker exec -it corecrm-postgres psql -U corecrm -d corecrm_metadata -c "\d entity_contact"
```

## Example 2: Creating an "Account" Entity with Enum

### Step 1: Create Account Entity

```bash
curl -X POST http://localhost:8001/api/v1/entities \
  -H "Content-Type: application/json" \
  -d '{
    "system_name": "account",
    "display_name": "Контрагент",
    "display_name_plural": "Контрагенти",
    "description": "Юридична особа",
    "icon": "building"
  }'
```

Save the entity ID from the response.

### Step 2: Add Attributes

#### Company Name
```bash
ACCOUNT_ID="<your-account-entity-id>"

curl -X POST http://localhost:8001/api/v1/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$ACCOUNT_ID\",
    \"system_name\": \"company_name\",
    \"display_name\": \"Назва компанії\",
    \"data_type\": \"string\",
    \"is_required\": true,
    \"is_unique\": true
  }"
```

#### Company Type (Enum)
```bash
curl -X POST http://localhost:8001/api/v1/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$ACCOUNT_ID\",
    \"system_name\": \"company_type\",
    \"display_name\": \"Тип компанії\",
    \"data_type\": \"enum\",
    \"is_required\": true,
    \"enum_values\": [
      {\"key\": \"llc\", \"value\": \"ТОВ\", \"display_order\": 1},
      {\"key\": \"jsc\", \"value\": \"ТОВ\", \"display_order\": 2},
      {\"key\": \"individual\", \"value\": \"ФОП\", \"display_order\": 3}
    ]
  }"
```

#### Tax ID
```bash
curl -X POST http://localhost:8001/api/v1/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$ACCOUNT_ID\",
    \"system_name\": \"tax_id\",
    \"display_name\": \"ЄДРПОУ/ІПН\",
    \"data_type\": \"string\",
    \"is_required\": true,
    \"is_unique\": true
  }"
```

#### Annual Revenue (Decimal for financial data)
```bash
curl -X POST http://localhost:8001/api/v1/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$ACCOUNT_ID\",
    \"system_name\": \"annual_revenue\",
    \"display_name\": \"Річний оборот\",
    \"data_type\": \"decimal\",
    \"is_required\": false
  }"
```

### Step 3: Generate Schema
```bash
curl -X POST http://localhost:8001/api/v1/entities/$ACCOUNT_ID/schema
```

## Example 3: Creating a Lookup Relationship

Let's create an "Opportunity" entity that references Contact and Account.

### Step 1: Create Opportunity Entity

```bash
curl -X POST http://localhost:8001/api/v1/entities \
  -H "Content-Type: application/json" \
  -d '{
    "system_name": "opportunity",
    "display_name": "Угода",
    "display_name_plural": "Угоди",
    "description": "Можливість продажу",
    "icon": "dollar-sign"
  }'
```

### Step 2: Add Lookup Attributes

#### Link to Account (Lookup)
```bash
OPPORTUNITY_ID="<your-opportunity-entity-id>"
ACCOUNT_ID="<your-account-entity-id>"  # From Example 2

curl -X POST http://localhost:8001/api/v1/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$OPPORTUNITY_ID\",
    \"system_name\": \"account_id\",
    \"display_name\": \"Контрагент\",
    \"data_type\": \"lookup\",
    \"is_required\": true,
    \"lookup_entity_id\": \"$ACCOUNT_ID\"
  }"
```

#### Link to Contact (Lookup)
```bash
CONTACT_ID="<your-contact-entity-id>"  # From Example 1

curl -X POST http://localhost:8001/api/v1/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$OPPORTUNITY_ID\",
    \"system_name\": \"contact_id\",
    \"display_name\": \"Контактна особа\",
    \"data_type\": \"lookup\",
    \"is_required\": false,
    \"lookup_entity_id\": \"$CONTACT_ID\"
  }"
```

#### Opportunity Name
```bash
curl -X POST http://localhost:8001/api/v1/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$OPPORTUNITY_ID\",
    \"system_name\": \"name\",
    \"display_name\": \"Назва угоди\",
    \"data_type\": \"string\",
    \"is_required\": true
  }"
```

#### Amount (Decimal)
```bash
curl -X POST http://localhost:8001/api/v1/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$OPPORTUNITY_ID\",
    \"system_name\": \"amount\",
    \"display_name\": \"Сума\",
    \"data_type\": \"decimal\",
    \"is_required\": true
  }"
```

#### Close Date
```bash
curl -X POST http://localhost:8001/api/v1/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$OPPORTUNITY_ID\",
    \"system_name\": \"close_date\",
    \"display_name\": \"Дата закриття\",
    \"data_type\": \"date\",
    \"is_required\": true
  }"
```

### Step 3: Generate Schema with Foreign Keys
```bash
curl -X POST http://localhost:8001/api/v1/entities/$OPPORTUNITY_ID/schema
```

The generated SQL will include foreign key constraints!

## Querying Metadata

### List All Entities
```bash
curl http://localhost:8001/api/v1/entities
```

### Get Specific Entity
```bash
curl http://localhost:8001/api/v1/entities/$ENTITY_ID
```

### List Attributes for Entity
```bash
curl http://localhost:8001/api/v1/entities/$ENTITY_ID/attributes
```

### Get Specific Attribute
```bash
ATTRIBUTE_ID="<attribute-uuid>"
curl http://localhost:8001/api/v1/attributes/$ATTRIBUTE_ID
```

## Updating Metadata

### Update Entity Display Name
```bash
curl -X PUT http://localhost:8001/api/v1/entities/$ENTITY_ID \
  -H "Content-Type: application/json" \
  -d '{
    "display_name": "Контакт (Оновлено)",
    "is_active": true
  }'
```

### Update Attribute
```bash
curl -X PUT http://localhost:8001/api/v1/attributes/$ATTRIBUTE_ID \
  -H "Content-Type: application/json" \
  -d '{
    "display_name": "Email (обов'язковий)",
    "is_required": true
  }'
```

## Deleting Metadata

### Delete Attribute
```bash
curl -X DELETE http://localhost:8001/api/v1/attributes/$ATTRIBUTE_ID
```

### Delete Entity (and all its attributes)
```bash
curl -X DELETE http://localhost:8001/api/v1/entities/$ENTITY_ID
```

**Note**: System entities (is_system=true) cannot be deleted.

## Complete Workflow Script

Here's a complete bash script to create a working CRM structure:

```bash
#!/bin/bash

BASE_URL="http://localhost:8001/api/v1"

# Create Contact entity
CONTACT_RESPONSE=$(curl -s -X POST $BASE_URL/entities \
  -H "Content-Type: application/json" \
  -d '{
    "system_name": "contact",
    "display_name": "Контакт",
    "display_name_plural": "Контакти",
    "description": "Фізична особа"
  }')

CONTACT_ID=$(echo $CONTACT_RESPONSE | jq -r '.id')
echo "Contact Entity ID: $CONTACT_ID"

# Add contact attributes
curl -s -X POST $BASE_URL/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$CONTACT_ID\",
    \"system_name\": \"first_name\",
    \"display_name\": \"Ім'я\",
    \"data_type\": \"string\",
    \"is_required\": true
  }" > /dev/null

curl -s -X POST $BASE_URL/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$CONTACT_ID\",
    \"system_name\": \"email\",
    \"display_name\": \"Email\",
    \"data_type\": \"string\",
    \"is_required\": true,
    \"is_unique\": true
  }" > /dev/null

# Generate schema
curl -s -X POST $BASE_URL/entities/$CONTACT_ID/schema | jq

echo "✅ Contact entity created and schema generated!"
```

## Testing with HTTPie (Alternative to curl)

If you prefer HTTPie:

```bash
# Install httpie
pip install httpie

# Create entity
http POST localhost:8001/api/v1/entities \
  system_name=contact \
  display_name=Контакт \
  display_name_plural=Контакти

# Create attribute
http POST localhost:8001/api/v1/attributes \
  entity_id=<uuid> \
  system_name=first_name \
  display_name="Ім'я" \
  data_type=string \
  is_required:=true
```

## Next Steps (MVP 2)

In MVP 2, you'll be able to:
- Perform CRUD operations on dynamic entities via auto-generated endpoints
- Use GraphQL to query entity data with flexible schemas
- Authenticate with JWT tokens

Example (coming in MVP 2):
```bash
# Create a contact record
curl -X POST http://localhost:8000/api/v1/data/contact \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <jwt-token>" \
  -d '{
    "first_name": "Іван",
    "last_name": "Петренко",
    "email": "ivan@example.com"
  }'
```

---

**Happy Building! 🚀**
