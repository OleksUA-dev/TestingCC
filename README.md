# CoreCRM-R: High-Performance CRM Platform on Rust

**CoreCRM-R** is a metadata-driven, high-performance CRM platform built with Rust, designed for rapid deployment of customized business applications with minimal development overhead.

## 🎯 Project Goals

- **Performance**: < 50ms p99 latency for CRUD operations, 10,000+ RPS throughput
- **Reliability**: Memory-safe, crash-resistant, high availability ready
- **Flexibility**: Metadata-driven architecture for rapid customization
- **Scalability**: Horizontal and vertical scaling support

## 🏗️ Architecture

### Microservices Structure

```
CoreCRM-R/
├── services/
│   ├── metadata-service/     # Core metadata management engine
│   └── api-gateway/           # API routing and auto-generation (MVP 2)
├── crates/
│   └── shared/                # Common types and utilities
└── docker-compose.yml         # Development environment
```

### Technology Stack

| Component | Technology | Rationale |
|-----------|-----------|-----------|
| Language | Rust (stable) | Performance, memory safety, reliability |
| Web Framework | Axum | High performance, mature ecosystem |
| Async Runtime | Tokio | Industry standard for async Rust |
| Database Driver | SQLx | Async, type-safe, compile-time checks |
| Database | PostgreSQL | Reliability, JSONB support, scalability |
| API | REST (GraphQL in MVP 2) | Flexibility for metadata-driven systems |
| Frontend | TBD (React/Vue/Svelte) | Decoupled from backend |
| Containerization | Docker, Kubernetes | HA and scaling support |

## 🚀 Quick Start

### Prerequisites

