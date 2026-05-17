-- Administrative accounts for backoffice JWT login.
-- The username is unique; password_hash stores a bcrypt hash (cost=12).
CREATE TABLE IF NOT EXISTS admin_users (
    id            CHAR(36) PRIMARY KEY,
    username      VARCHAR(64) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at    DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at    DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_admin_users_username (username)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
