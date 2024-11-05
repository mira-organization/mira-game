mod player_base;
mod player_input;

use bevy::prelude::*;
use crate::entities::EntitiesBase;
use crate::entities::player::player_base::PlayerBasePlugin;
use crate::entities::player::player_input::PlayerInputPlugin;

//################################################# Models #################################################
#[derive(Component, Reflect, Resource, Debug)]
#[reflect(Component)]
pub struct Player {
    pub general: PlayerGeneralStats,
    pub base: EntitiesBase,
    pub speed_sprinting_multiplier: f32,
    pub speed_sneaking_multiplier: f32,
    pub state: PlayerState,
    pub environment_state: PlayerEnvironmentState,
    pub timers: StatsTimer,
    pub consume_entries: ConsumeEntries
}

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub enum PlayerState {
    Idling,
    Moving,
    Jumping,
    Attacking,
    Sprinting,
    Sneaking,
    Dodging,
    Grounded,
    Climbing,
    Blocking,
    Dead,
}

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
#[reflect(Component)]
pub enum PlayerEnvironmentState {
    Fighting,
    BossFight,
    DungeonEncounter,
    Ui,
    Exploring,
    Invasion,
    Trapped
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct StatsTimer {
    pub sprint_timer: f32,
    pub stamina_fill_timer: f32,
    pub stamina_fill_delay: f32,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct ConsumeEntries {
    pub dodge: bool,
    pub attack: bool,
    pub jump: bool,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct PlayerSkillAbleStats {
    pub vitality: f32,
    pub endurance: f32,
    pub attunement: f32,
    pub strength: f32,
    pub dexterity: f32,
    pub intelligence: f32,
    pub faith: f32,
    pub demonic: f32,
    pub luck: f32,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct PlayerGeneralStats {
    pub level: u16,
    pub equipment_load: f32,
    pub poise: f32,
    pub discovery: f32
}

//################################################# Default Values #################################################
impl Default for Player {
    fn default() -> Self {
        Self {
            general: PlayerGeneralStats::default(),
            base: EntitiesBase::default(),
            speed_sprinting_multiplier: 1.5,
            speed_sneaking_multiplier: 0.7,
            state: PlayerState::default(),
            environment_state: PlayerEnvironmentState::default(),
            timers: StatsTimer::default(),
            consume_entries: ConsumeEntries::default(),
        }
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::Idling
    }
}

impl Default for PlayerEnvironmentState {
    fn default() -> Self {
        PlayerEnvironmentState::Exploring
    }
}

impl Default for StatsTimer {
    fn default() -> Self {
        Self {
            sprint_timer: 0.0,
            stamina_fill_timer: 0.0,
            stamina_fill_delay: 0.8,
        }
    }
}

impl Default for ConsumeEntries {
    fn default() -> Self {
        Self {
            dodge: true,
            attack: true,
            jump: true,
        }
    }
}

impl Default for PlayerSkillAbleStats {
    fn default() -> Self {
        Self {
            vitality: 0.0,
            endurance: 0.0,
            attunement: 0.0,
            strength: 0.0,
            dexterity: 0.0,
            intelligence: 0.0,
            faith: 0.0,
            demonic: 0.0,
            luck: 0.0,
        }
    }
}

impl Default for PlayerGeneralStats {
    fn default() -> Self {
        Self {
            level: 1,
            equipment_load: 0.0,
            poise: 12.0,
            discovery: 5.0,
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>();
        app.add_plugins((PlayerBasePlugin, PlayerInputPlugin));
    }
}