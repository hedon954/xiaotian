# Xiaotian 后端部署指南

## 📋 部署概述

本文档详细说明了 Xiaotian 后端服务的部署步骤、环境要求和运维注意事项。

## 🛠️ 环境要求

### 系统要求

- **操作系统**: Linux (Ubuntu 20.04+, CentOS 8+), macOS 10.15+, Windows 10+
- **CPU**: 最少 2 核，推荐 4 核以上
- **内存**: 最少 4GB，推荐 8GB 以上
- **存储**: 最少 20GB，推荐 50GB 以上 SSD
- **网络**: 稳定的互联网连接

### 软件依赖

- **Rust**: 1.70.0 或更高版本
- **PostgreSQL**: 12.0 或更高版本
- **Redis**: 6.0 或更高版本
- **Docker**: 20.10+ (可选，用于容器化部署)
- **Docker Compose**: 2.0+ (可选)

## 🚀 快速部署

### 方式一：源码部署

#### 1. 环境准备

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 安装 PostgreSQL (Ubuntu)
sudo apt update
sudo apt install postgresql postgresql-contrib

# 安装 Redis (Ubuntu)
sudo apt install redis-server

# 安装 sea-orm-cli
cargo install sea-orm-cli
```

#### 2. 数据库配置

```bash
# 创建数据库用户和数据库
sudo -u postgres psql
CREATE USER xiaotian WITH PASSWORD 'your_password';
CREATE DATABASE xiaotian OWNER xiaotian;
GRANT ALL PRIVILEGES ON DATABASE xiaotian TO xiaotian;
\q
```

#### 3. 应用部署

```bash
# 克隆代码
git clone <repository-url>
cd xiaotian/xiaotian-api

# 配置环境变量
cp .env.example .env
# 编辑 .env 文件，配置数据库连接等信息

# 构建应用
cargo build --release

# 运行数据库迁移
export DATABASE_URL="postgresql://xiaotian:your_password@localhost:5432/xiaotian"
sea-orm-cli migrate up

# 启动服务
cargo run --release
```

### 方式二：Docker 部署

#### 1. 创建 Dockerfile

```dockerfile
# 构建阶段
FROM rust:1.75 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migration ./migration

# 构建应用
RUN cargo build --release

# 运行阶段
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# 创建应用用户
RUN useradd -r -s /bin/false xiaotian

WORKDIR /app

# 复制构建产物
COPY --from=builder /app/target/release/xiaotian-api /app/xiaotian-api
COPY --from=builder /app/migration ./migration

# 设置权限
RUN chown -R xiaotian:xiaotian /app
USER xiaotian

# 暴露端口
EXPOSE 8080

# 健康检查
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8080/health || exit 1

# 启动命令
CMD ["./xiaotian-api"]
```

#### 2. Docker Compose 部署

创建 `docker-compose.yml`:

```yaml
version: '3.8'

services:
  xiaotian-api:
    build: .
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgresql://xiaotian:${DB_PASSWORD}@db:5432/xiaotian
      - REDIS_URL=redis://redis:6379
      - SERVER_HOST=0.0.0.0
      - SERVER_PORT=8080
      - APP_ENVIRONMENT=production
      - LOG_LEVEL=info
    depends_on:
      db:
        condition: service_healthy
      redis:
        condition: service_started
    restart: unless-stopped
    volumes:
      - ./logs:/app/logs
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s

  db:
    image: postgres:15-alpine
    environment:
      - POSTGRES_DB=xiaotian
      - POSTGRES_USER=xiaotian
      - POSTGRES_PASSWORD=${DB_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./init.sql:/docker-entrypoint-initdb.d/init.sql
    restart: unless-stopped
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U xiaotian"]
      interval: 10s
      timeout: 5s
      retries: 5

  redis:
    image: redis:7-alpine
    command: redis-server --appendonly yes --requirepass ${REDIS_PASSWORD}
    volumes:
      - redis_data:/data
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "redis-cli", "--raw", "incr", "ping"]
      interval: 10s
      timeout: 3s
      retries: 5

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
      - ./ssl:/etc/nginx/ssl
    depends_on:
      - xiaotian-api
    restart: unless-stopped

volumes:
  postgres_data:
  redis_data:
```

#### 3. 启动服务

```bash
# 创建环境变量文件
echo "DB_PASSWORD=your_db_password" >> .env
echo "REDIS_PASSWORD=your_redis_password" >> .env

# 启动服务
docker-compose up -d

# 运行数据库迁移
docker-compose exec xiaotian-api sea-orm-cli migrate up

