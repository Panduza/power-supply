# 05 - Terminal User Interface (TUI) — Stories

## Contexte
La fonctionnalité TUI permet de piloter le bloc d'alimentation depuis un terminal (CLI) en conservant la traçabilité, la sortie machine‑lisible et des validations d'entrée. Les stories ci‑dessous reprennent et formalisent les scénarios utilisateur essentiels.

---

## User Story 1 — Power Control (P1)
En tant qu'utilisateur, je veux activer et désactiver l'alimentation depuis le terminal, afin de contrôler l'alimentation sans interface graphique.

Acceptance Criteria:
- `power on` met l'appareil en ON, affiche l'état et un timestamp.
- `power off` met l'appareil en OFF, affiche l'état et un timestamp.
- Commandes renvoient un code de sortie non‑zéro en cas d'erreur matérielle.

Test indépendant:
- Exécuter `power on` puis `status --json` et vérifier l'état et le timestamp.

---

## User Story 2 — Set Output Voltage (P2)
En tant qu'utilisateur, je veux définir la tension de sortie pour configurer l'appareil.

Acceptance Criteria:
- `set voltage 5.00` valide l'entrée (format et bornes), applique le setpoint et affiche "Requested: 5.00 V / Measured: X.XX V" avec timestamp.
- Option `--json` fournit un objet JSON structuré contenant `requested`, `measured`, `unit`, `timestamp`.

Test indépendant:
- Exécuter `set voltage 5.00 --json` et valider le schéma JSON.

---

## User Story 3 — Read Output Voltage (P2)
En tant qu'utilisateur, je veux lire la tension mesurée pour vérifier le setpoint.

Acceptance Criteria:
- `get voltage` affiche la valeur mesurée et le timestamp.
- `get voltage --json` renvoie un objet JSON compatible avec la sortie de `set voltage --json`.

---

## User Story 4 — Set Current Limit (P2)
En tant qu'utilisateur, je veux définir la limite de courant pour protéger la charge.

Acceptance Criteria:
- `set current 0.500` valide et applique la limite, affiche "Requested: 0.500 A" avec timestamp.
- Option `--json` renvoie le setpoint appliqué et le timestamp.

---

## User Story 5 — Read Output Current (P2)
En tant qu'utilisateur, je veux lire le courant de sortie pour surveiller la consommation.

Acceptance Criteria:
- `get current` affiche le courant mesuré et le timestamp.
- `get current --json` renvoie un objet JSON structuré.

---

## User Story 6 — Summary Status (P1)
En tant qu'utilisateur, je veux un résumé concis de l'état (power, set/measured voltage/current) pour un aperçu rapide.

Acceptance Criteria:
- `status` affiche: Power (ON/OFF), Voltage (set/measured), Current (set/measured) avec timestamps.
- `status --json` renvoie un schéma stable utilisé par des scripts d'automatisation.

---

## Edge Cases (exigences de comportement)
- Appareil déconnecté: la commande doit échouer proprement, afficher un message d'erreur lisible et renvoyer un code non‑zéro.
- Valeurs hors limites: la CLI doit rejeter les entrées et indiquer les limites valides.
- Concurrence: accès sérialisé à l'API matérielle pour éviter les courses.
- Défauts matériels: présenter des codes d'erreur lisibles et basculer vers un état sûr (par ex. power off) si nécessaire.
