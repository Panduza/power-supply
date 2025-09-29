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