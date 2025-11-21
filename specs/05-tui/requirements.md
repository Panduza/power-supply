# 05 - Terminal User Interface (TUI) — Requirements

## Functional Requirements

- FR-001: Fournir une CLI TUI exposant les contrôles principaux: `power on|off`, `set voltage <V>`, `get voltage`, `set current <A>`, `get current`, `status`.
- FR-002: Valider les formats numériques et les bornes avant d'appeler les APIs matérielles.
- FR-003: Supporter une sortie machine‑lisible via l'option `--json` pour toutes les commandes de mesure/état.
- FR-004: Persister des logs utilisateur lisibles pour l'audit (format texte) sans exposer d'informations sensibles.
- FR-005: Fournir des codes d'erreur et messages clairs pour les échecs de communication matérielle.
- FR-006: Inclure un timestamp ISO8601 (`last_updated`) pour mesures et changements d'état.

## Platform / Cross-cutting

- FR-PLATFORMS: Supporter Linux, macOS, Windows. Documenter toute limitation plateforme.
- FR-CLI: Tous les comportements utilisateurs exposés par le TUI doivent être accessibles via la CLI, avec `--help` détaillé.
- FR-MCP: Si une intégration est exposée, fournir un contrat MCP et des tests de contrat (à préciser si nécessaire).
- FR-TEST-FIRST: Les tests doivent être spécifiés ici et implémentés avant l'implémentation (tests unitaires/integration CLI).

## Tests à implémenter (exigés par la règle PR-002)
- Test `power_on_off`: exécuter `power on` puis `power off` / valider `status --json`.
- Test `set_voltage_bounds`: vérifier que les valeurs invalides sont rejetées et les valeurs valides appliquées.
- Test `get_voltage_json_schema`: vérifier que `get voltage --json` respecte le schéma.
- Test `status_json_schema`: valider la stabilité du format JSON retourné par `status --json`.

## Project Rules (rappel — must follow)
- PR-001 (Docs First): Chaque commande doit avoir une entrée dans `docs/` et dans `--help`.
- PR-002 (Test Coverage): Chaque story a au moins un test automatisé CLI.
- PR-004 (Error Handling): Aucun panic non géré, erreurs user‑friendly + codes de sortie non‑zéro.
- PR-007 (Formatting & Linting): `cargo fmt` et `cargo clippy` doivent passer.
- PR-008 (CLI Machine-Readable Output): Les commandes de mesure doivent offrir `--json`.

## Key Entities
- DeviceState: `{ power: "on"|"off", voltage_set: f64, current_set: f64, voltage_measured: f64, current_measured: f64, last_updated: iso8601 }`.
- Command: Représentation d'une commande CLI parsée, ses arguments, verdict de validation.
- Measurement: `{ value: f64, unit: "V"|"A", timestamp: iso8601 }`.

## Success Criteria (mesurables)
- SC-001: Réponse < 1s pour appareils locaux lors d'opérations `power` et `status`.
- SC-002: `set voltage` et `set current` renvoient les valeurs mesurées avec timestamp dans >95% des runs d'intégration.
- SC-003: `status --json` conforme à un schéma stable utilisable par des scripts.

## Notes d'implémentation
- Les bornes exactes (min/max voltage/current) doivent être lues depuis la configuration du driver (`drivers::...`) ou documentées dans `requirements.md` si le matériel impose des limites.
- Les logs d'audit peuvent être gérés par le composant `server::services` ou équivalent; éviter d'écrire des secrets.
