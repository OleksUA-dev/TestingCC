# Swagger UI для CoreCRM-R 📚

## Що це?

**Swagger UI** - це інтерактивна документація вашого API. Як Postman, але вбудований прямо у ваш сервіс.

## Як запустити?

```bash
cd services/metadata-service
cargo run
```

Відкрити в браузері:
```
http://localhost:8001/swagger-ui/
```

## Що я побачу?

У Swagger UI ви побачите **всі ваші API endpoint'и**:

### 📋 Workflows (Бізнес-процеси):
- `POST /api/v1/workflows` - Створити workflow
- `GET /api/v1/workflows` - Список workflows
- `POST /api/v1/workflows/{id}/validate` - Перевірити workflow
- `POST /api/v1/workflows/{id}/execute` - Запустити workflow
- і ще 12 інших...

### 📦 Entities (Сутності):
- `POST /api/v1/entities` - Створити сутність
- `GET /api/v1/entities` - Список сутностей
- та інші...

### 🏷️ Attributes (Атрибути):
- `POST /api/v1/attributes` - Додати атрибут
- та інші...

## Як тестувати API?

1. Відкрийте Swagger UI
2. Оберіть endpoint (наприклад, `POST /api/v1/workflows`)
3. Натисніть **"Try it out"**
4. Заповніть дані (Swagger покаже приклади)
5. Натисніть **"Execute"**
6. Побачите відповідь від сервера

## Приклад використання:

### Створити workflow:

1. Знайдіть `POST /api/v1/workflows`
2. Натисніть "Try it out"
3. Введіть JSON:
```json
{
  "name": "Мій перший workflow",
  "description": "Тестовий процес",
  "trigger_type": "manual"
}
```
4. Натисніть "Execute"
5. Отримаєте UUID нового workflow

### Додати вузол Start:

1. Знайдіть `POST /api/v1/workflows/nodes`
2. Введіть:
```json
{
  "workflow_id": "ваш-uuid-з-попереднього-кроку",
  "node_type": "start",
  "name": "Початок",
  "position_x": 0,
  "position_y": 0,
  "config": {}
}
```

### Перевірити workflow:

1. `POST /api/v1/workflows/{workflow_id}/validate`
2. Замість `{workflow_id}` вставте ваш UUID
3. Побачите результат валідації з помилками (якщо є)

## Чим це краще Postman?

✅ **Завжди актуальне** - документація генерується з коду
✅ **Вбудоване** - не потрібно нічого встановлювати
✅ **Приклади** - автоматично показує структуру запитів
✅ **Типи даних** - видно які поля обов'язкові, їх типи
✅ **Швидко** - одразу бачите всі endpoints

## Що ще можна?

- **Фільтрація**: шукайте endpoints по назві
- **Схеми**: дивіться структуру всіх моделей (Workflow, Node, Edge, тощо)
- **Теги**: endpoints згруповані по категоріях

## Це працює як в .NET / Java / Python?

Так! Це **OpenAPI стандарт** (колишній Swagger), який використовується скрізь:
- .NET: Swashbuckle
- Java: SpringDoc
- Python: FastAPI
- Rust: utoipa (те що ми використовуємо)

## Технічні деталі:

У Rust це працює через **compile-time макроси**:

```rust
// Анотуємо структуру
#[derive(ToSchema)]
pub struct Workflow {
    pub id: Uuid,
    pub name: String,
    // ...
}

// Анотуємо функцію-handler
#[utoipa::path(
    post,
    path = "/api/v1/workflows",
    responses(
        (status = 201, body = Workflow)
    )
)]
pub async fn create_workflow(...) {
    // ...
}
```

Rust перевіряє це **під час компіляції** - якщо документація не відповідає коду, просто не скомпілюється! Це значить документація **завжди правильна**. 🎯

## Питання?

Дивіться повну документацію у `SWAGGER_SETUP.md` 📖
