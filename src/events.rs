use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NostrEvent {
    pub id: String,
    pub pubkey: String,
    pub created_at: u64,
    pub kind: u32,
    pub tags: Vec<Vec<String>>,
    pub content: String,
    pub sig: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscriptionFilter {
    ids: Option<Vec<String>>,
    kinds: Option<Vec<u32>>,
    authors: Option<Vec<String>>,
    since: Option<u64>,
    until: Option<u64>,
    tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    Event(NostrEvent),
    Req(String, Vec<SubscriptionFilter>), // Subscription request
    Close(String),                        // Close subscription
    Notice(String),                       // Error notices
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    Event(String, NostrEvent), // Event message to client
    Eose(String),              // End of subscription
}
