# Module: MQTT Server

## Functional Requirements

- Handles MQTT communication for power supply control and monitoring
- Subscribes to command topics for output enable, voltage, and current
- Publishes state, voltage, and current updates to relevant topics
- Integrates with device drivers to execute commands received via MQTT

## Technical Requirements

- Uses `pza_toolkit::rumqtt` for MQTT client and broker
- Topics are managed via the shared `Topics` struct for consistency
- Async event loop for handling MQTT events and updating state


## Auto Testing Scenarios

HERE ARE THE TEST AGENT CAN CODE TO TEST THE MODULE FUNCTIONALITIES.

## Manual Testing Scenarios

HERE I WILL INSERT THE TESTING MANUAL SCENARIOS JUST TO GIVE CONTEXT TO THE AGENT. AGENT MUST NOT CODE THEM.

- [x] Enable Output via MQTT
To enable output, publish the following JSON payload to the state command topic:

```bash
mosquitto_pub -h 127.0.0.1 -p 1883 -t "power-supply/emulator/state/cmd" -m '{"pza_id":"A","state":"ON"}'
```

Check the same payload appear in `power-supply/emulator/state`

- [x] Disable Output via MQTT
To disable output, publish the following JSON payload to the state command topic:

```bash
mosquitto_pub -h 127.0.0.1 -p 1883 -t "power-supply/emulator/state/cmd" -m '{"pza_id":"B","state":"OFF"}'
```

Check the same payload appear in `power-supply/emulator/state`

- [x] Push some errors on enum values

```bash
mosquitto_pub -h 127.0.0.1 -p 1883 -t "power-supply/emulator/state/cmd" -m '{"pza_id":"B","state":"OF"}'
```

Check for error in `power-supply/emulator/error`, with the same pza_id and describing this error.

- [x] Push some errors on keys

```bash
mosquitto_pub -h 127.0.0.1 -p 1883 -t "power-supply/emulator/state/cmd" -m '{"pza_id":"B","stat":"OFF"}'
```

- [x] Push some errors on pza_id

```bash
mosquitto_pub -h 127.0.0.1 -p 1883 -t "power-supply/emulator/state/cmd" -m '{"pzad":"B","stat":"OFF"}'
```

- [x] Set Voltage via MQTT
To set the voltage, publish the following JSON payload to the voltage command topic:

```bash
mosquitto_pub -h 127.0.0.1 -p 1883 -t "power-supply/emulator/voltage/cmd" -m '{"pza_id":"C","voltage":"12.0"}'
```

- [x] Set Current via MQTT
To set the current, publish the following JSON payload to the current command topic:

```bash
mosquitto_pub -h 127.0.0.1 -p 1883 -t "power-supply/emulator/current/cmd" -m '{"pza_id":"D","current":"2.5"}'
```

- [ ] Handle driver panic

```bash
mosquitto_pub -h 127.0.0.1 -p 1883 -t "power-supply/emulator/voltage/cmd" -m '{"pza_id":"C","voltage":"9999.9999"}'
```

