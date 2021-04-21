CREATE TABLE visits (
    id BIGINT NOT NULL PRIMARY KEY,
    account_id VARCHAR(36) NOT NULL,
    campaign_id VARCHAR(36) NOT NULL,
    traffic_source_id VARCHAR(36) NOT NULL,
    funnel_id VARCHAR NOT NULL,
    impressions_from_traffic_source VARCHAR NOT NULL,
    clicks VARCHAR NOT NULL,
    referrer VARCHAR NOT NULL,
    parameters VARCHAR NOT NULL,
    click_map VARCHAR NOT NULL,
    user_agent_data VARCHAR NOT NULL,
    geo_ip_data VARCHAR NOT NULL,
    conversions VARCHAR NOT NULL,
    custom_conversions VARCHAR NOT NULL,
    last_updated BIGINT NOT NULL
);
