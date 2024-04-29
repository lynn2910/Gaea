use serde::{Deserialize, Serialize};
use uuid::Uuid;
use players::PlayerID;

pub type DoctrineID = Uuid;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Doctrine {
    id: DoctrineID,
    name: String,
    #[serde(default)]
    icon: Option<String>
}

impl Doctrine {
    pub fn new(name: String) -> Self {
        Self { name, id: Uuid::new_v4(), icon: None }
    }

    pub fn set_icon(&mut self, icon: Option<String>) {
        self.icon = icon;
    }

    pub fn icon(&self) -> Option<&String> {
        self.icon.as_ref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> DoctrineID {
        self.id
    }
}

pub type GroupID = Uuid;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Group {
    id: GroupID,
    name: String,
    /// The group's tag can be used by extensions to identify a specific country or group
    tag: String,
    /// A path to this group icon (image)
    flag: Option<String>,
    /// The list of players in this group
    players: Vec<PlayerID>
}

impl Group {
    pub fn new(name: String, tag: String) -> Self {
        Self { name, tag, flag: None, id: Uuid::new_v4(), players: Vec::new() }
    }

    pub fn set_flag(&mut self, flag: Option<String>) {
        self.flag = flag;
    }

    pub fn add_players(&mut self, players: Vec<PlayerID>) {
        for p in players { self.add_player(p); }
    }

    pub fn add_player(&mut self, player: PlayerID) {
        self.players.push(player)
    }

    pub fn id(&self) -> GroupID {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn flag(&self) -> Option<&String> {
        self.flag.as_ref()
    }
    
    pub fn players(&self) -> &Vec<PlayerID> {
        &self.players
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }
}