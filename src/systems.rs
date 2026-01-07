use bevy::prelude::*;
use crate::components::*;
use crate::logs::log_event;

pub fn setup_port(mut commands: Commands, stats: Res<PortStats>) {
    commands.spawn(Camera2dBundle::default());

    // Spawn de la UI de dinero
    commands.spawn((
        TextBundle::from_section(
            format!("Dinero: ${}", stats.money),
            TextStyle { font_size: 40.0, color: Color::WHITE, ..default() },
        ),
        MoneyText,
    ));

    // Spawn de algunos contenedores iniciales
    for i in 0..5 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite { color: Color::srgb(0.0, 0.8, 0.0), custom_size: Some(Vec2::new(50.0, 50.0)), ..default() },
                transform: Transform::from_xyz(-200.0 + (i as f32 * 60.0), 0.0, 0.0),
                ..default()
            },
            Container { id: i, status: ContainerStatus::Full },
        ));
    }
}

pub fn process_port_logic(
    mut stats: ResMut<PortStats>,
    mut query: Query<(&mut Container, &mut Sprite)>,
) {
    for (mut container, mut sprite) in query.iter_mut() {
        if container.status == ContainerStatus::Full {
            // Simulación simple: el 1% de probabilidad de vaciarse por frame
            if rand::random::<f32>() < 0.005 {
                container.status = ContainerStatus::Empty;
                sprite.color = Color::srgb(0.5, 0.5, 0.5); // Cambia a gris
                stats.money += 200.0;
                
                log_event("VACIADO", container.id, "Empty", stats.money);
            }
        }
    }
}

pub fn update_ui(stats: Res<PortStats>, mut query: Query<&mut Text, With<MoneyText>>) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Dinero: ${:.2}", stats.money);
    }
}

pub fn handle_cleanup_input(
    keys: Res<ButtonInput<KeyCode>>, // Detecta teclado
    mut stats: ResMut<PortStats>,
    mut commands: Commands,
    query: Query<(Entity, &Container)>,
) {
    // Si presionas la tecla Espacio
    if keys.just_pressed(KeyCode::Space) {
        for (entity, container) in query.iter() {
            if container.status == ContainerStatus::Empty && stats.money >= 50.0 {
                // Eliminar el contenedor del juego (se lo lleva el barco)
                commands.entity(entity).despawn();
                
                // Coste de transporte
                stats.money -= 50.0;
                
                log_event("DESPACHADO", container.id, "Removed", stats.money);
                
                // Solo eliminamos uno por cada pulsación para que sea un reto
                break; 
            }
        }
    }
}