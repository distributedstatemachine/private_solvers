use artemis_core::engine::Engine;
use tracing::info;

pub mod action_confirmation_collector;

pub async fn run_engine<Event, Action>(engine: Engine<Event, Action>)
where
    Event: Send + Clone + 'static + std::fmt::Debug,
    Action: Send + Clone + 'static + std::fmt::Debug,
{
    if let Ok(mut set) = engine.run().await {
        while let Some(res) = set.join_next().await {
            info!("Result: {:?}", res);
        }
    }
}
