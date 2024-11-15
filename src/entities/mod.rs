pub mod player;

use bevy::prelude::*;
use crate::entities::player::PlayerPlugin;

//################################################# Models #################################################
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct EntitiesBase {
    pub max_health: f32,
    pub max_mana: f32,
    pub max_stamina: f32,
    pub current_stats: CurrentStats,
    pub speed: f32,
    pub jump_height: f32,
    pub offset: OffsetTransform,
    pub general_defence: GeneralDefence,
    pub resistances: Resistances
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct OffsetTransform {
    pub y: f32,
    pub x: f32,
    pub z: f32
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct CurrentStats {
    pub health: f32,
    pub stamina: f32,
    pub stamina_fill_count: f32,
    pub mana: f32,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct PhysicalDefence {
    pub vs_strike: f32,
    pub vs_slash: f32,
    pub vs_thrust: f32
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct GeneralDefence {
    pub physical_defence: PhysicalDefence,
    pub magic_defence: f32,
    pub fire_defence: f32,
    pub lightning_defence: f32,
    pub demonic_defence: f32,
    pub corruption_defence: f32,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Resistances {
    pub bleed_resistance: f32,
    pub poison_resistance: f32,
    pub frost_resistance: f32,
    pub curse_resistance: f32,
    pub holy_resistance: f32,
    pub rotten_resistance: f32
}

#[derive(Component, Reflect, Resource, Debug)]
pub struct Animations {
    pub(crate) animations: Vec<AnimationNodeIndex>,
    #[allow(dead_code)]
    pub graph: Handle<AnimationGraph>
}

//################################################# Default Values #################################################
impl Default for EntitiesBase {
    fn default() -> Self {
        Self {
            max_health: 450.0,
            max_mana: 145.0,
            max_stamina: 270.0,
            current_stats: CurrentStats::default(),
            speed: 2.3,
            jump_height: 3.5,
            offset: OffsetTransform::default(),
            general_defence: GeneralDefence::default(),
            resistances: Resistances::default(),
        }
    }
}

impl Default for OffsetTransform {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }
}

impl Default for CurrentStats {
    fn default() -> Self {
        Self {
            health: 450.0,
            stamina: 270.0,
            stamina_fill_count: 70.0,
            mana: 145.0,
        }
    }
}

impl Default for PhysicalDefence {
    fn default() -> Self {
        Self {
            vs_strike: 5.0,
            vs_slash: 5.0,
            vs_thrust: 5.0,
        }
    }
}

impl Default for GeneralDefence {
    fn default() -> Self {
        Self {
            physical_defence: PhysicalDefence::default(),
            magic_defence: 5.0,
            fire_defence: 5.0,
            lightning_defence: 5.0,
            demonic_defence: 5.0,
            corruption_defence: 5.0
        }
    }
}

impl Default for Resistances {
    fn default() -> Self {
        Self {
            bleed_resistance: 2.0,
            poison_resistance: 2.0,
            frost_resistance: 2.0,
            curse_resistance: 2.0,
            holy_resistance: 2.0,
            rotten_resistance: 2.0,
        }
    }
}

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin);
    }
}