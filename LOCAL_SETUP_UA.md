# 🚀 Як запустити CoreCRM-R локально

## Передумови (що потрібно встановити)

### 1. **Rust** (мова програмування)

**Windows:**
```bash
# Завантажте та запустіть rustup-init.exe
https://rustup.rs/

# Або через winget:
winget install Rustlang.Rustup
```

**macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Перевірте установку:
```bash
rustc --version  # Повинно показати версію (напр. 1.75+)
cargo --version  # Cargo - це як npm/nuget для Rust
```

### 2. **PostgreSQL** (база даних)

**Windows:**
```bash
# Завантажте інсталятор
https://www.postgresql.org/download/windows/

# Або через winget:
winget install PostgreSQL.PostgreSQL
```

**macOS:**
```bash
brew install postgresql@16
brew services start postgresql@16
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt update
sudo apt install postgresql postgresql-contrib
sudo systemctl start postgresql
```

**Docker (найпростіший варіант):**
```bash
docker run --name corecrm-postgres \
  -e POSTGRES_USER=corecrm \
  -e POSTGRES_PASSWORD=corecrm123 \
  -e POSTGRES_DB=metadata_db \
  -p 5432:5432 \
  -d postgres:16
```

### 3. **Git** (якщо ще не встановлено)

```bash
git --version  # Перевірити чи є
```

## Крок 1: Клонувати репозиторій

```bash
# Клонуйте проєкт
git clone https://github.com/OleksUA-dev/TestingCC.git
cd TestingCC

# Переключіться на вашу гілку (якщо потрібно)
git checkout claude/core-crm-rust-setup-011CUpwXLznXuCWD9puk8VP2
```

## Крок 2: Налаштувати базу даних

### Варіант A: PostgreSQL локально

1. **Створіть базу даних:**

```bash
# Підключіться до PostgreSQL
psql -U postgres

# У PostgreSQL консолі:
CREATE USER corecrm WITH PASSWORD 'corecrm123';
CREATE DATABASE metadata_db OWNER corecrm;
CREATE DATABASE api_gateway_db OWNER corecrm;
\q  # Вийти
```

2. **Створіть .env файл для metadata-service:**

```bash
cd services/metadata-service
```

Створіть файл `.env` з таким вмістом:
```env
METADATA__DATABASE__URL=postgresql://corecrm:corecrm123@localhost:5432/metadata_db
METADATA__SERVER__HOST=0.0.0.0
METADATA__SERVER__PORT=8001
```

3. **Створіть .env для api-gateway:**

```bash
cd ../api-gateway
```

Створіть файл `.env`:
```env
GATEWAY__DATABASE__URL=postgresql://corecrm:corecrm123@localhost:5432/api_gateway_db
GATEWAY__SERVER__HOST=0.0.0.0
GATEWAY__SERVER__PORT=8000
GATEWAY__METADATA_SERVICE__URL=http://localhost:8001
GATEWAY__JWT__SECRET=your-super-secret-key-change-in-production
GATEWAY__JWT__EXPIRATION_HOURS=24
```

### Варіант B: Docker Compose (рекомендовано)

У кореневій папці проєкту перевірте чи є `docker-compose.yml`:

```bash
cd TestingCC
ls docker-compose.yml  # Має бути
```

Запустіть все одразу:
```bash
docker-compose up -d
```

Це запустить:
- PostgreSQL для metadata-service (порт 5433)
- PostgreSQL для api-gateway (порт 5434)
- Автоматично створить бази даних

## Крок 3: Встановити залежності (перша компіляція)

Це як `npm install` або `dotnet restore`, але автоматично при першій компіляції:

```bash
cd TestingCC
cargo build
```

⏱️ **Перше збирання займе 5-10 хвилин** - Rust завантажує та компілює всі залежності. Наступні збірки будуть швидкі (10-30 секунд).

Ви побачите:
```
   Compiling serde v1.0.195
   Compiling tokio v1.35.1
   Compiling sqlx v0.7.3
   ...
   Finished dev [unoptimized + debuginfo] target(s) in 8m 32s
```

## Крок 4: Запустити Metadata Service

```bash
cd services/metadata-service

# Запустити сервіс (міграції виконаються автоматично)
cargo run
```

