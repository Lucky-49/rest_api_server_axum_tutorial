-- используется ТОЛЬКО для разработки - Удаление базы данных методом перебора
-- (для локальной разработки и модульного тестирования)
SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE usename = 'app_user'
OR datname = 'app_db';
DROP DATABASE IF EXISTS app_db;
DROP USER IF EXISTS app_user;

-- используется ТОЛЬКО для разработки - пароль только для разработчиков
-- (для локальных разработчиков и модульного тестирования).

CREATE USER app_user PASSWORD 'dev_only_pwd';
CREATE DATABASE app_db owner app_user ENCODING = 'UTF-8';