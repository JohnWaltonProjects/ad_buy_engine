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

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CouchSecurity {
    pub admins: DatabaseAdmins,
    pub members: DatabaseMembers,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DatabaseAdmins {
    pub names: Vec<String>,
    pub roles: Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DatabaseMembers {
    pub names: Vec<String>,
    pub roles: Vec<String>,
}

impl CouchSecurity {
    pub fn for_user(username: String) -> Self {
        let mut members = DatabaseMembers::default();
        members.names.push(username);

        let res = Self {
            admins: DatabaseAdmins::default(),
            members,
        };
        dbg!(&res);
        res
    }
}
