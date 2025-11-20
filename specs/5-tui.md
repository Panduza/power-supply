# Terminal User Interface

This server **must** propose a Terminal UI.

**User Stories**

- **Utilisateur (Allumer/Éteindre)**: En tant qu'utilisateur, je veux pouvoir allumer et éteindre l'appareil depuis l'interface terminal pour contrôler l'alimentation sans interface graphique.
	- **Critère d'acceptation**: Une commande simple permet d'envoyer l'ordre ON ou OFF et l'interface confirme l'état actuel (allumé / éteint).

- **Utilisateur (Régler la tension)**: En tant qu'utilisateur, je veux pouvoir définir la tension de sortie afin d'ajuster la sortie électrique à la valeur souhaitée.
	- **Critère d'acceptation**: Une commande permet d'envoyer une consigne de tension (ex. 5.00 V) et l'interface affiche la valeur demandée et la valeur actuellement mesurée.

- **Utilisateur (Lire la tension)**: En tant qu'utilisateur, je veux pouvoir interroger la tension actuelle de sortie pour vérifier que la consigne a été appliquée.
	- **Critère d'acceptation**: Une commande renvoie la tension mesurée actuelle et un timestamp de lecture.

- **Utilisateur (Régler le courant)**: En tant qu'utilisateur, je veux pouvoir définir la limite de courant pour protéger la charge et régler le comportement du dispositif.
	- **Critère d'acceptation**: Une commande envoie une consigne de courant (ex. 0.500 A) et l'interface affiche la consigne appliquée.

- **Utilisateur (Lire le courant)**: En tant qu'utilisateur, je veux pouvoir lire le courant de sortie mesuré afin de vérifier la consommation réelle.
	- **Critère d'acceptation**: Une commande renvoie le courant mesuré actuel et un timestamp de lecture.

- **Utilisateur (Retour d'état synthétique)**: En tant qu'utilisateur, je veux pouvoir afficher un état synthétique (power, tension consignée / mesurée, courant consigné / mesuré) pour obtenir un aperçu rapide.
	- **Critère d'acceptation**: Une commande d'état affiche les 4 informations : alimentation (ON/OFF), tension consignée/mesurée, courant consigné/mesuré.

**Notes**: ces stories sont écrites sans détails techniques d'implémentation; elles seront liées ensuite aux fonctions existantes dans la GUI (`on/off`, `set voltage`, `get voltage`, `set/get current`) définies dans `src/server/gui.rs`.

