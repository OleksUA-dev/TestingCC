# CoreCRM-R MVP 3 Guide

## 🎨 What's New in MVP 3

MVP 3 introduces the **UI Engine** with dynamic form and grid rendering, bringing the full metadata-driven experience to the frontend!

### ✨ Key Features

1. **React Frontend Application** - Modern, responsive UI built with React 18 + TypeScript
2. **Dynamic Form Renderer** - Automatically generates forms based on entity metadata
3. **Dynamic Grid/Table Renderer** - Displays records with sortable, filterable columns
4. **Authentication UI** - Login page with JWT integration
5. **Entity Navigation** - Browse and manage all your custom entities
6. **CRUD Operations** - Full create, read, update, delete functionality via UI
7. **RBAC Foundation** - Basic role-based access control (metadata infrastructure)

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Browser (React)                       │
│  ┌───────────────────────────────────────────────────┐  │
│  │  Dynamic Form Renderer │ Dynamic Grid Renderer    │  │
│  │  - Reads entity metadata                          │  │
│  │  - Generates UI on-the-fly                        │  │
│  │  - Type-aware input components                    │  │
│  └───────────────────────────────────────────────────┘  │
└────────────────────────┬────────────────────────────────┘
                         │ HTTP + JWT
                         │
                         ▼
┌─────────────────────────────────────────────────────────┐
│              API Gateway (Port 8000)                     │
│  - JWT Authentication                                    │
│  - Dynamic CRUD Endpoints                                │
│  - Fetches metadata from Metadata Service               │
└──────────────┬──────────────────────┬───────────────────┘
               │                      │
               ▼                      ▼
┌──────────────────────┐    ┌──────────────────────┐
│  Metadata Service    │    │  User Data (entity_*) │
│  (Port 8001)         │    │                      │
│  - Entities          │    │  PostgreSQL          │
│  - Attributes        │    │  corecrm_gateway     │
│  - UI Layouts (RBAC) │    └──────────────────────┘
│                      │
│  PostgreSQL          │
│  corecrm_metadata    │
└──────────────────────┘
```

---

## 🚀 Quick Start

### Option 1: Docker Compose (Recommended)

```bash
# 1. Start all services (includes frontend)
docker-compose up -d

# 2. Wait for services to be ready (~30 seconds)
docker-compose logs -f frontend

# 3. Open browser
open http://localhost:3000

# 4. Login with default credentials
# Username: admin
# Password: admin123
```

### Option 2: Local Development

```bash
# Terminal 1: Backend services
docker-compose up -d postgres metadata-service api-gateway

# Terminal 2: Frontend development server
cd frontend/corecrm-ui
npm install
npm run dev

# Open browser at http://localhost:3000
```

---

## 📱 User Interface Guide

### 1. Login Page

![Login](docs/images/login.png)

- **URL**: `http://localhost:3000/login`
- **Default credentials**: `admin` / `admin123`
- JWT token is stored in localStorage
- Automatic redirect to entities page on success

### 2. Entity List Page

![Entity List](docs/images/entities.png)

- **URL**: `http://localhost:3000/`
- Shows all active entities from metadata
- Click any entity card to view its records
- Displays entity icon, name, and description

### 3. Entity Data Page (Dynamic Grid + Form)

![Entity Data](docs/images/entity-data.png)

- **URL**: `http://localhost:3000/entities/{entity_name}`
- **Grid View**: Lists all records for the entity
- **Dynamic Columns**: Based on attribute metadata
- **Actions**: Edit, Delete buttons for each record
- **New Button**: Opens form to create new record

### 4. Dynamic Form Modal

![Dynamic Form](docs/images/form.png)

- **Form Fields**: Automatically generated from attributes
- **Data Types Supported**:
  - `string` → Text input
  - `text` → Textarea
  - `integer/float/decimal` → Number input
  - `boolean` → Checkbox
  - `date` → Date picker
  - `date_time` → DateTime picker
  - `enum` → Select dropdown
- **Validation**: Required fields marked with `*`
- **Submit**: Creates or updates record via API

---

## 🎨 Frontend Tech Stack

| Technology | Purpose |
|------------|---------|
| **React 18** | UI library |
| **TypeScript** | Type safety |
| **Vite** | Build tool and dev server |
| **React Router** | Client-side routing |
| **TanStack Query** | Server state management |
| **TanStack Table** | Table/grid rendering |
| **React Hook Form** | Form handling |
| **Zustand** | Client state (auth) |
| **Axios** | HTTP client |
| **Tailwind CSS** | Styling |

