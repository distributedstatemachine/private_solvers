-- Up
CREATE TABLE IntentState (
    intent_id UUID PRIMARY KEY,
    status TEXT NOT NULL,
    intent_bid_id TEXT NOT NULL,
    spoke_chain_call JSONB NOT NULL,
    block_number BIGINT NOT NULL
);

-- Down
DROP TABLE IntentState;