use bevy::prelude::*;
use medieval_call_of_duty::server::ServerPlugin;

fn main() {
    App::new()
        .add_plugins(ServerPlugin)
        .run();
}
