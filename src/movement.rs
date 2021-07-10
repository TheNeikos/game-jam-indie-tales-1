use bevy::{
    math::Vec3A,
    prelude::*,
    utils::{HashMap, HashSet},
};
use ordered_float::OrderedFloat;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub struct MovementStage;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum MovementCalculation {
    Movements,
    Velocity,
    Position,
}
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_stage_after(CoreStage::Update, MovementStage, SystemStage::parallel());
        app.add_system_to_stage(
            MovementStage,
            apply_movements
                .system()
                .label(MovementCalculation::Movements),
        );
        app.add_system_to_stage(
            MovementStage,
            incorporate_velocity
                .system()
                .label(MovementCalculation::Velocity)
                .after(MovementCalculation::Movements),
        );
    }
}

#[derive(Bundle, Default)]
pub struct MovementBundle {
    position: Position,
    velocity: Velocity,
    movements: Movements,
}

#[derive(Default, Debug)]
pub struct Position {
    translation: Vec3,
}

#[derive(Default, Debug)]
pub struct Velocity {
    velocity: Vec3,
}

pub fn incorporate_velocity(
    mut velo_query: Query<(&Velocity, &mut Position, &mut Transform)>,
    time: Res<Time>,
) {
    for (velo, mut position, mut transform) in velo_query.iter_mut() {
        // info!("{:?} {:?} {:?}", velo, position, transform);
        if velo.velocity.length() > SMALLEST_MAGNITUDE {
            position.translation += velo.velocity * time.delta_seconds();
            transform.translation = position.translation.round();
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum MovementModifier {
    /// An impulse is directly added to the velocity, bypassing the acceleration
    Impulse { acceleration: Vec3 },
    /// Momentum is acceleration, up to the specified cap in velocity
    Momentum {
        acceleration: Vec3,
        dampening: f32,
        maximal_velocity: Option<f32>,
    },
}

impl Eq for MovementModifier {}

impl std::hash::Hash for MovementModifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            MovementModifier::Impulse { acceleration } => {
                OrderedFloat(acceleration.x).hash(state);
                OrderedFloat(acceleration.y).hash(state);
                OrderedFloat(acceleration.z).hash(state);
            }
            MovementModifier::Momentum {
                acceleration,
                dampening,
                maximal_velocity,
            } => {
                OrderedFloat(acceleration.x).hash(state);
                OrderedFloat(acceleration.y).hash(state);
                OrderedFloat(acceleration.z).hash(state);
                OrderedFloat(*dampening).hash(state);
                maximal_velocity
                    .as_ref()
                    .copied()
                    .map(OrderedFloat)
                    .hash(state);
            }
        }
    }
}

#[derive(Default)]
struct MovementEffect {
    velocity: Vec3,
    acceleration: Vec3,
    dampening: f32,
    maximal_velocity: f32,
    maximal_acceleration: f32,
    updated: bool,
}

#[derive(PartialEq, Hash, Eq, Debug, Clone)]
pub struct Movement {
    pub modifier: MovementModifier,
    pub name: String,
}

impl Movement {
    pub fn new(name: impl Into<String>, modifier: MovementModifier) -> Self {
        Movement {
            name: name.into(),
            modifier,
        }
    }
}

const SMALLEST_MAGNITUDE: f32 = 0.0005;

#[derive(Default)]
pub struct Movements {
    movements: HashSet<Movement>,
    current_effects: HashMap<String, MovementEffect>,
}

impl Movements {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, movement: Movement) {
        self.movements.insert(movement);
    }

    fn process_new_movements(&mut self) {
        for movement in self.movements.drain() {
            match movement.modifier {
                MovementModifier::Impulse { acceleration } => {
                    self.current_effects.insert(
                        movement.name,
                        MovementEffect {
                            acceleration,
                            dampening: 0.00001,
                            updated: true,
                            ..Default::default()
                        },
                    );
                }
                MovementModifier::Momentum {
                    acceleration,
                    dampening,
                    maximal_velocity,
                } => {
                    let entry = self.current_effects.entry(movement.name).or_default();
                    entry.acceleration = acceleration;
                    entry.dampening = dampening;
                    entry.maximal_velocity = maximal_velocity.unwrap_or(f32::MAX);
                    entry.updated = true;
                }
            }
        }
    }

    fn update_movements(&mut self, delta_time: f32) {
        for effect in self.current_effects.values_mut() {
            if !effect.updated {
                effect.velocity *= effect.dampening.powf(delta_time);
            }
            effect.velocity += effect.acceleration * delta_time;

            if !effect.updated {
                effect.acceleration *= effect.dampening.powf(delta_time);
            }

            effect.velocity = effect.velocity.length() * effect.acceleration.normalize().lerp(effect.velocity.normalize(), 0.75).normalize();

            effect.velocity = effect.velocity.clamp_length(0., effect.maximal_velocity);

            if effect.velocity.x.abs() < SMALLEST_MAGNITUDE {
                effect.velocity.x = 0.;
            }
            if effect.velocity.y.abs() < SMALLEST_MAGNITUDE {
                effect.velocity.y = 0.;
            }
            if effect.velocity.z.abs() < SMALLEST_MAGNITUDE {
                effect.velocity.z = 0.;
            }

            effect.updated = false;
        }

        self.current_effects.retain(|_name, effect| {
            effect.velocity.length() > SMALLEST_MAGNITUDE
                || effect.acceleration.length() > SMALLEST_MAGNITUDE
        });
    }

    fn get_total_direction(&self) -> Vec3 {
        self.current_effects
            .iter()
            .map(|(_, effect)| Vec3A::from(effect.velocity))
            .fold(Vec3A::ZERO, |data, next| data + next)
            .into()
    }
}

fn apply_movements(mut mov_query: Query<(&mut Movements, &mut Velocity)>, time: Res<Time>) {
    for (mut movements, mut velocity) in mov_query.iter_mut() {
        movements.process_new_movements();
        movements.update_movements(time.delta_seconds());
        velocity.velocity = movements.get_total_direction();
    }
}
