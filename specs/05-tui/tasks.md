# 05 - Terminal User Interface (TUI) — Tasks

But: découper le travail en petites tâches PR‑friendly, mapées sur les stories et tests.

1. Task 01 — Spec: Stories & Requirements
   - Créer `specs/05-tui/stories.md` et `requirements.md`. (PR: docs only)
   - Acceptance: fichiers ajoutés et revus.

2. Task 02 — CLI skeleton & parsing
   - Ajouter un sous‑module `cli` (ou étendre `main.rs`) pour parser les commandes: `power`, `set voltage`, `get voltage`, `set current`, `get current`, `status`.
   - Utiliser `clap` ou un parseur existant du projet.
   - Implémenter `--help` et `--json` flag global.
   - Tests: unitaires pour parsing et validation d'arguments.

3. Task 03 — Implement `power on|off`
   - Implémenter l'appel vers l'API device (driver) pour changer l'état.
   - Retourner message texte et JSON selon flag.
   - Tests d'intégration simulées avec driver `emulator`.

4. Task 04 — Implement `set/get voltage`
   - Validation des bornes et format (ex: 2 décimales pour V).
   - Appel au driver, lecture de la mesure, sortie texte/JSON.
   - Tests: unité (validation), intégration (`emulator`), schéma JSON.

5. Task 05 — Implement `set/get current`
   - Comme voltage mais pour le courant.
   - Tests: unité + intégration.

6. Task 06 — Implement `status` summary
   - Aggréger `DeviceState` et formater sortie texte/JSON.
   - Tests: valider inclusion de timestamps et stabilité du schéma.

7. Task 07 — Logging & Audit
   - Ajouter persistance des commandes et réponses (fichier de log rotatif ou système existant).
   - S'assurer que les logs n'exposent pas de secrets.
   - Tests: vérifier écriture de log pour commandes critiques.

8. Task 08 — Error handling and edge cases
   - Tests pour appareil déconnecté, valeurs hors limites, défaut matériel.
   - Documenter comportements d'erreur dans `docs/`.

9. Task 09 — Docs & CI
   - Documenter chaque commande dans `docs/` (format demandé par PR-001).
   - Ajouter tests CLI aux workflows CI, garantir `cargo fmt` et `cargo clippy`.

10. Task 10 — Release notes / Migration
   - Si breaking changes CLI: rédiger notes de migration et bump major.

Notes PR strategy:
- Chaque task 02–06 idéalement un PR séparé (petite surface), incluant tests et docs minimales.
- Tâches 3–6 doivent fournir `--json` dès la première itération pour répondre à PR-008.

Références:
- Spécification source: `specs/5-tui.md` (migration vers ce dossier).