# 查看日志
docker-compose logs -f xiaotian-api
```

## 🔧 环境配置详解

### 生产环境配置 (.env)

```bash
# 服务器配置
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
MAX_REQUEST_SIZE=10485760
REQUEST_TIMEOUT=30
CORS_ORIGINS=https://yourdomain.com,https://admin.yourdomain.com

# 数据库配置
DATABASE_URL=postgresql://xiaotian:${DB_PASSWORD}@db:5432/xiaotian
DB_MAX_CONNECTIONS=20
DB_MIN_CONNECTIONS=5
DB_CONNECT_TIMEOUT=30
DB_IDLE_TIMEOUT=600

# Redis 配置
REDIS_URL=redis://:${REDIS_PASSWORD}@redis:6379
REDIS_MAX_CONNECTIONS=20
REDIS_CONNECT_TIMEOUT=5
REDIS_COMMAND_TIMEOUT=5
REDIS_DEFAULT_TTL=3600

# 邮件配置
SMTP_HOST=smtp.yourprovider.com
SMTP_PORT=587
SMTP_USERNAME=noreply@yourdomain.com
SMTP_PASSWORD=${SMTP_PASSWORD}
SMTP_USE_TLS=true
EMAIL_SENDER=小天AI助手 <noreply@yourdomain.com>

# AI 配置
AI_ENABLED=true
AI_PROVIDER=openai
AI_API_KEY=${OPENAI_API_KEY}
AI_MODEL=gpt-3.5-turbo
AI_MAX_TOKENS=1000
AI_TEMPERATURE=0.7
AI_TIMEOUT=60

# 同步配置
SYNC_MAX_ITEMS=50
SYNC_SUMMARY_LENGTH=medium
SYNC_INTERVAL_MINUTES=60
SYNC_REQUEST_TIMEOUT=30
SYNC_USER_AGENT=Xiaotian-RSS-Reader/1.0

# 应用配置
APP_NAME=Xiaotian API
APP_VERSION=1.0.0
APP_ENVIRONMENT=production
LOG_LEVEL=info
MAX_CHAT_SESSIONS_PER_USER=50
MAX_FEEDS_PER_USER=100
DATA_RETENTION_DAYS=365
```

### Nginx 配置

```nginx
events {
    worker_connections 1024;
}

http {
    upstream xiaotian_backend {
        server xiaotian-api:8080;
    }

    # 限制请求大小
    client_max_body_size 10M;

    # 日志格式
    log_format main '$remote_addr - $remote_user [$time_local] "$request" '
                    '$status $body_bytes_sent "$http_referer" '
                    '"$http_user_agent" "$http_x_forwarded_for"';

    # HTTP 重定向到 HTTPS
    server {
        listen 80;
        server_name yourdomain.com www.yourdomain.com;
        return 301 https://$server_name$request_uri;
    }

    # HTTPS 配置
    server {
        listen 443 ssl http2;
        server_name yourdomain.com www.yourdomain.com;

        # SSL 证书配置
        ssl_certificate /etc/nginx/ssl/cert.pem;
        ssl_certificate_key /etc/nginx/ssl/key.pem;
        ssl_protocols TLSv1.2 TLSv1.3;
        ssl_ciphers ECDHE-RSA-AES256-GCM-SHA512:DHE-RSA-AES256-GCM-SHA512;

        # 安全头
        add_header Strict-Transport-Security "max-age=63072000" always;
        add_header X-Frame-Options DENY;
        add_header X-Content-Type-Options nosniff;
        add_header X-XSS-Protection "1; mode=block";

        # 日志
        access_log /var/log/nginx/xiaotian_access.log main;
        error_log /var/log/nginx/xiaotian_error.log;

        # API 代理
        location /api/ {
            proxy_pass http://xiaotian_backend;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;

            # 超时配置
            proxy_connect_timeout 30s;
            proxy_send_timeout 30s;
            proxy_read_timeout 30s;

            # 缓冲配置
            proxy_buffering on;
            proxy_buffer_size 4k;
            proxy_buffers 8 4k;
        }

        # 健康检查
        location /health {
            proxy_pass http://xiaotian_backend;
            access_log off;
        }

        # 静态文件缓存
        location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg)$ {
            expires 1y;
            add_header Cache-Control "public, immutable";
        }
    }
}
```

## 🔍 监控和日志

### 应用监控

```yaml
# docker-compose.monitoring.yml
version: '3.8'

services:
  prometheus:
    image: prom/prometheus:latest
    ports:
      - "9090:9090"
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus_data:/prometheus

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3000:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=${GRAFANA_PASSWORD}
    volumes:
      - grafana_data:/var/lib/grafana
      - ./grafana/dashboards:/etc/grafana/provisioning/dashboards

  alertmanager:
    image: prom/alertmanager:latest
    ports:
      - "9093:9093"
    volumes:
      - ./alertmanager.yml:/etc/alertmanager/alertmanager.yml

