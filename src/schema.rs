table! {
    accounts (id) {
        id -> Varchar,
        report_time_zone -> Varchar,
        billing_currency -> Varchar,
        sys_language -> Varchar,
        domains_configuration -> Varchar,
        work_spaces -> Varchar,
        fuel -> Varchar,
        conversion_registration_time_reporting -> Varchar,
        default_home_screen -> Varchar,
        default_way_to_open_report -> Varchar,
        ip_anonymization -> Bool,
        default_reporting_currency -> Varchar,
        profile_first_name -> Varchar,
        profile_last_name -> Varchar,
        primary_user -> Varchar,
        additional_users -> Varchar,
        skype -> Varchar,
        phone_number -> Varchar,
        two_factor_authentication -> Varchar,
        api_access_keys -> Varchar,
        billing_information -> Varchar,
        custom_conversions -> Varchar,
        referrer_handling_list -> Varchar,
        last_updated -> Int8,
    }
}

table! {
    campaigns (id) {
        id -> Varchar,
        account_id -> Varchar,
        clearance -> Varchar,
        traffic_source -> Varchar,
        country -> Varchar,
        name -> Varchar,
        cost_model -> Varchar,
        cost_value -> Varchar,
        redirect_option -> Varchar,
        campaign_destination -> Varchar,
        campaign_core -> Varchar,
        notes -> Varchar,
        archived -> Bool,
        last_updated -> Int8,
        last_clicked -> Int8,
        hosts -> Varchar,
    }
}

table! {
    click_identity (ua_ip_id) {
        ua_ip_id -> Varchar,
        visit_id -> Int8,
        account_id -> Varchar,
        click_map -> Varchar,
    }
}

table! {
    emails (id) {
        id -> Varchar,
    }
}

table! {
    funnels (id) {
        id -> Varchar,
        account_id -> Varchar,
        country -> Varchar,
        name -> Varchar,
        clearance -> Varchar,
        redirect_option -> Varchar,
        referrer_handling -> Varchar,
        notes -> Varchar,
        conditional_sequences -> Varchar,
        default_sequences -> Varchar,
        archived -> Bool,
        last_updated -> Int8,
    }
}

table! {
    invitation (id) {
        id -> Varchar,
        email -> Varchar,
        email_confirmed -> Bool,
        expires_at -> Timestamp,
    }
}

table! {
    landing_pages (id) {
        id -> Varchar,
        account_id -> Varchar,
        clearance -> Varchar,
        country -> Varchar,
        name -> Varchar,
        tags -> Varchar,
        url -> Varchar,
        url_tokens -> Varchar,
        number_of_calls_to_action -> Varchar,
        vertical -> Varchar,
        language -> Varchar,
        notes -> Varchar,
        weight -> Varchar,
        archived -> Bool,
        last_updated -> Int8,
    }
}

table! {
    linked_conversion (id) {
        id -> Varchar,
        visit_id -> Varchar,
        campaign_id -> Varchar,
        account_id -> Varchar,
        offer_id -> Varchar,
        created_at -> Int8,
    }
}

table! {
    offer_sources (id) {
        id -> Varchar,
        account_id -> Varchar,
        name -> Varchar,
        clearance -> Varchar,
        click_id_token -> Varchar,
        payout_token -> Varchar,
        conversion_id_token -> Varchar,
        custom_events -> Varchar,
        tracking_domain -> Varchar,
        conversion_tracking_method -> Varchar,
        include_additional_parameters_in_postback_url -> Bool,
        payout_currency -> Varchar,
        append_click_id -> Bool,
        accept_duplicate_post_backs -> Bool,
        whitelisted_postback_ips -> Varchar,
        referrer_handling -> Varchar,
        notes -> Varchar,
        archived -> Bool,
        last_updated -> Int8,
    }
}

table! {
    offers (id) {
        id -> Varchar,
        account_id -> Varchar,
        clearance -> Varchar,
        offer_source -> Varchar,
        country -> Varchar,
        name -> Varchar,
        tags -> Varchar,
        url -> Varchar,
        offer_tokens -> Varchar,
        conversion_tracking_method -> Varchar,
        payout_type -> Varchar,
        manual_payout_config -> Varchar,
        conversion_cap_config -> Varchar,
        payout_value -> Varchar,
        currency -> Varchar,
        language -> Varchar,
        vertical -> Varchar,
        notes -> Varchar,
        weight -> Varchar,
        archived -> Bool,
        last_updated -> Int8,
    }
}

table! {
    traffic_sources (id) {
        id -> Varchar,
        account_id -> Varchar,
        name -> Varchar,
        clearance -> Varchar,
        external_id_token_data -> Varchar,
        cost_token_data -> Varchar,
        custom_token_data -> Varchar,
        currency -> Varchar,
        traffic_source_postback_url -> Varchar,
        traffic_source_postback_url_on_custom_event -> Varchar,
        pixel_redirect_url -> Varchar,
        track_impressions -> Bool,
        direct_tracking -> Bool,
        notes -> Varchar,
        archived -> Bool,
        last_updated -> Int8,
    }
}

table! {
    users (id) {
        id -> Varchar,
        account_id -> Varchar,
        email -> Varchar,
        password -> Varchar,
        last_updated -> Int8,
    }
}

allow_tables_to_appear_in_same_query!(
    accounts,
    campaigns,
    click_identity,
    emails,
    funnels,
    invitation,
    landing_pages,
    linked_conversion,
    offer_sources,
    offers,
    traffic_sources,
    users,
);
