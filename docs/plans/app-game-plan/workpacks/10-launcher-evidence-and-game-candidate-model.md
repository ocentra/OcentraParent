# 10 Launcher Evidence And Game Candidate Model

## Target State

Game launchers and launcher-game candidates are first-class states that do not
overclaim active game play.

## Scope

- Steam, Epic, Xbox, Riot, Battle.net, EA, Ubisoft, GOG, Roblox, Minecraft,
  itch.io, and native cloud-game client evidence where available.
- Launcher installed, launcher running, launcher foreground, child game process,
  manifest game id, and launcher-game candidate states.
- Parent-facing explanation for launcher-only versus active game.

## Tests And Proof

- Launcher installed is inventory only.
- Launcher running is runtime only.
- Launcher foreground is launcher foreground only.
- Launcher-game candidate is not known game.
- Known game requires deterministic or classifier-backed child-game proof.

## Done Signal

Launchers can help identify games without becoming fake game-session proof.

Use the standard checklist in [workpacks README](README.md).
