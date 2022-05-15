# Generating a Player Character

To generate a player character you send a JSON POST request to `/api/player_characters/generate`. The payload for the generation request looks like
```JSON
{
  "username": "username", // Whatever username you want to associate with this player character.
  "character_size": "average", // optional, will be random if not specified
  "character_species": "bugbear", // optional, will be random if not specified
  "character_name": "string" // optional, will be empty if not specified
}
```
