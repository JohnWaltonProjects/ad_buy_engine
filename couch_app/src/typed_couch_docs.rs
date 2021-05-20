#[derive(Serialize, Deserialize)]
pub struct CouchUser {
    pub _id: String,
    pub name: String,
    pub password: String,
    pub roles: Vec<String>,
    #[serde(rename = "type")]
    pub _type: String,
}

impl CouchUser {
    pub fn new(name: String, password: String) -> Self {
        Self {
            _id: format!("org.couchdb.user:{}", &name),
            name,
            password,
            roles: Vec::new(),
            _type: String::from("user"),
        }
    }
}
//
// "name": &format!("\"{}\"",&username),
// "password": &format!("\"{}\"", password),
// "roles": [],
// "type": "user"
