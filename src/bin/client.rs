use bevy::prelude::*;
use medieval_call_of_duty::client::ClientPlugin;

fn main() {
    App::new()
        .add_plugins(ClientPlugin)
        .run();
}
