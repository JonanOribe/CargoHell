use bevy::prelude::*;
use serde::Serialize;

#[derive(PartialEq, Serialize, Clone, Copy)]
pub enum ContainerStatus {
    Full,
    Empty,
}

#[derive(Component)]
pub struct Container {
    pub id: u32,
    pub status: ContainerStatus,
}

#[derive(Resource)]
pub struct PortStats {
    pub capacity: usize,
    pub money: f32,
}

#[derive(Component)]
pub struct MoneyText; // Para identificar el texto en la UI