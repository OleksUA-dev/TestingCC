#!/bin/bash

# CoreCRM-R MVP 2 Demo Script
# This script demonstrates the complete workflow of MVP 2

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
METADATA_URL="http://localhost:8001"
GATEWAY_URL="http://localhost:8000"

echo -e "${BLUE}╔═══════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   CoreCRM-R MVP 2 Demo Script        ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════╝${NC}"
echo ""

# Check if services are running
echo -e "${YELLOW}→ Checking if services are running...${NC}"
if ! curl -s $METADATA_URL/health > /dev/null; then
    echo "❌ Metadata Service is not running on $METADATA_URL"
    echo "   Start it with: docker-compose up -d"
    exit 1
fi

if ! curl -s $GATEWAY_URL/health > /dev/null; then
    echo "❌ API Gateway is not running on $GATEWAY_URL"
    echo "   Start it with: docker-compose up -d"
    exit 1
fi

echo -e "${GREEN}✓ Services are running${NC}"
echo ""

# Step 1: Create Entity
echo -e "${BLUE}═══ Step 1: Creating 'contact' entity ===${NC}"
ENTITY_RESPONSE=$(curl -s -X POST $METADATA_URL/api/v1/entities \
  -H "Content-Type: application/json" \
  -d '{
    "system_name": "contact",
    "display_name": "Контакт",
    "display_name_plural": "Контакти",
    "description": "Фізична особа - клієнт або партнер"
  }')

ENTITY_ID=$(echo $ENTITY_RESPONSE | jq -r '.id')
echo -e "${GREEN}✓ Entity created with ID: $ENTITY_ID${NC}"
echo ""

# Step 2: Add Attributes
echo -e "${BLUE}═══ Step 2: Adding attributes ===${NC}"

echo "  → Adding 'first_name' attribute..."
curl -s -X POST $METADATA_URL/api/v1/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$ENTITY_ID\",
    \"system_name\": \"first_name\",
    \"display_name\": \"Ім'я\",
    \"data_type\": \"string\",
    \"is_required\": true
  }" > /dev/null

echo "  → Adding 'last_name' attribute..."
curl -s -X POST $METADATA_URL/api/v1/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$ENTITY_ID\",
    \"system_name\": \"last_name\",
    \"display_name\": \"Прізвище\",
    \"data_type\": \"string\",
    \"is_required\": true
  }" > /dev/null

echo "  → Adding 'email' attribute..."
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

echo "  → Adding 'phone' attribute..."
curl -s -X POST $METADATA_URL/api/v1/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$ENTITY_ID\",
    \"system_name\": \"phone\",
    \"display_name\": \"Телефон\",
    \"data_type\": \"string\"
  }" > /dev/null

echo -e "${GREEN}✓ 4 attributes created${NC}"
echo ""

# Step 3: Generate Schema
echo -e "${BLUE}═══ Step 3: Generating database schema ===${NC}"
SCHEMA_RESPONSE=$(curl -s -X POST $METADATA_URL/api/v1/entities/$ENTITY_ID/schema)
SCHEMA_SUCCESS=$(echo $SCHEMA_RESPONSE | jq -r '.success')

if [ "$SCHEMA_SUCCESS" = "true" ]; then
    echo -e "${GREEN}✓ Database table 'entity_contact' created successfully${NC}"
else
    echo -e "${YELLOW}⚠ Schema generation result:${NC}"
    echo $SCHEMA_RESPONSE | jq
fi
echo ""

# Step 4: Login
echo -e "${BLUE}═══ Step 4: Authenticating with API Gateway ===${NC}"
LOGIN_RESPONSE=$(curl -s -X POST $GATEWAY_URL/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "password": "admin123"
  }')

TOKEN=$(echo $LOGIN_RESPONSE | jq -r '.access_token')

if [ "$TOKEN" = "null" ] || [ -z "$TOKEN" ]; then
    echo "❌ Login failed"
    echo $LOGIN_RESPONSE | jq
    exit 1
fi

echo -e "${GREEN}✓ Logged in as admin${NC}"
echo "  Access Token: ${TOKEN:0:50}..."
echo ""

# Step 5: Create Contact Records
echo -e "${BLUE}═══ Step 5: Creating contact records ===${NC}"

echo "  → Creating contact: Іван Петренко..."
CONTACT1_RESPONSE=$(curl -s -X POST $GATEWAY_URL/api/v1/data/contact \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "first_name": "Іван",
    "last_name": "Петренко",
    "email": "ivan.petrenko@example.com",
    "phone": "+380501234567"
  }')

CONTACT1_ID=$(echo $CONTACT1_RESPONSE | jq -r '.id')
echo -e "    ${GREEN}✓ Created with ID: $CONTACT1_ID${NC}"

