#!/bin/bash

# Кольори для виводу
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}=== CoreCRM-R Quick Start Script ===${NC}\n"

# Перевірка Rust
echo -e "${YELLOW}[1/6] Перевірка Rust...${NC}"
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Rust не встановлено!${NC}"
    echo "Встановіть Rust: https://rustup.rs/"
    echo "Або виконайте: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi
echo -e "${GREEN}✅ Rust встановлено: $(rustc --version)${NC}\n"

# Перевірка PostgreSQL
echo -e "${YELLOW}[2/6] Перевірка PostgreSQL...${NC}"
if command -v docker &> /dev/null; then
    echo "Docker знайдено. Запускаємо PostgreSQL в Docker..."

    # Перевірити чи контейнер вже існує
    if docker ps -a | grep -q corecrm-postgres; then
        echo "Контейнер corecrm-postgres вже існує. Запускаємо..."
        docker start corecrm-postgres
    else
        echo "Створюємо новий контейнер PostgreSQL..."
        docker run --name corecrm-postgres \
          -e POSTGRES_USER=corecrm \
          -e POSTGRES_PASSWORD=corecrm123 \
          -e POSTGRES_DB=metadata_db \
          -p 5432:5432 \
          -d postgres:16
    fi

    echo -e "${GREEN}✅ PostgreSQL запущено в Docker${NC}\n"
    sleep 3  # Дати час БД запуститися
else
    echo -e "${YELLOW}⚠️  Docker не знайдено. Переконайтеся що PostgreSQL встановлено локально${NC}\n"
fi

# Створення .env файлів
echo -e "${YELLOW}[3/6] Створення .env файлів...${NC}"

# .env для metadata-service
cat > services/metadata-service/.env << EOF
METADATA__DATABASE__URL=postgresql://corecrm:corecrm123@localhost:5432/metadata_db
METADATA__SERVER__HOST=0.0.0.0
METADATA__SERVER__PORT=8001
EOF
echo -e "${GREEN}✅ Створено services/metadata-service/.env${NC}"

# .env для api-gateway (якщо потрібно)
if [ -d "services/api-gateway" ]; then
    cat > services/api-gateway/.env << EOF
GATEWAY__DATABASE__URL=postgresql://corecrm:corecrm123@localhost:5432/api_gateway_db
GATEWAY__SERVER__HOST=0.0.0.0
GATEWAY__SERVER__PORT=8000
GATEWAY__METADATA_SERVICE__URL=http://localhost:8001
GATEWAY__JWT__SECRET=your-super-secret-key-change-in-production
GATEWAY__JWT__EXPIRATION_HOURS=24
EOF
    echo -e "${GREEN}✅ Створено services/api-gateway/.env${NC}\n"
fi

# Збірка проєкту
echo -e "${YELLOW}[4/6] Збірка проєкту (перший раз займе 5-10 хвилин)...${NC}"
cargo build
if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Помилка компіляції${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Проєкт зібрано успішно${NC}\n"

# Підказка про запуск
echo -e "${YELLOW}[5/6] Готово до запуску!${NC}\n"
echo -e "${GREEN}Для запуску metadata-service:${NC}"
echo -e "  cd services/metadata-service"
echo -e "  cargo run"
echo ""
echo -e "${GREEN}Після запуску відкрийте в браузері:${NC}"
echo -e "  ${YELLOW}http://localhost:8001/swagger-ui/${NC}"
echo ""
echo -e "${GREEN}Або для швидкого тесту:${NC}"
echo -e "  curl http://localhost:8001/health"
echo ""

# Питання чи запустити зараз
echo -e "${YELLOW}[6/6] Запустити metadata-service зараз? (y/n)${NC}"
read -r response
if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
    echo -e "${GREEN}Запускаю metadata-service...${NC}\n"
    cd services/metadata-service
    cargo run
else
    echo -e "${GREEN}Скрипт завершено. Запустіть сервіс вручну коли будете готові.${NC}"
fi
