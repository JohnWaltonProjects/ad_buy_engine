use maxminddb::geoip2::model::Traits;
use maxminddb::geoip2::{AnonymousIp, City, ConnectionType, DensityIncome, Isp};
use std::net::IpAddr;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GeoIPData {
    pub ip: IpAddr,
    pub city: String,
    pub continent: String,
    pub country_iso_code: String,
    pub subdivision_iso_code: String,
    pub time_zone: String,
    pub latitude: f64,
    pub longitude: f64,
    pub metro_code: u16,
    pub postal_code: String,
    pub asn: String,
    pub isp: String,
    pub connection_type: String,
    pub is_anonymous_proxy: bool,
    pub is_anonymous: bool,
    pub is_anonymous_vpn: bool,
    pub is_hosting_provider: bool,
    pub is_public_proxy: bool,
    pub is_satellite_provider: bool,
    pub is_tor_exit_node: bool,
    pub average_income: u32,
    pub population_density: u32,
}

impl GeoIPData {
    pub fn new(ip: IpAddr, path_to_db: &str) -> Self {
        let city_reader =
            maxminddb::Reader::open_readfile(format!("{}GeoLite2-City.mmdb", path_to_db))
                .expect("F3442");
        // maxminddb::Reader::open_readfile("../../../GeoLite2-City.mmdb").expect("F32");
        let asn_reader =
            maxminddb::Reader::open_readfile(format!("{}GeoLite2-ASN.mmdb", path_to_db))
                .expect("Fdd32");

        let city: City = city_reader.lookup(ip).expect("fds");
        let isp: Isp = asn_reader.lookup(ip).expect("fdds");
        let ct: ConnectionType = asn_reader.lookup(ip).expect("fdds");
        let traits: Traits = city_reader.lookup(ip).expect("fdds");
        let anonymous_ip: AnonymousIp = city_reader.lookup(ip).expect("fdds");
        let density: DensityIncome = city_reader.lookup(ip).expect("fdds");

        let city_name = if let Some(c) = city.city {
            if let Some(n) = c.names {
                n.get("en").expect("Gg3")
            } else {
                ""
            }
        } else {
            ""
        };

        let continent = if let Some(c) = city.continent {
            if let Some(n) = c.names {
                n.get("en").expect("Gg3")
            } else {
                ""
            }
        } else {
            ""
        };

        let country_iso_code = if let Some(c) = city.country {
            if let Some(n) = c.iso_code {
                n
            } else {
                ""
            }
        } else {
            ""
        };

        let subdivision_iso_code = if let Some(c) = city.subdivisions {
            if let Some(n) = c.get(0) {
                n.iso_code.unwrap_or("")
            } else {
                ""
            }
        } else {
            ""
        };

        let time_zone = if let Some(c) = &city.location {
            if let Some(n) = c.time_zone {
                n
            } else {
                ""
            }
        } else {
            ""
        };

        let latitude = if let Some(c) = &city.location {
            if let Some(n) = c.latitude {
                n
            } else {
                0.0
            }
        } else {
            0.0
        };

        let longitude = if let Some(c) = &city.location {
            if let Some(n) = c.longitude {
                n
            } else {
                0.0
            }
        } else {
            0.0
        };

        let metro_code = if let Some(c) = &city.location {
            if let Some(n) = c.metro_code {
                n
            } else {
                0
            }
        } else {
            0
        };

        let postal_code = if let Some(c) = city.postal {
            if let Some(n) = c.code {
                n
            } else {
                ""
            }
        } else {
            ""
        };

        let asn = if let Some(c) = isp.autonomous_system_organization {
            c
        } else {
            ""
        };

        let isp = if let Some(c) = isp.isp { c } else { "" };

        let connection_type = if let Some(c) = ct.connection_type {
            c
        } else {
            ""
        };

        let is_anonymous_proxy = if let Some(c) = traits.is_anonymous_proxy {
            c
        } else {
            false
        };

        let is_anonymous = if let Some(c) = anonymous_ip.is_anonymous {
            c
        } else {
            false
        };

        let is_anonymous_vpn = if let Some(c) = anonymous_ip.is_anonymous_vpn {
            c
        } else {
            false
        };

        let is_hosting_provider = if let Some(c) = anonymous_ip.is_hosting_provider {
            c
        } else {
            false
        };

        let is_public_proxy = if let Some(c) = anonymous_ip.is_public_proxy {
            c
        } else {
            false
        };

        let is_satellite_provider = if let Some(c) = traits.is_satellite_provider {
            c
        } else {
            false
        };

        let is_tor_exit_node = if let Some(c) = anonymous_ip.is_tor_exit_node {
            c
        } else {
            false
        };

        let average_income = if let Some(c) = density.average_income {
            c
        } else {
            0
        };

        let population_density = if let Some(c) = density.population_density {
            c
        } else {
            0
        };

        Self {
            ip,
            city: "".to_string(),
            continent: "".to_string(),
            country_iso_code: "".to_string(),
            subdivision_iso_code: "".to_string(),
            time_zone: "".to_string(),
            latitude,
            longitude,
            metro_code,
            postal_code: "".to_string(),
            asn: "".to_string(),
            isp: "".to_string(),
            connection_type: "".to_string(),
            is_anonymous_proxy,
            is_anonymous,
            is_anonymous_vpn,
            is_hosting_provider,
            is_public_proxy,
            is_satellite_provider,
            is_tor_exit_node,
            average_income,
            population_density,
        }
    }
}