- **Rust** 1.75+ ([Install Rust](https://rustup.rs/))
- **Docker** and **Docker Compose** ([Install Docker](https://docs.docker.com/get-docker/))
- **PostgreSQL** 14+ (or use Docker Compose)

### Development Setup

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd TestingCC
   ```

2. **Copy environment configuration**:
   ```bash
   cp .env.example .env
   ```

3. **Start PostgreSQL with Docker Compose**:
   ```bash
   docker-compose up -d postgres
   ```

4. **Run database migrations**:
   ```bash
   cd services/metadata-service
   cargo install sqlx-cli --no-default-features --features postgres
   sqlx migrate run
   ```

5. **Start the metadata service**:
   ```bash
   cargo run --package metadata-service
   ```

6. **Verify the service is running**:
   ```bash
   curl http://localhost:8001/health
   ```

## 📚 API Documentation

### Metadata Service (Port 8001)

#### Health Check
```bash
GET /health
```

#### Entity Management

**Create Entity**:
```bash
POST /api/v1/entities
Content-Type: application/json

{
  "system_name": "contact",
  "display_name": "Контакт",
  "display_name_plural": "Контакти",
  "description": "Фізична особа",
  "icon": "user"
}
```

**List Entities**:
```bash
GET /api/v1/entities
```

**Get Entity**:
```bash
GET /api/v1/entities/{id}
```

**Update Entity**:
```bash
PUT /api/v1/entities/{id}
Content-Type: application/json

{
  "display_name": "Контакт (оновлено)",
  "is_active": true
}
```

**Delete Entity**:
```bash
DELETE /api/v1/entities/{id}
```

#### Attribute Management

**Create Attribute**:
```bash
POST /api/v1/attributes
Content-Type: application/json

{
  "entity_id": "uuid-of-entity",
  "system_name": "first_name",
  "display_name": "Ім'я",
  "data_type": "string",
  "is_required": true,
  "is_unique": false
}
```

**List Attributes for Entity**:
```bash
GET /api/v1/entities/{entity_id}/attributes
```

**Update Attribute**:
```bash
PUT /api/v1/attributes/{id}
```

**Delete Attribute**:
```bash
DELETE /api/v1/attributes/{id}
```

#### Schema Generation

**Generate Database Schema for Entity**:
```bash
POST /api/v1/entities/{id}/schema
```

This endpoint automatically creates a PostgreSQL table based on the entity's metadata.

### Supported Data Types

| Data Type | PostgreSQL Type | Description |
|-----------|----------------|-------------|
| `string` | VARCHAR(255) | Short text field |
| `text` | TEXT | Long text field |
| `integer` | BIGINT | Integer number |
| `float` | DOUBLE PRECISION | Floating point number |
| `decimal` | NUMERIC(19,4) | Precise decimal (for financial data) |
| `boolean` | BOOLEAN | True/False |
| `date_time` | TIMESTAMPTZ | Date and time with timezone |
| `date` | DATE | Date only |
| `lookup` | UUID | Reference to another entity |
| `enum` | VARCHAR(255) | Enumeration of values |

## 🧪 Testing

### Run Unit Tests
```bash
cargo test
```

### Run Integration Tests
```bash
cargo test --package metadata-service -- --ignored
```

### Manual API Testing

Example workflow:

1. **Create an entity** (Contact):
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

2. **Create attributes** (first_name, email):
```bash
# Get entity_id from previous response
ENTITY_ID="<uuid-from-previous-response>"

curl -X POST http://localhost:8001/api/v1/attributes \
  -H "Content-Type: application/json" \
  -d "{
    \"entity_id\": \"$ENTITY_ID\",
    \"system_name\": \"first_name\",
    \"display_name\": \"Ім'я\",
    \"data_type\": \"string\",
    \"is_required\": true
  }"

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

3. **Generate database schema**:
```bash
curl -X POST http://localhost:8001/api/v1/entities/$ENTITY_ID/schema
```

4. **Verify table creation**:
```bash
docker exec -it corecrm-postgres psql -U corecrm -d corecrm_metadata -c "\d entity_contact"
```

## 🐳 Docker Deployment

### Build and Run with Docker Compose

```bash
docker-compose up --build
```

This starts:
- PostgreSQL on port 5432
- Metadata Service on port 8001

### Production Deployment

For production, consider:
- Using Kubernetes for orchestration
- Implementing health checks and liveness probes
- Setting up monitoring (Prometheus + Grafana)
- Configuring proper secrets management
- Setting up reverse proxy (nginx/Traefik)

## 🗺️ Roadmap

### ✅ MVP 1: Metadata Core (Current)
- [x] Entity CRUD via API
- [x] Attribute CRUD via API
- [x] Automatic PostgreSQL table generation
- [x] Data type support (String, Integer, Lookup, etc.)
- [x] Database migrations
- [x] Docker deployment

### 🚧 MVP 2: API Gateway + Dynamic CRUD
- [ ] Auto-generated REST endpoints for custom entities
- [ ] GraphQL API support
- [ ] Dynamic CRUD operations on custom entities
- [ ] Basic authentication (JWT)
- [ ] API documentation (OpenAPI/Swagger)

### 📋 MVP 3: Basic UI Engine
- [ ] React/Vue/Svelte frontend application
- [ ] Dynamic form rendering based on metadata
- [ ] Dynamic grid/list rendering
- [ ] Basic RBAC (roles and entity-level permissions)

### 🎯 MVP 4: Core CRM Configuration
- [ ] Pre-configured core entities (Contact, Account, Activity, Opportunity)
- [ ] Standard CRM workflows
- [ ] Release v1.0

### 🔮 Future Enhancements (Out of MVP Scope)
- Business Process Engine (BPMN)
- Dashboard builder
- Integration framework (Webhooks, Message Queues)
- Advanced RBAC (field-level security)
- Multi-tenancy support
- Real-time collaboration features

## 🏛️ Architecture Principles

### 1. Metadata-Driven
All system logic (data structure, UI, access rules) is based on metadata stored in the database. The Rust core interprets this metadata on-the-fly.

### 2. API-First
Every entity or business logic created through low-code tools is automatically available via a secured API.

### 3. Stateless Services
All backend services are stateless for horizontal scaling and high availability (critical for web farm environments).

### 4. Asynchronous Core
The entire system uses async/await (Tokio runtime) to handle thousands of concurrent connections efficiently.

## 📊 Performance Targets

- **Latency**: p99 < 50ms for basic CRUD operations (excluding network)
- **Throughput**: 10,000+ RPS on standard cloud instance (4 vCPU, 8GB RAM)
- **Memory Safety**: Guaranteed by Rust (no unsafe blocks without audit)
- **High Availability**: Ready for clustered deployment (Kubernetes, web farm)

## 🤝 Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 📞 Support

For questions and support:
- Open an issue on GitHub
- Contact the development team

---

**Built with ❤️ and Rust 🦀**
