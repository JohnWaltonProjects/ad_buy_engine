CREATE TABLE linked_conversion (
    id VARCHAR(24) NULL PRIMARY KEY,
    visit_id BIGINT NOT NULL,
    campaign_id VARCHAR(36) NOT NULL,
    offer_id VARCHAR(36) NOT NULL,
    created_at BIGINT NOT NULL
);
