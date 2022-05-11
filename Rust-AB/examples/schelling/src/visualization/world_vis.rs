use crate::model::world::World;
use crate::Patch;
use rust_ab::bevy::prelude::Commands;
use rust_ab::engine::agent::Agent;
use rust_ab::engine::fields::field::Field;
use rust_ab::engine::fields::sparse_object_grid_2d::SparseGrid2D;
use rust_ab::engine::schedule::Schedule;
use rust_ab::engine::state::State;
use rust_ab::visualization::agent_render::AgentRender;
use rust_ab::visualization::asset_handle_factory::AssetHandleFactoryResource;
use rust_ab::visualization::fields::object_grid_2d::RenderObjectGrid2D;
use rust_ab::visualization::simulation_descriptor::SimulationDescriptor;
use rust_ab::visualization::visualization_state::VisualizationState;

#[derive(Clone)]
pub struct WorldVis;

impl VisualizationState<World> for WorldVis {
    fn on_init(
        &self,
        commands: &mut Commands,
        sprite_factory: &mut AssetHandleFactoryResource,
        state: &mut World,
        _schedule: &mut Schedule,
        _sim: &mut SimulationDescriptor,
    ) {
        state.field.update();
        SparseGrid2D::<Patch>::init_graphics_grid(sprite_factory, commands, state);
    }

    fn before_render(
        &mut self,
        _state: &mut World,
        _schedule: &Schedule,
        _commands: &mut Commands,
        _sprite_factory: &mut AssetHandleFactoryResource,
    ) {
    }

    fn get_agent_render(
        &self,
        _agent: &Box<dyn Agent>,
        _state: &World,
    ) -> Option<Box<dyn AgentRender>> {
        None
    }

    fn get_agent(
        &self,
        _agent_render: &Box<dyn AgentRender>,
        _state: &Box<&dyn State>,
    ) -> Option<Box<dyn Agent>> {
        None
    }
}

impl WorldVis {}
