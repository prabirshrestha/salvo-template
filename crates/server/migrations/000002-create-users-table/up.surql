DEFINE TABLE user SCHEMAFULL;
DEFINE FIELD name               ON user TYPE string;
DEFINE FIELD username           ON user TYPE string;
DEFINE FIELD email              ON user TYPE string;
DEFINE FIELD email_verified_at  ON user TYPE option<datetime>;
DEFINE FIELD password           ON user TYPE string;
DEFINE FIELD created_at         ON user TYPE datetime;
DEFINE FIELD updated_at         ON user TYPE datetime;

DEFINE INDEX idx_user_username  ON TABLE user FIELDS username UNIQUE;
DEFINE INDEX idx_user_email     ON TABLE user FIELDS email UNIQUE;
