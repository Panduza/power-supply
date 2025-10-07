# Panduza Power Supply Workspace

Ce workspace contient les différents packages du projet Panduza Power Supply :

## Packages

### Server (`server/`)
Le serveur MQTT et le gestionnaire de périphériques pour l'alimentation électrique Panduza.

**Fonctionnalités principales :**
- Broker MQTT intégré
- Gestion des pilotes d'émulation
- Interface web avec Dioxus
- API MCP (Model Context Protocol)

**Pour lancer le server :**
```bash
cargo run --package panduza-power-supply-server
# ou
cd server && cargo run
```

### Client (`client/`) - *À venir*
Interface client pour interagir avec le serveur Panduza Power Supply.

## Développement

### Build du workspace complet
```bash
cargo build
```

### Test du workspace complet
```bash
cargo test
```

### Commandes utiles
```bash
# Lister tous les packages du workspace
cargo workspace

# Build uniquement le server
cargo build --package panduza-power-supply-server

# Build uniquement le client (quand il existera)
# cargo build --package panduza-power-supply-client
```



## Acceptation Tests

- Start a server with an "emulator" instance

- MQTT: With a client tool
    - send "ON" in "power-supply/emulator/control/oe/cmd"
    - send "OFF" in "power-supply/emulator/control/oe/cmd"
    - send "0.5" in "power-supply/emulator/control/voltage/cmd"
    - send "5.23" in "power-supply/emulator/control/voltage/cmd"
    - send "5.23" in "power-supply/emulator/control/current/cmd"
    - send "5.23" in "power-supply/emulator/control/current/cmd"


- MCP: With copilot ()
    - prompt "turn on the power supply"
    - prompt "turn off the power supply"
    - prompt "configure power supply to 2.8V"
    - prompt "configure power supply to 3A"

```json
{
	"servers": {		
		"power_supply": {
			"url": "http://127.0.0.1:3000/power-supply/emulator",
			"type": "http"
		}
	},
	"inputs": []
}
```

- GUI:
    - Tun a gui without device configured = must show an error message
    - Select emulator (must be selected by default)
        - test on/off
        - test to set voltage
        - test to set current
    - Change ON/OFF from MQTT and check that gui show the change
    - Change voltage from MQTT and check that gui show the change
    - Change current from MQTT and check that gui show the change

