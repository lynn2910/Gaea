use serde::{Deserialize, Serialize};
use uuid::Uuid;
use groups::GroupID;
use map::Point;
use crate::entity_type::EntityTypeID;

pub type EntityID = Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Entity {
    id:          EntityID,
    name:        String,
    tag:         String,
    data:        Option<serde_json::Value>,
    group:       GroupID,
    properties:  EntityProperties,
    entity_type: EntityTypeID,
    coordinates: Point,
}

impl Entity {
    pub fn new(name: String, tag: String, group: GroupID) -> Self {
        Self { name, tag, group, id: Uuid::new_v4(), ..Default::default() }
    }
    
    pub fn id(&self) -> EntityID { self.id }
    pub fn name(&self) -> &str { &self.name }
    pub fn tag(&self) -> &str { &self.tag }
    pub fn group(&self) -> GroupID { self.group }
    pub fn entity_type(&self) -> EntityTypeID { self.entity_type }
    
    pub fn data(&self) -> Option<&serde_json::Value> { self.data.as_ref() }
    pub fn data_mut(&mut self) -> Option<&mut serde_json::Value> { self.data.as_mut() }

    pub fn coordinates(&self) -> &Point { &self.coordinates }
    pub fn coordinates_mut(&mut self) -> &mut Point { &mut self.coordinates }

    pub fn properties(&self) -> &EntityProperties { &self.properties }
    pub fn properties_mut(&mut self) -> &mut EntityProperties { &mut self.properties }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct EntityProperties {
    pub attack: bool,
    pub range: u32,
    pub default_speed: f32
}