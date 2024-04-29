use serde::{Deserialize, Serialize};
use uuid::Uuid;
use groups::GroupID;
use map::LandType;

pub type EntityTypeID = Uuid;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct EntityType {
    id:            EntityTypeID,
    name:          String,
    description:   Option<String>,
    /// Only used for the editor
    extended_from: Vec<EntityTypeID>,
    reserved_to:   Vec<GroupID>,
    unusable_for:  Vec<GroupID>,
    properties:    EntityProperties,
    icon:          Option<String>
}

impl EntityType {
    pub fn new(name: String) -> Self {
        Self {
            name,
            id: Uuid::new_v4(),
            icon: None,
            properties: EntityProperties::default(),
            unusable_for: Vec::new(),
            reserved_to: Vec::new(),
            extended_from: Vec::new(),
            description: None
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn extended_from(&self) -> &Vec<EntityTypeID> {
        &self.extended_from
    }
    pub fn reserved_to(&self) -> &Vec<GroupID> {
        &self.reserved_to
    }
    pub fn unusable_for(&self) -> &Vec<GroupID> {
        &self.unusable_for
    }

    pub fn properties(&self) -> &EntityProperties {
        &self.properties
    }
    pub fn properties_mut(&mut self) -> &mut EntityProperties {
        &mut self.properties
    }

    pub fn icon(&self) -> Option<&String> {
        self.icon.as_ref()
    }
    pub fn set_icon(&mut self, icon: Option<String>) {
        self.icon = icon;
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct EntityProperties {
    pub terrains: Vec<LandType>
}