volumes:
  prometheus_data:
  grafana_data:
```

### 日志配置

```rust
// 在 main.rs 中配置结构化日志
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn init_tracing(config: &Config) {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_current_span(true)
                .with_span_list(true)
        )
        .init();
}
```

## 🔒 安全配置

### 防火墙设置

```bash
# Ubuntu UFW 配置
sudo ufw enable
sudo ufw allow 22/tcp    # SSH
sudo ufw allow 80/tcp    # HTTP
sudo ufw allow 443/tcp   # HTTPS
sudo ufw deny 5432/tcp  # PostgreSQL (仅内部访问)
sudo ufw deny 6379/tcp  # Redis (仅内部访问)
```

### 数据库安全

```sql
-- 创建只读用户用于报表查询
CREATE USER xiaotian_readonly WITH PASSWORD 'readonly_password';
GRANT CONNECT ON DATABASE xiaotian TO xiaotian_readonly;
GRANT USAGE ON SCHEMA public TO xiaotian_readonly;
GRANT SELECT ON ALL TABLES IN SCHEMA public TO xiaotian_readonly;

-- 创建备份用户
CREATE USER xiaotian_backup WITH PASSWORD 'backup_password';
GRANT CONNECT ON DATABASE xiaotian TO xiaotian_backup;
GRANT SELECT ON ALL TABLES IN SCHEMA public TO xiaotian_backup;
```

## 🚨 故障排查

### 常见问题

1. **数据库连接失败**
   ```bash
   # 检查数据库状态
   docker-compose exec db pg_isready -U xiaotian

   # 查看数据库日志
   docker-compose logs db
   ```

2. **Redis 连接失败**
   ```bash
   # 检查 Redis 状态
   docker-compose exec redis redis-cli ping

   # 查看 Redis 日志
   docker-compose logs redis
   ```

3. **内存不足**
   ```bash
   # 监控内存使用
   docker stats xiaotian-api

   # 调整内存限制
   # 在 docker-compose.yml 中添加:
   # deploy:
   #   resources:
   #     limits:
   #       memory: 2G
   ```

4. **磁盘空间不足**
   ```bash
   # 检查磁盘使用
   df -h

   # 清理日志
   docker-compose exec xiaotian-api find /app/logs -name "*.log" -mtime +7 -delete
   ```

### 性能调优

1. **数据库优化**
   ```sql
   -- 创建索引
   CREATE INDEX CONCURRENTLY idx_summaries_feed_id_created_at
   ON summaries(feed_id, created_at DESC);

   -- 分析查询性能
   EXPLAIN ANALYZE SELECT * FROM summaries
   WHERE feed_id = 1 ORDER BY created_at DESC LIMIT 20;
   ```

2. **连接池调优**
   ```bash
   # 在 .env 中调整连接池大小
   DB_MAX_CONNECTIONS=50
   DB_MIN_CONNECTIONS=10
   REDIS_MAX_CONNECTIONS=30
   ```

## 📋 维护任务

### 定期维护

```bash
#!/bin/bash
# maintenance.sh - 定期维护脚本

# 清理旧日志
find /var/log/xiaotian -name "*.log" -mtime +30 -delete

# 数据库维护
docker-compose exec db psql -U xiaotian -d xiaotian -c "VACUUM ANALYZE;"

# 备份数据库
docker-compose exec db pg_dump -U xiaotian xiaotian > backup_$(date +%Y%m%d_%H%M%S).sql

# 清理 Redis 过期键
docker-compose exec redis redis-cli --scan --pattern "*" | xargs redis-cli del
```

### 监控检查

```bash
#!/bin/bash
# health_check.sh - 健康检查脚本

# 检查 API 健康状态
curl -f http://localhost:8080/health || exit 1

# 检查数据库连接
docker-compose exec db pg_isready -U xiaotian || exit 1

# 检查 Redis 连接
docker-compose exec redis redis-cli ping || exit 1

# 检查磁盘空间
DISK_USAGE=$(df / | awk 'NR==2{print $5}' | sed 's/%//')
if [ $DISK_USAGE -gt 80 ]; then
    echo "Warning: Disk usage is ${DISK_USAGE}%"
fi

echo "All health checks passed"
```

## 📚 参考资料

- [PostgreSQL 官方文档](https://www.postgresql.org/docs/)
- [Redis 官方文档](https://redis.io/documentation)
- [Docker 部署指南](https://docs.docker.com/)
- [Nginx 配置指南](https://nginx.org/en/docs/)
- [Rust 部署最佳实践](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)