---

## 🔑 Key Frontend Components

### DynamicForm Component

**Location**: `src/components/DynamicForm.tsx`

**Purpose**: Renders forms based on entity attributes

**Features**:
- Automatic field type detection
- Required field validation
- Default values support
- Enum dropdowns
- Date/datetime pickers

**Usage**:
```typescript
<DynamicForm
  attributes={attributes}
  initialData={record}
  onSubmit={(data) => createRecord(data)}
  onCancel={() => setShowForm(false)}
/>
```

### DynamicGrid Component

**Location**: `src/components/DynamicGrid.tsx`

**Purpose**: Displays records in a table

**Features**:
- Sortable columns
- Type-aware cell rendering
- Row click handling
- Edit/Delete actions

**Usage**:
```typescript
<DynamicGrid
  attributes={attributes}
  data={records}
  onEdit={(record) => handleEdit(record)}
  onDelete={(record) => handleDelete(record)}
/>
```

### API Client

**Location**: `src/api/client.ts`

**Features**:
- Automatic JWT token injection
- 401 handling (redirect to login)
- Centralized error handling

**Services**:
- `authApi` - Authentication endpoints
- `metadataApi` - Entity/attribute metadata
- `dataApi` - Dynamic CRUD operations

---

## 📊 Complete Workflow Example

### Step 1: Create Entity (via API or metadata service)

```bash
curl -X POST http://localhost:8001/api/v1/entities \
  -H "Content-Type: application/json" \
  -d '{
    "system_name": "product",
    "display_name": "Product",
    "display_name_plural": "Products",
    "description": "Product catalog"
  }'
```

### Step 2: Add Attributes

```bash
ENTITY_ID="<from-step-1>"

# Product Name
curl -X POST http://localhost:8001/api/v1/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$ENTITY_ID\",
    \"system_name\": \"name\",
    \"display_name\": \"Product Name\",
    \"data_type\": \"string\",
    \"is_required\": true
  }"

# Price
curl -X POST http://localhost:8001/api/v1/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$ENTITY_ID\",
    \"system_name\": \"price\",
    \"display_name\": \"Price\",
    \"data_type\": \"decimal\",
    \"is_required\": true
  }"

# In Stock
curl -X POST http://localhost:8001/api/v1/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$ENTITY_ID\",
    \"system_name\": \"in_stock\",
    \"display_name\": \"In Stock\",
    \"data_type\": \"boolean\"
  }"
```

### Step 3: Generate Schema

```bash
curl -X POST http://localhost:8001/api/v1/entities/$ENTITY_ID/schema
```

### Step 4: Use the UI

1. **Open browser**: `http://localhost:3000`
2. **Login**: admin / admin123
3. **Click "Products"** card on entity list page
4. **Click "+ New Product"** button
5. **Fill form**:
   - Product Name: "Laptop"
   - Price: 999.99
   - In Stock: ✓
6. **Click "Save"**
7. **See record** appear in grid!

---

## 🛠️ Development

### Running Frontend Locally