Ви побачите логи:
```
2025-01-04T10:00:00Z INFO metadata_service: Configuration loaded successfully
2025-01-04T10:00:01Z INFO metadata_service: Database connection pool created
2025-01-04T10:00:02Z INFO sqlx::migrate: Applied migration 20240101000001
2025-01-04T10:00:02Z INFO sqlx::migrate: Applied migration 20240104000001
2025-01-04T10:00:02Z INFO sqlx::migrate: Applied migration 20240104000002
2025-01-04T10:00:02Z INFO metadata_service: Database migrations completed
2025-01-04T10:00:02Z INFO metadata_service: Metadata Service listening on 0.0.0.0:8001
```

✅ **Сервіс працює!**

## Крок 5: Відкрити Swagger UI

Відкрийте браузер:

```
http://localhost:8001/swagger-ui/
```

Ви побачите інтерактивну документацію API! 🎉

### Швидкий тест через Swagger UI:

#### Тест 1: Health Check
1. Знайдіть `GET /health`
2. Натисніть "Try it out"
3. Натисніть "Execute"
4. Побачите: `{"status": "healthy"}`

#### Тест 2: Створити Entity
1. Знайдіть `POST /api/v1/entities`
2. Натисніть "Try it out"
3. Вставте JSON:
```json
{
  "system_name": "customer",
  "display_name": "Клієнт",
  "display_name_plural": "Клієнти",
  "description": "Клієнти компанії",
  "icon": "user"
}
```
4. Натисніть "Execute"
5. Отримаєте відповідь зі статусом 201 та UUID створеної сутності

#### Тест 3: Створити Workflow
1. Знайдіть `POST /api/v1/workflows`
2. Натисніть "Try it out"
3. Вставте JSON:
```json
{
  "name": "Мій перший workflow",
  "description": "Тестовий бізнес-процес",
  "trigger_type": "manual"
}
```
4. Натисніть "Execute"
5. Скопіюйте `id` з відповіді

#### Тест 4: Додати вузол Start
1. Знайдіть `POST /api/v1/workflows/nodes`
2. Вставте (замініть YOUR_WORKFLOW_ID):
```json
{
  "workflow_id": "YOUR_WORKFLOW_ID",
  "node_type": "start",
  "name": "Початок",
  "position_x": 0,
  "position_y": 0,
  "config": {}
}
```

#### Тест 5: Додати вузол End
```json
{
  "workflow_id": "YOUR_WORKFLOW_ID",
  "node_type": "end",
  "name": "Кінець",
  "position_x": 200,
  "position_y": 0,
  "config": {}
}
```

