CREATE TABLE click_identity (
    visit_record_id BIGINT NULL PRIMARY KEY,
    user_agent VARCHAR(255) NOT NULL,
    ip VARCHAR(255) NOT NULL,
    click_map VARCHAR NOT NULL
);
