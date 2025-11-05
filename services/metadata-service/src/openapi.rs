use utoipa::OpenApi;

/// OpenAPI документація для Metadata Service
#[derive(OpenApi)]
#[openapi(
    info(
        title = "CoreCRM-R Metadata Service API",
        version = "0.1.0",
        description = "API для управління метаданими, сутностями, атрибутами та workflow автоматизацією.\n\n\
        ## Основні можливості:\n\
        - **Управління сутностями**: Створення та конфігурація динамічних сутностей\n\
        - **Управління атрибутами**: Типізовані поля для сутностей\n\
        - **Генерація схем**: Автоматичне створення таблиць PostgreSQL\n\
        - **BPM Engine**: Повноцінний workflow engine з підтримкою:\n\
          - 11 типів логічних блоків (Start, End, Task, Decision, Fork, Join, Assignment, Loop, ApiCall, Notification, Wait)\n\
          - Обов'язкова перевірка типів змінних\n\
          - Обов'язкове виявлення безкінечних циклів\n\
          - Валідація workflow перед виконанням\n\
          - Виконання workflow з відстеженням стану",
        contact(
            name = "CoreCRM-R Team",
            email = "support@corecrm-r.example.com"
        ),
        license(
            name = "MIT",
        )
    ),
    // ПРИМІТКА: paths() та schemas() потрібно додати після анотації handlers
    // Дивіться SWAGGER_SETUP.md для прикладів
    //
    // Зараз Swagger UI покаже тільки базову інформацію про API
    // Для повної документації додайте:
    // 1. #[derive(ToSchema)] до моделей у shared/models/
    // 2. #[utoipa::path(...)] до handlers
    // 3. Додайте їх сюди у paths() та components(schemas())
    paths(),
    components(schemas()),
    tags(
        (name = "Workflows", description = "Управління workflow (бізнес-процесами)"),
        (name = "Nodes", description = "Управління вузлами workflow (логічними блоками)"),
        (name = "Edges", description = "Управління з'єднаннями між вузлами"),
        (name = "Validation", description = "Валідація та виконання workflow"),
        (name = "Entities", description = "Управління сутностями системи"),
        (name = "Attributes", description = "Управління атрибутами сутностей"),
        (name = "Schema", description = "Генерація схем баз даних"),
    )
)]
pub struct ApiDoc;

/// Swagger UI конфігурація
pub fn swagger_config() -> utoipa_swagger_ui::Config<'static> {
    utoipa_swagger_ui::Config::new(["/api-docs/openapi.json"])
        .try_it_out_enabled(true)
        .filter(true)
        .persist_authorization(true)
}
