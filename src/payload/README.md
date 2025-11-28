# Module: payload

## Functional Requirements

- Define and serialize/deserialize the `PowerState` enum representing ON/OFF states.
- Provide the `PowerStatePayload` struct to encapsulate a power state message with a unique PZA identifier.
- Define and serialize/deserialize a `VoltagePayload` struct on the same model as `PowerStatePayload` for voltage values.
- Define and serialize/deserialize a `CurrentPayload` struct on the same model as `PowerStatePayload` for current values.
- Each payload type (`PowerStatePayload`, `VoltagePayload`, `CurrentPayload`) must have its own dedicated source file in this module.
- Support creation of new payloads and conversion to JSON bytes for transmission.

## Technical Requirements

- Uses the `serde` crate for serialization/deserialization.
- Uses the `bytes` crate for efficient byte handling.
- Relies on a `generate_pza_id` function from the parent module for unique ID generation.

## Auto Testing Scenarios

- Test serialization and deserialization of `PowerState` and `PowerStatePayload` to/from JSON.
- Test creation of a new `PowerStatePayload` and verify the `pza_id` is set.
- Test the `to_json_bytes` method returns valid JSON bytes.
- Test error handling when serialization fails.

## Manual Testing Scenarios

- [ ] Create a payload and inspect the JSON output for correctness.
- [ ] Simulate sending and receiving a payload between client and server, ensuring the `pza_id` is echoed.
- [ ] Manually test with invalid or missing fields to verify error handling.
