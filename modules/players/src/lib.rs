use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type PlayerID = Uuid;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Player {
    id:   PlayerID,
    name: String,
    tag:  String,
    /// The password hash converted to a hex suit
    password: String
}

impl Player {
    pub fn check_password(&self, password: impl ToString) -> bool {
        password.to_string() == self.password
    }

    /// Create a new user
    pub fn new(name: String, tag: String, password: String) -> Self {
        Self { name, tag, password, id: Uuid::new_v4() }
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn tag(&self) -> &str {
        &self.tag
    }
    
    pub fn id(&self) -> PlayerID {
        self.id
    }

}