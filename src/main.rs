mod components;
mod systems;
mod logs;

use bevy::prelude::*;
use components::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(PortStats {
            capacity: 20,
            money: 1000.0,
        })
        .add_systems(Startup, setup_port)
        .add_systems(Update, (process_port_logic, update_ui,handle_cleanup_input))
        .run();
}