#### Тест 6: З'єднати вузли
1. Знайдіть `POST /api/v1/workflows/edges`
2. Вставте (замініть UUID'и):
```json
{
  "workflow_id": "YOUR_WORKFLOW_ID",
  "source_node_id": "START_NODE_ID",
  "target_node_id": "END_NODE_ID"
}
```

#### Тест 7: Валідація workflow
1. `POST /api/v1/workflows/{workflow_id}/validate`
2. Замініть `{workflow_id}` у path
3. Execute
4. Побачите результат валідації:
```json
{
  "is_valid": true,
  "errors": [],
  "warnings": []
}
```

#### Тест 8: Виконати workflow
1. `POST /api/v1/workflows/{workflow_id}/execute`
2. У тілі запиту можна передати контекст:
```json
{
  "customer_name": "Тестовий клієнт",
  "email": "test@example.com"
}
```
3. Отримаєте WorkflowExecution зі статусом виконання

## Крок 6 (Опціонально): Запустити API Gateway

Відкрийте **новий термінал**:

```bash
cd services/api-gateway
cargo run
```

API Gateway запуститься на порту **8000** і матиме:
- JWT аутентифікацію
- Динамічні CRUD операції для сутностей
- Проксі до metadata-service

## Альтернатива: Тестування через curl

Якщо не працює Swagger UI, можна через curl/Postman:

### Health check:
```bash
curl http://localhost:8001/health
```

### Створити entity:
```bash
curl -X POST http://localhost:8001/api/v1/entities \
  -H "Content-Type: application/json" \
  -d '{
    "system_name": "customer",
    "display_name": "Клієнт",
    "display_name_plural": "Клієнти"
  }'
```

### Список entities:
```bash
curl http://localhost:8001/api/v1/entities
```

## Troubleshooting (якщо щось не працює)

### Помилка: "Failed to connect to database"

**Рішення 1:** Перевірте чи працює PostgreSQL:
```bash
# Windows (PowerShell):
Get-Service -Name postgresql*

# Linux/macOS:
sudo systemctl status postgresql
# або для Docker:
docker ps | grep postgres
```

**Рішення 2:** Перевірте правильність DATABASE_URL у `.env`

**Рішення 3:** Перевірте чи існує база даних:
```bash
psql -U corecrm -l  # Список баз даних
```

### Помилка: "Address already in use"

Порт 8001 зайнято іншою програмою.

**Рішення:** Змініть порт у `.env`:
```env
METADATA__SERVER__PORT=8002  # Або інший вільний порт
```

### Помилка компіляції: "linker `cc` not found"

**Windows:** Встановіть Visual Studio Build Tools
```
https://visualstudio.microsoft.com/downloads/
# Оберіть "Build Tools for Visual Studio"
# Виберіть "Desktop development with C++"
```

**Linux:**
```bash
sudo apt install build-essential
```

**macOS:**
```bash
xcode-select --install
```

### Дуже повільна компіляція

Це нормально для першого разу! Rust компілює все з нуля.

**Прискорити (опціонально):**
```bash
# Встановити lld (швидший лінкер)
# Linux:
sudo apt install lld

# macOS:
brew install llvm

# Додати у ~/.cargo/config.toml:
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```

### Сервіс запустився, але Swagger UI показує порожню сторінку

Перевірте чи правильний URL:
```
http://localhost:8001/swagger-ui/
```
(з `/` в кінці)

Або відкрийте напряму OpenAPI JSON:
```
http://localhost:8001/api-docs/openapi.json
```

## Корисні команди Cargo (як npm)

```bash
# Збірка (компіляція)
cargo build              # Debug build
cargo build --release    # Production build (швидший, але довше компілюється)

# Запуск
cargo run               # Збірка + запуск

# Тести
cargo test              # Запустити всі тести

# Перевірка коду (швидко, без компіляції)
cargo check             # Перевірити чи компілюється

# Форматування коду
cargo fmt               # Автоформатування (як prettier)

# Лінтер
cargo clippy            # Перевірка на помилки та best practices

# Очистити build артефакти
cargo clean             # Видалити target/ (звільнити місце)

# Оновити залежності
cargo update            # Оновити до останніх версій
```

## Структура проєкту

```
TestingCC/
├── services/
│   ├── metadata-service/     ← Metadata + Workflow API (порт 8001)
│   │   ├── src/
│   │   │   ├── handlers/     ← API endpoints
│   │   │   ├── service/      ← Бізнес-логіка
│   │   │   ├── repository/   ← База даних
│   │   │   ├── workflow/     ← BPM engine
│   │   │   └── main.rs
│   │   ├── migrations/       ← SQL міграції
│   │   └── Cargo.toml        ← Залежності (як package.json)
│   │
│   └── api-gateway/          ← API Gateway (порт 8000)
│
├── crates/
│   └── shared/               ← Спільні моделі та утиліти
│
├── frontend/
│   └── corecrm-ui/          ← React + TypeScript UI
│
├── Cargo.toml               ← Workspace config
├── docker-compose.yml       ← Docker налаштування
└── README.md
```

## Швидкий старт (TL;DR)

```bash
# 1. Встановити Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Запустити PostgreSQL (Docker)
docker run --name corecrm-postgres \
  -e POSTGRES_USER=corecrm \
  -e POSTGRES_PASSWORD=corecrm123 \
  -e POSTGRES_DB=metadata_db \
  -p 5432:5432 -d postgres:16

# 3. Клонувати та запустити
git clone https://github.com/OleksUA-dev/TestingCC.git
cd TestingCC/services/metadata-service
echo 'METADATA__DATABASE__URL=postgresql://corecrm:corecrm123@localhost:5432/metadata_db' > .env
cargo run

# 4. Відкрити браузер
# http://localhost:8001/swagger-ui/
```

## Що далі?

- 📚 Дивіться **SWAGGER_UA.md** - як використовувати Swagger UI
- 🔧 Дивіться **BPM_ENGINE_IMPLEMENTATION.md** - детальна документація BPM
- 🎨 Запустіть **frontend** - React UI для візуалізації

## Підтримка

Якщо щось не працює - напишіть які помилки бачите! 😊
