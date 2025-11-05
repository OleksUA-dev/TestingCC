# Swagger UI налаштування для CoreCRM-R

## ✅ Що вже зроблено:

1. **Додано залежності**:
   - `utoipa` - для генерації OpenAPI специфікації
   - `utoipa-swagger-ui` - інтерактивний Swagger UI

2. **Створено OpenAPI конфігурацію** (`services/metadata-service/src/openapi.rs`):
   - Опис API
   - Теги для групування endpoint'ів
   - Список всіх схем та endpoint'ів

3. **Додано Swagger UI route** (`/swagger-ui/`):
   - Інтерактивна документація API
   - Можливість тестувати endpoint'и прямо з браузера

## 🚀 Як використовувати:

### Запуск сервісу:
```bash
cd services/metadata-service
cargo run
```

### Відкрити Swagger UI:
```
http://localhost:8001/swagger-ui/
```

### OpenAPI JSON:
```
http://localhost:8001/api-docs/openapi.json
```

## 📝 Як додавати документацію до endpoints:

### 1. Анотація моделей (Rust structs):

```rust
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Workflow {
    /// Унікальний ідентифікатор workflow
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,

    /// Назва workflow
    #[schema(example = "Customer Onboarding")]
    pub name: String,

    /// Опис workflow
    #[schema(example = "Автоматизований процес онбордингу клієнтів")]
    pub description: Option<String>,

    // ... інші поля
}
```

### 2. Анотація endpoints (handlers):

```rust
use utoipa::path;

/// Створити новий workflow
#[utoipa::path(
    post,
    path = "/api/v1/workflows",
    tag = "Workflows",
    request_body = CreateWorkflowRequest,
    responses(
        (status = 201, description = "Workflow успішно створено", body = Workflow),
        (status = 400, description = "Невалідні дані запиту"),
        (status = 500, description = "Внутрішня помилка сервера")
    )
)]
pub async fn create_workflow(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateWorkflowRequest>,
) -> Result<impl IntoResponse, AppError> {
    // ... implementation
}
```

### 3. Анотація enum'ів:

```rust
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
    /// Початковий вузол workflow
    Start,
    /// Кінцевий вузол workflow
    End,
    /// Виконання задачі
    Task,
    /// Умовне розгалуження
    Decision,
    // ... інші варіанти
}
```

## 🎯 Поточний стан:

### ✅ Готово:
- Swagger UI доступний на `/swagger-ui/`
- OpenAPI схема генерується автоматично
- Базова структура документації створена

### 🔄 Потрібно додати (опціонально, для повної документації):

1. **Додати `#[derive(ToSchema)]` до моделей**:
   - `crates/shared/src/models/workflow.rs` - всі workflow моделі
   - `crates/shared/src/models/entity.rs` - Entity моделі
   - `crates/shared/src/models/attribute.rs` - Attribute моделі

2. **Додати `#[utoipa::path(...)]` до handlers**:
   - `services/metadata-service/src/handlers/workflow.rs` - 16 endpoints
   - `services/metadata-service/src/handlers/entity.rs` - 5 endpoints
   - `services/metadata-service/src/handlers/attribute.rs` - 5 endpoints

## 🌟 Переваги Swagger UI:

1. **Інтерактивна документація**:
   - Можна тестувати API прямо з браузера
   - Автоматична генерація прикладів запитів

2. **Актуальність**:
   - Документація генерується з коду
   - Завжди синхронізована з реальним API

3. **Типобезпека**:
   - Compile-time перевірка відповідності документації та коду
   - Неможливо створити неправильну документацію

4. **OpenAPI стандарт**:
   - Можна використовувати з будь-якими OpenAPI інструментами
   - Генерація клієнтів для різних мов програмування

## 📖 Приклади використання в Swagger UI:

### Створення workflow:

```json
POST /api/v1/workflows
{
  "name": "Customer Onboarding",
  "description": "Automated customer onboarding process",
  "trigger_type": "manual"
}
```

### Додавання вузла:

```json
POST /api/v1/workflows/nodes
{
  "workflow_id": "550e8400-e29b-41d4-a716-446655440000",
  "node_type": "task",
  "name": "Send Welcome Email",
  "position_x": 100,
  "position_y": 50,
  "config": {
    "task_name": "send_email",
    "email_template": "welcome"
  }
}
```

### Валідація workflow:

```
POST /api/v1/workflows/{workflow_id}/validate
```

Поверне ValidationResult з детальною інформацією про помилки:
```json
{
  "is_valid": false,
  "errors": [
    {
      "error_type": "missing_start_node",
      "message": "Workflow must have at least one Start node",
      "node_id": null
    }
  ],
  "warnings": []
}
```

## 🔧 Налаштування Swagger UI:

В `openapi.rs` можна налаштувати:

```rust
pub fn swagger_config() -> utoipa_swagger_ui::Config<'static> {
    utoipa_swagger_ui::Config::new(["/api-docs/openapi.json"])
        .try_it_out_enabled(true)    // Дозволити тестування endpoint'ів
        .filter(true)                // Пошук по endpoint'ах
        .persist_authorization(true) // Зберігати токени авторизації
}
```

## 📚 Додаткові ресурси:

- [utoipa документація](https://docs.rs/utoipa/)
- [utoipa examples](https://github.com/juhaku/utoipa/tree/master/examples)
- [OpenAPI Specification](https://swagger.io/specification/)

## 💡 Поради:

1. **Для production**: додайте повні анотації до всіх endpoints
2. **Для development**: поточної версії достатньо для базової документації
3. **Безпека**: у production можна додати auth middleware для `/swagger-ui/`
4. **Кастомізація**: можна змінити стиль Swagger UI через CSS

## 🎨 Що бачить користувач:

Swagger UI відображає:
- ✅ Список всіх endpoint'ів згрупованих по тегах (Workflows, Entities, Attributes, тощо)
- ✅ HTTP методи (GET, POST, PUT, DELETE)
- ✅ URL paths з параметрами
- ✅ Request/Response схеми
- ✅ Можливість виконати запит прямо з UI
- ✅ Приклади даних для кожної схеми

Це як Postman, але вбудований в ваш API і завжди актуальний! 🚀
