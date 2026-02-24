use bevy::prelude::*;

use vibetorio_sim::{
    CommandQueue, SimulationClock, SimulationConfig, SimulationWorldState, WorldGrid,
    init_world_state, process_command_queue, step_clock, step_world,
};

mod bootstrap;

fn main() {
    build_app().run();
}

pub fn build_app() -> App {
    let mut app = App::new();

    #[cfg(not(test))]
    app.add_plugins(DefaultPlugins);

    #[cfg(test)]
    app.add_plugins(MinimalPlugins);

    app.insert_resource(SimulationConfig::default())
        .insert_resource(SimulationClock::default())
        .insert_resource(SimulationWorldState::default())
        .insert_resource(WorldGrid::default())
        .insert_resource(CommandQueue::default())
        .add_systems(Startup, bootstrap::on_startup)
        .add_systems(Startup, init_world_state)
        .add_systems(
            Update,
            (
                step_clock,
                process_command_queue,
                step_world,
                bootstrap::on_update,
            ),
        );

    app
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_app_registers_simulation_resources() {
        let app = build_app();

        assert!(app.world().get_resource::<SimulationConfig>().is_some());
        assert!(app.world().get_resource::<SimulationClock>().is_some());
        assert!(app.world().get_resource::<SimulationWorldState>().is_some());
        assert!(app.world().get_resource::<WorldGrid>().is_some());
        assert!(app.world().get_resource::<CommandQueue>().is_some());
    }
}
