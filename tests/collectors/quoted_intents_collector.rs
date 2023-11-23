use anyhow::Result;
use artemis_core::types::Collector;
use ethers::types::U256;
use futures::StreamExt;
use khalani_solver::collectors::quoted_intents_collector::QuotedIntentsCollector;
use khalani_solver::inventory::amount::Amount;
use khalani_solver::quote::quoted_intent::QuotedIntent;
use khalani_solver::strategies::types::Event;

#[tokio::test]
async fn test_quoted_intents_collector() -> Result<()> {
    let (quoted_intents_collector, quoted_intents_sender) = QuotedIntentsCollector::new();
    let event_stream = quoted_intents_collector.get_event_stream().await.unwrap();
    let quoted_intent = QuotedIntent::default();
    // TODO: maybe create a trait that generates a random intent for tests.
    let quoted_intent: QuotedIntent = QuotedIntent {
        kai_amount: Amount::from_base_units(U256::one(), 6),
        ..quoted_intent
    };
    quoted_intents_sender.send(quoted_intent.clone()).await?;
    let take = event_stream.take(1);
    let received_events: Vec<Event> = take.collect::<Vec<Event>>().await;
    assert_eq!(received_events, vec![Event::IntentQuoted(quoted_intent)]);
    Ok(())
}
