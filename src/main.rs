use bevy::asset::RenderAssetUsages;
use bevy::color::palettes::css;
//use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
//use bevy::render::extract_resource::ExtractResource;
//use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::{
    Extent3d, /*TextureDescriptor,*/ TextureDimension, TextureFormat, TextureUsages,
};
//use bevy::render::texture::GpuImage;
//use bevy::render::{Extract, Render, RenderApp, RenderSet};
//use bevy::utils::HashMap;
use bevy_smithay::external_image::ExternalImages;
use bevy_smithay::state::{/*DiagnosticText,*/ MainCamera, MainTexture};
use bevy_smithay::SmithayPlugin;

// #[derive(Clone, Debug, Resource)]
// pub struct FontCollection {
//     roboto_mono: Handle<Font>,
//     noto_mono: Handle<Font>,
//     noto_symbols: Handle<Font>,
// }

// impl FontCollection {
//     fn text_style(font: &Handle<Font>, font_size: f32, color: impl Into<Color>) -> TextStyle {
//         TextStyle {
//             font: font.clone(),
//             font_size,
//             color: color.into(),
//         }
//     }

//     pub fn roboto_mono(&self, font_size: f32, color: impl Into<Color>) -> TextStyle {
//         Self::text_style(&self.roboto_mono, font_size, color)
//     }

//     pub fn noto_mono(&self, font_size: f32, color: impl Into<Color>) -> TextStyle {
//         Self::text_style(&self.noto_mono, font_size, color)
//     }

//     pub fn noto_symbols(&self, font_size: f32, color: impl Into<Color>) -> TextStyle {
//         Self::text_style(&self.noto_symbols, font_size, color)
//     }
// }

// fn setup_fonts(asset_server: ResMut<AssetServer>, mut commands: Commands) {
//     commands.insert_resource(FontCollection {
//         roboto_mono: asset_server.load("fonts/RobotoMono-SemiBold.ttf"),
//         noto_mono: asset_server.load("fonts/NotoSansMono-Bold.ttf"),
//         noto_symbols: asset_server.load("fonts/NotoSansSymbols2-Regular.ttf"),
//     });
// }

// fn update_text(
//     diagnostic: Res<DiagnosticsStore>,
//     mut diagnostic_text: Query<&mut Text, With<DiagnosticText>>,
//     main_camera: Query<&mut Transform, With<MainCamera>>,
// ) {
//     let Ok(mut diagnostic_text) = diagnostic_text.get_single_mut() else {
//         return;
//     };

//     let main_camera = main_camera.single();

//     if let Some(fps) = diagnostic.get(&FrameTimeDiagnosticsPlugin::FPS) {
//         if let Some(value) = fps.smoothed() {
//             diagnostic_text.sections[1].value = format!("{value:.2}");
//         }
//     }

//     let view_angle = main_camera.rotation.to_euler(EulerRot::YXZ);
//     let (yaw, pitch, roll) =
//         (Vec3::from(view_angle) * Vec3::splat(180.0 / std::f32::consts::PI)).into();

//     let roll = if roll == 0.0 { 0.0 } else { roll };

//     diagnostic_text.sections[5].value = format!("{yaw:.2}");
//     diagnostic_text.sections[7].value = format!("{pitch:.2}");
//     diagnostic_text.sections[9].value = format!("{roll:.2}");
// }

fn setup(
    asset_server: ResMut<AssetServer>,
    mut commands: Commands,
    images: ResMut<Assets<Image>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    main_texture: Res<MainTexture>,
) {
    //commands.spawn(DirectionalLightBundle::default());

    let roboto_mono = asset_server.load("RobotoMono-SemiBold.ttf");

    let camera = commands
        .spawn((
            Camera3d::default(),
            Camera {
                target: main_texture.0.clone(),
                ..default()
            },
            Transform::default().looking_at(Vec3::Z * -180.0, Vec3::Y),
            MainCamera,
        ))
        .id();

    commands.spawn((
        Text::new("ok it works"),
        TextColor(Color::BLACK),
        TextFont::from_font(roboto_mono).with_font_size(24.0),
        TargetCamera(camera),
    ));
}

#[derive(Component)]
struct Dont;

fn setup_window(
    asset_server: ResMut<AssetServer>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut external_images: ResMut<ExternalImages>,
    query: Query<&Dont>,
) {
    if query.get_single().is_ok() {
        return;
    }

    let Some(texture_id) = external_images.assets.keys().next() else {
        return;
    };

    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    let mut image = Image::new_fill(
        size,
        TextureDimension::D2,
        &[0; 4],
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::default(),
    );

    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING;

    let image_handle = images.add(image);

    let texture_camera = commands
        .spawn((
            Camera2d::default(),
            Camera {
                clear_color: ClearColorConfig::Custom(css::GREEN.into()),
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
        ))
        .id();

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle),
        unlit: true,
        ..default()
    });

    let mut transform = Transform::from_xyz(0.0, 0.0, -180.0);

    transform.rotate_x(90.0_f32.to_radians());

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(256.0, 144.0))),
        MeshMaterial3d(material_handle),
        transform,
    ));

    commands
        .spawn((
            Node {
                display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            TargetCamera(texture_camera),
            Dont,
        ))
        .with_children(|builder| {
            builder.spawn(ImageNode::from(texture_id.clone()));
        });
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins((DefaultPlugins, SmithayPlugin))
        .add_systems(Update, (setup.run_if(resource_added::<MainTexture>)))
        .run();
}