echo "  → Creating contact: Марія Коваленко..."
CONTACT2_RESPONSE=$(curl -s -X POST $GATEWAY_URL/api/v1/data/contact \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "first_name": "Марія",
    "last_name": "Коваленко",
    "email": "maria.kovalenko@example.com",
    "phone": "+380509876543"
  }')

CONTACT2_ID=$(echo $CONTACT2_RESPONSE | jq -r '.id')
echo -e "    ${GREEN}✓ Created with ID: $CONTACT2_ID${NC}"

echo "  → Creating contact: Олександр Шевченко..."
CONTACT3_RESPONSE=$(curl -s -X POST $GATEWAY_URL/api/v1/data/contact \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "first_name": "Олександр",
    "last_name": "Шевченко",
    "email": "oleksandr.shevchenko@example.com"
  }')

CONTACT3_ID=$(echo $CONTACT3_RESPONSE | jq -r '.id')
echo -e "    ${GREEN}✓ Created with ID: $CONTACT3_ID${NC}"
echo ""

# Step 6: List Contacts
echo -e "${BLUE}═══ Step 6: Listing all contacts ===${NC}"
CONTACTS_LIST=$(curl -s $GATEWAY_URL/api/v1/data/contact \
  -H "Authorization: Bearer $TOKEN")

CONTACTS_COUNT=$(echo $CONTACTS_LIST | jq '. | length')
echo -e "${GREEN}✓ Found $CONTACTS_COUNT contacts${NC}"
echo ""
echo "$CONTACTS_LIST" | jq -r '.[] | "  • \(.first_name) \(.last_name) - \(.email)"'
echo ""

# Step 7: Get Single Contact
echo -e "${BLUE}═══ Step 7: Getting contact details ===${NC}"
CONTACT_DETAIL=$(curl -s $GATEWAY_URL/api/v1/data/contact/$CONTACT1_ID \
  -H "Authorization: Bearer $TOKEN")

echo -e "${GREEN}✓ Contact details retrieved${NC}"
echo "$CONTACT_DETAIL" | jq
echo ""

# Step 8: Update Contact
echo -e "${BLUE}═══ Step 8: Updating contact ===${NC}"
echo "  → Updating phone number for Іван Петренко..."
UPDATE_RESPONSE=$(curl -s -X PUT $GATEWAY_URL/api/v1/data/contact/$CONTACT1_ID \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{
    "phone": "+380501111111"
  }')

NEW_PHONE=$(echo $UPDATE_RESPONSE | jq -r '.phone')
echo -e "${GREEN}✓ Phone updated to: $NEW_PHONE${NC}"
echo ""

# Step 9: Delete Contact
echo -e "${BLUE}═══ Step 9: Deleting contact ===${NC}"
echo "  → Deleting Олександр Шевченко..."
curl -s -X DELETE $GATEWAY_URL/api/v1/data/contact/$CONTACT3_ID \
  -H "Authorization: Bearer $TOKEN" > /dev/null

echo -e "${GREEN}✓ Contact deleted${NC}"
echo ""

# Step 10: Verify Deletion
echo -e "${BLUE}═══ Step 10: Verifying final state ===${NC}"
FINAL_CONTACTS=$(curl -s $GATEWAY_URL/api/v1/data/contact \
  -H "Authorization: Bearer $TOKEN")

FINAL_COUNT=$(echo $FINAL_CONTACTS | jq '. | length')
echo -e "${GREEN}✓ Final contact count: $FINAL_COUNT${NC}"
echo ""
echo "$FINAL_CONTACTS" | jq -r '.[] | "  • \(.first_name) \(.last_name) - \(.email) - \(.phone // "N/A")"'
echo ""

# Summary
echo -e "${BLUE}╔═══════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   Demo Completed Successfully! 🎉    ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════╝${NC}"
echo ""
echo -e "${GREEN}What we demonstrated:${NC}"
echo "  ✓ Created entity 'contact' via Metadata Service"
echo "  ✓ Added 4 attributes (first_name, last_name, email, phone)"
echo "  ✓ Generated database schema automatically"
echo "  ✓ Authenticated with JWT token"
echo "  ✓ Created 3 contact records via Dynamic CRUD API"
echo "  ✓ Listed all contacts"
echo "  ✓ Retrieved single contact details"
echo "  ✓ Updated contact information"
echo "  ✓ Deleted contact record"
echo ""
echo -e "${YELLOW}Try it yourself:${NC}"
echo "  • Metadata Service: $METADATA_URL"
echo "  • API Gateway: $GATEWAY_URL"
echo "  • Your JWT Token: ${TOKEN:0:50}..."
echo ""
echo -e "${BLUE}For more information, see MVP2_GUIDE.md${NC}"