```bash
cd frontend/corecrm-ui

# Install dependencies
npm install

# Start dev server (with hot reload)
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

### Environment Configuration

Create `frontend/corecrm-ui/.env`:

```bash
VITE_API_BASE_URL=http://localhost:8000
```

### Code Structure

```
frontend/corecrm-ui/
├── src/
│   ├── api/              # API clients
│   │   ├── client.ts    # Base Axios client
│   │   ├── auth.ts      # Authentication API
│   │   ├── metadata.ts  # Metadata API
│   │   └── data.ts      # Dynamic CRUD API
│   ├── components/      # React components
│   │   ├── DynamicForm.tsx
│   │   └── DynamicGrid.tsx
│   ├── pages/           # Page components
│   │   ├── LoginPage.tsx
│   │   ├── EntityListPage.tsx
│   │   └── EntityDataPage.tsx
│   ├── stores/          # State management
│   │   └── authStore.ts
│   ├── types/           # TypeScript types
│   │   └── index.ts
│   ├── App.tsx          # Main app component
│   ├── main.tsx         # Entry point
│   └── index.css        # Tailwind CSS
├── index.html
├── package.json
├── vite.config.ts
├── tailwind.config.js
├── Dockerfile
└── nginx.conf
```

---

## 🔐 Authentication Flow

1. **User enters credentials** on `/login`
2. **POST** to `/api/v1/auth/login`
3. **Receive JWT token** and user info
4. **Store in localStorage**: `access_token`, `user`
5. **Axios interceptor** adds token to all requests:
   ```typescript
   headers: { Authorization: `Bearer ${token}` }
   ```
6. **401 response** → Redirect to `/login`

---

## 🎨 UI Metadata (RBAC Foundation)

MVP 3 includes database schema for UI metadata and RBAC:

### New Tables (corecrm_metadata)

#### `roles`
```sql
id, name, description, is_system, created_at, updated_at
```

Default roles: Administrator, User, Guest

#### `entity_permissions`
```sql
id, role_id, entity_id, can_create, can_read, can_update, can_delete
```

#### `user_roles`
```sql
user_id, role_id, assigned_at
```

#### `form_layouts`
```sql
id, entity_id, name, is_default, sections (JSONB), created_at, updated_at
```

#### `grid_layouts`
```sql
id, entity_id, name, is_default, columns (JSONB), default_sort_by, page_size
```

**Note**: UI metadata API endpoints and form/grid designers are planned for future releases.

---

## 🚧 Limitations & Future Enhancements

### Current Limitations (MVP 3)

- ❌ No form designer UI (forms are auto-generated)
- ❌ No grid designer UI (grids are auto-generated)
- ❌ No field-level permissions (only entity-level RBAC schema)
- ❌ No filtering/search in grids
- ❌ No pagination controls (returns all records)
- ❌ No Lookup field UI (displays UUID)
- ❌ No many-to-many relationship UI

### Planned for Future

- 🔜 **Form Designer**: Drag-and-drop form builder
- 🔜 **Grid Designer**: Configure columns, sorting, filters
- 🔜 **Advanced RBAC**: Field-level permissions, conditional access
- 🔜 **Lookup Fields**: Dropdown with entity records
- 🔜 **Advanced Filtering**: Search, filter, sort in grids
- 🔜 **Pagination**: Navigate large datasets
- 🔜 **Dashboard**: Customizable widgets
- 🔜 **Audit Log Viewer**: Track changes

---

## 🐛 Troubleshooting

### Frontend won't start

```bash
# Check if ports are available
lsof -i :3000

# Rebuild Docker image
docker-compose build frontend
docker-compose up -d frontend
```

### "Failed to load entities"

- Check metadata service is running: `curl http://localhost:8001/health`
- Check network connectivity in Docker
- View logs: `docker-compose logs metadata-service`

### Authentication errors

- Check API Gateway is running: `curl http://localhost:8000/health`
- Clear localStorage: Browser DevTools → Application → Local Storage → Clear
- Check JWT secret is configured in `.env`

### Form validation errors

- Ensure required attributes are marked correctly in metadata
- Check data types match input values
- Verify attribute is_active = true

### CORS errors

- Backend has CORS enabled for all origins (development)
- For production, configure `CorsLayer` in `services/api-gateway/src/routes.rs`

---

## 📈 Performance Tips

### Frontend

- React Query caches API responses
- Lazy load entity attributes only when needed
- Use React.memo for expensive components

### Backend

- Metadata is fetched per-request (consider caching for production)
- Use pagination for large datasets
- Add database indexes for frequently queried fields

---

## 🧪 Testing

### Manual Testing Checklist

- [ ] Login with admin/admin123
- [ ] View entity list
- [ ] Click entity to view records
- [ ] Create new record
- [ ] Edit existing record
- [ ] Delete record
- [ ] Logout and verify redirect to login

### Automated Tests (Future)

```bash
# Frontend unit tests
cd frontend/corecrm-ui
npm run test

# E2E tests with Playwright
npm run test:e2e
```

---

## 🎓 Learning Resources

### React + TypeScript
- https://react.dev/
- https://www.typescriptlang.org/docs/

### TanStack Query
- https://tanstack.com/query/latest

### TanStack Table
- https://tanstack.com/table/latest

### Tailwind CSS
- https://tailwindcss.com/docs

---

## 📝 Summary

MVP 3 completes the **metadata-driven UI** vision:

✅ **Backend** (MVP 1): Metadata management, schema generation
✅ **API** (MVP 2): Dynamic CRUD, JWT auth
✅ **Frontend** (MVP 3): Dynamic forms, grids, SPA

The platform is now fully functional for building custom CRM applications with zero hardcoded entities!

---

**Next**: Enhance with form designers, advanced RBAC, and business process automation (BPMN).

**Happy Building! 🚀**
