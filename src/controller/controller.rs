use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::prelude::*;
use std::f32::consts::*;

#[derive(Component, Default, Debug)]
pub struct FpsControllerInput {
    pub jump: bool,
    pub pitch: f32,
    pub yaw: f32,
    pub movement: Vec2,
}

#[derive(Component)]
pub struct ControllerSettings {
    pub enable_input: bool,
    pub gravity: f32,
    pub jump_force: f32,
    pub walk_speed: f32,
    pub key_forward: KeyCode,
    pub key_back: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_jump: KeyCode,
    pub sensitivity: f32,
    pub camera_height: f32,
}

impl Default for ControllerSettings {
    fn default() -> Self {
        ControllerSettings {
            enable_input: true,
            gravity: 23.0,
            jump_force: 10.0,
            walk_speed: 9.0,
            key_forward: KeyCode::W,
            key_back: KeyCode::S,
            key_left: KeyCode::A,
            key_right: KeyCode::D,
            key_jump: KeyCode::Space,
            sensitivity: 0.001,
            camera_height: 0.75,
        }
    }
}

#[derive(Component)]
pub struct LogicalPlayer;

#[derive(Component)]
pub struct RenderPlayer {
    pub logical_entity: Entity,
}

#[derive(Bundle)]
pub struct FpsCharacterController {
    rapier_controller: KinematicCharacterController,
    controller_settings: ControllerSettings,
    velocity: Velocity,
    transform: TransformBundle,
    collider: Collider,
    input: FpsControllerInput,
    logical_player: LogicalPlayer,
}

impl Default for FpsCharacterController {
    fn default() -> Self {
        Self {
            rapier_controller: KinematicCharacterController {
                offset: CharacterLength::Absolute(0.1),
                max_slope_climb_angle: 45.0_f32.to_radians(),
                min_slope_slide_angle: 30.0_f32.to_radians(),
                autostep: Some(CharacterAutostep {
                    max_height: CharacterLength::Absolute(0.5),
                    min_width: CharacterLength::Absolute(0.2),
                    include_dynamic_bodies: true,
                }),
                snap_to_ground: Some(CharacterLength::Absolute(0.5)),
                ..default()
            },
            controller_settings: ControllerSettings::default(),
            velocity: Velocity::default(),
            transform: TransformBundle::default(),
            collider: Collider::capsule(Vec3::Y * 0.5, Vec3::Y * 1.5, 0.5),
            input: FpsControllerInput {
                pitch: -TAU / 12.0,
                yaw: TAU * 5.0 / 8.0,
                ..default()
            },
            logical_player: LogicalPlayer,
        }
    }
}

pub struct FpsControllerPlugin;

impl Plugin for FpsControllerPlugin {
    fn build(&self, app: &mut App) {
        use bevy::input::{gamepad, keyboard, mouse, touch};

        app.add_systems(
            PreUpdate,
            (
                fps_controller_input,
                fps_controller_move,
                fps_controller_update,
                fps_controller_render,
            )
                .chain()
                .after(mouse::mouse_button_input_system)
                .after(keyboard::keyboard_input_system)
                .after(gamepad::gamepad_axis_event_system)
                .after(gamepad::gamepad_button_event_system)
                .after(gamepad::gamepad_connection_system)
                .after(gamepad::gamepad_event_system)
                .after(touch::touch_screen_input_system),
        );
    }
}

const ANGLE_EPSILON: f32 = 0.001953125;

pub fn fps_controller_input(
    key_input: Res<Input<KeyCode>>,
    mut mouse_events: EventReader<MouseMotion>,
    mut query: Query<(&ControllerSettings, &mut FpsControllerInput)>,
) {
    for (settings, mut input) in query.iter_mut() {
        if !settings.enable_input {
            continue;
        }

        let mut mouse_delta = Vec2::ZERO;
        for mouse_event in mouse_events.read() {
            mouse_delta += mouse_event.delta;
        }
        mouse_delta *= settings.sensitivity;

        input.pitch = (input.pitch - mouse_delta.y)
            .clamp(-FRAC_PI_2 + ANGLE_EPSILON, FRAC_PI_2 - ANGLE_EPSILON);
        input.yaw -= mouse_delta.x;
        if input.yaw.abs() > PI {
            input.yaw = input.yaw.rem_euclid(TAU);
        }

        input.movement = Vec2::new(
            get_axis(&key_input, settings.key_forward, settings.key_back),
            get_axis(&key_input, settings.key_right, settings.key_left),
        );
        input.jump = key_input.pressed(settings.key_jump);
    }
}

pub fn fps_controller_update(
    time: Res<Time>,
    mut query: Query<(
        &FpsControllerInput,
        &ControllerSettings,
        &mut Velocity,
        &KinematicCharacterControllerOutput,
    )>,
) {
    let dt = time.delta_seconds();

    for (input, settings, mut velocity, output) in query.iter_mut() {
        if output.grounded {
            if input.jump {
                velocity.linvel.y = settings.jump_force;
            } else {
                velocity.linvel.y = -0.5;
            }
        } else {
            velocity.linvel.y -= settings.gravity * dt;
        }
    }
}

pub fn fps_controller_move(
    time: Res<Time>,
    mut query: Query<(
        &FpsControllerInput,
        &ControllerSettings,
        &Velocity,
        &mut KinematicCharacterController,
    )>,
) {
    let dt = time.delta_seconds();

    for (input, settings, velocity, mut controller) in query.iter_mut() {
        let yaw = Quat::from_rotation_y(input.yaw);
        let direction = yaw * Vec3::X * input.movement.y - yaw * Vec3::Z * input.movement.x;
        let direction = if direction.length_squared() > 0.0 {
            direction.normalize()
        } else {
            direction
        };
        let velocity = direction * settings.walk_speed + Vec3::Y * velocity.linvel.y;
        let translation = velocity * dt;

        controller.translation = Some(translation);
    }
}

pub fn fps_controller_render(
    mut render_query: Query<(&mut Transform, &RenderPlayer), With<RenderPlayer>>,
    logical_query: Query<
        (&Transform, &Collider, &ControllerSettings, &FpsControllerInput),
        (With<LogicalPlayer>, Without<RenderPlayer>),
    >,
) {
    for (mut render_transform, render_player) in render_query.iter_mut() {
        if let Ok((logical_transform, collider, settings, input)) =
            logical_query.get(render_player.logical_entity)
        {
            if let Some(capsule) = collider.as_capsule() {
                let camera_height = capsule.segment().b().y + capsule.radius() * settings.camera_height;
                render_transform.translation =
                    logical_transform.translation + Vec3::Y * camera_height;
                render_transform.rotation =
                    Quat::from_euler(EulerRot::YXZ, input.yaw, input.pitch, 0.0);
            }
        }
    }
}

fn get_pressed(key_input: &Res<Input<KeyCode>>, key: KeyCode) -> f32 {
    if key_input.pressed(key) {
        1.0
    } else {
        0.0
    }
}

fn get_axis(key_input: &Res<Input<KeyCode>>, key_pos: KeyCode, key_neg: KeyCode) -> f32 {
    get_pressed(key_input, key_pos) - get_pressed(key_input, key_neg)
}
