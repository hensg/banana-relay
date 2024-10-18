# Banana Nostr Relay

Shitty nostr relay basic implementation for learning a bit of nostr.

### Developing
`nix develop`

### Running
Run the server: `RUST_LOG=info cargo run`.

### Testing

Requirements:
- `websocat`

Run client 1:
```
websocat ws://localhost:3030/relay
```

Run client 2 (in a new tab):
```
websocat ws://localhost:3030/relay
```


1. Send Event:
```
{"Event": {"id": "1", "pubkey": "abcd1234", "created_at": 1680091234, "kind": 1, "tags": ["banana4fun"], "content": "We all love bananas", "sig": "signature"}}
```

2. Send subscribe:
```
{"Req": ["subid123", [{"kinds": [1], "authors": ["pubkey123"]}] ]}
```

3. Close subscription:
```
{"Close": "subid123"}
```
