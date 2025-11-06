# CoreCRM-R Quick Start Script для Windows
# Запускати у PowerShell

Write-Host "=== CoreCRM-R Quick Start Script ===" -ForegroundColor Green
Write-Host ""

# Перевірка Rust
Write-Host "[1/6] Перевірка Rust..." -ForegroundColor Yellow
try {
    $rustVersion = cargo --version
    Write-Host "✅ Rust встановлено: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Rust не встановлено!" -ForegroundColor Red
    Write-Host "Встановіть Rust: https://rustup.rs/"
    Write-Host "Або виконайте: winget install Rustlang.Rustup"
    exit 1
}
Write-Host ""

# Перевірка Docker
Write-Host "[2/6] Перевірка PostgreSQL..." -ForegroundColor Yellow
try {
    docker --version | Out-Null
    Write-Host "Docker знайдено. Запускаємо PostgreSQL в Docker..." -ForegroundColor White

    # Перевірити чи контейнер існує
    $containerExists = docker ps -a --filter "name=corecrm-postgres" --format "{{.Names}}"

    if ($containerExists) {
        Write-Host "Контейнер corecrm-postgres вже існує. Запускаємо..." -ForegroundColor White
        docker start corecrm-postgres
    } else {
        Write-Host "Створюємо новий контейнер PostgreSQL..." -ForegroundColor White
        docker run --name corecrm-postgres `
          -e POSTGRES_USER=corecrm `
          -e POSTGRES_PASSWORD=corecrm123 `
          -e POSTGRES_DB=metadata_db `
          -p 5432:5432 `
          -d postgres:16
    }

    Write-Host "✅ PostgreSQL запущено в Docker" -ForegroundColor Green
    Start-Sleep -Seconds 3
} catch {
    Write-Host "⚠️  Docker не знайдено. Переконайтеся що PostgreSQL встановлено локально" -ForegroundColor Yellow
}
Write-Host ""

# Створення .env файлів
Write-Host "[3/6] Створення .env файлів..." -ForegroundColor Yellow

# .env для metadata-service
$metadataEnv = @"
METADATA__DATABASE__URL=postgresql://corecrm:corecrm123@localhost:5432/metadata_db
METADATA__SERVER__HOST=0.0.0.0
METADATA__SERVER__PORT=8001
"@

Set-Content -Path "services\metadata-service\.env" -Value $metadataEnv
Write-Host "✅ Створено services\metadata-service\.env" -ForegroundColor Green

# .env для api-gateway
if (Test-Path "services\api-gateway") {
    $gatewayEnv = @"
GATEWAY__DATABASE__URL=postgresql://corecrm:corecrm123@localhost:5432/api_gateway_db
GATEWAY__SERVER__HOST=0.0.0.0
GATEWAY__SERVER__PORT=8000
GATEWAY__METADATA_SERVICE__URL=http://localhost:8001
GATEWAY__JWT__SECRET=your-super-secret-key-change-in-production
GATEWAY__JWT__EXPIRATION_HOURS=24
"@
    Set-Content -Path "services\api-gateway\.env" -Value $gatewayEnv
    Write-Host "✅ Створено services\api-gateway\.env" -ForegroundColor Green
}
Write-Host ""

# Збірка проєкту
Write-Host "[4/6] Збірка проєкту (перший раз займе 5-10 хвилин)..." -ForegroundColor Yellow
cargo build
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Помилка компіляції" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Проєкт зібрано успішно" -ForegroundColor Green
Write-Host ""

# Підказки
Write-Host "[5/6] Готово до запуску!" -ForegroundColor Yellow
Write-Host ""
Write-Host "Для запуску metadata-service:" -ForegroundColor Green
Write-Host "  cd services\metadata-service"
Write-Host "  cargo run"
Write-Host ""
Write-Host "Після запуску відкрийте в браузері:" -ForegroundColor Green
Write-Host "  http://localhost:8001/swagger-ui/" -ForegroundColor Yellow
Write-Host ""
Write-Host "Або для швидкого тесту:" -ForegroundColor Green
Write-Host "  curl http://localhost:8001/health"
Write-Host ""

# Запит на запуск
Write-Host "[6/6] Запустити metadata-service зараз? (y/n)" -ForegroundColor Yellow
$response = Read-Host

if ($response -match '^[Yy]') {
    Write-Host "Запускаю metadata-service..." -ForegroundColor Green
    Write-Host ""
    Set-Location services\metadata-service
    cargo run
} else {
    Write-Host "Скрипт завершено. Запустіть сервіс вручну коли будете готові." -ForegroundColor Green
}
