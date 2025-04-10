use bevy::prelude::*;
use bevy_aseprite::{anim::AsepriteAnimation, AsepriteBundle, AsepritePlugin, AsepriteHandle};
use bevy_inspector_egui::quick::WorldInspectorPlugin;


#[derive(Component, Clone, Copy, Debug)]
struct CrowTag;

#[derive(Component, Clone, Copy, Debug)]
struct PlayerTag;

mod sprites {
    use bevy_aseprite::aseprite;

    // https://meitdev.itch.io/crow
    aseprite!(pub Crow, "crow.aseprite");
    // https://shubibubi.itch.io/cozy-people
    aseprite!(pub Player, "player.ase");
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AsepritePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, change_animation)
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("setup");
    commands.spawn(Camera2d::default());

    commands
        .spawn((
            AsepriteBundle {
                aseprite: AsepriteHandle(asset_server.load(sprites::Crow::PATH)),
                animation: AsepriteAnimation::from(sprites::Crow::tags::FLAP_WINGS),
                ..Default::default()
            }, 
            Transform {
                scale: Vec3::splat(4.),
                translation: Vec3::new(0., 80., 0.),
                ..Default::default()
            },
        ))
        .insert(CrowTag);

    commands
        .spawn((
            AsepriteBundle {
                aseprite: AsepriteHandle(asset_server.load(sprites::Player::PATH)),
                animation: AsepriteAnimation::from(sprites::Player::tags::LEFT_WALK),
                ..Default::default()
            },
            Transform {
                scale: Vec3::splat(4.),
                translation: Vec3::new(0., -100., 0.),
                ..Default::default()
            },
        ))
        .insert(PlayerTag);
}

fn change_animation(
    keys: Res<ButtonInput<KeyCode>>,
    mut aseprites: ParamSet<(
        Query<&mut AsepriteAnimation, With<CrowTag>>,
        Query<&mut AsepriteAnimation, With<PlayerTag>>,
    )>,
) {
    if keys.just_pressed(KeyCode::Digit1) {
        for mut crow_anim in aseprites.p0().iter_mut() {
            *crow_anim = AsepriteAnimation::from(sprites::Crow::tags::FLAP_WINGS);
        }
        for mut player_anim in aseprites.p1().iter_mut() {
            *player_anim = AsepriteAnimation::from(sprites::Player::tags::LEFT_WALK);
        }
    }
    if keys.just_pressed(KeyCode::Digit2) {
        for mut crow_anim in aseprites.p0().iter_mut() {
            *crow_anim = AsepriteAnimation::from(sprites::Crow::tags::GROOVE);
        }
        for mut player_anim in aseprites.p1().iter_mut() {
            *player_anim = AsepriteAnimation::from(sprites::Player::tags::RIGHT_WALK);
        }
    }

    if keys.pressed(KeyCode::KeyS) {
        for mut crow_anim in aseprites.p0().iter_mut() {
            crow_anim.custom_size = Some(crow_anim.custom_size.unwrap_or(Vec2::splat(40.)) + Vec2::splat(- 2.));
        }
        for mut player_anim in aseprites.p1().iter_mut() {
            player_anim.custom_size = Some(player_anim.custom_size.unwrap_or(Vec2::splat(40.)) + Vec2::splat(- 2.));
        }
    }

    if keys.pressed(KeyCode::KeyW) {
        for mut crow_anim in aseprites.p0().iter_mut() {
            crow_anim.custom_size = Some(crow_anim.custom_size.unwrap_or(Vec2::splat(40.)) + Vec2::splat(2.));
        }
        for mut player_anim in aseprites.p1().iter_mut() {
            player_anim.custom_size = Some(player_anim.custom_size.unwrap_or(Vec2::splat(40.)) + Vec2::splat(2.));
        }
    }

    if keys.just_pressed(KeyCode::Space) {
        for mut crow_anim in aseprites.p0().iter_mut() {
            crow_anim.toggle();
        }
        for mut player_anim in aseprites.p1().iter_mut() {
            player_anim.toggle();
        }
    }
}