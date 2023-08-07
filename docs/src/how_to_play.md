# How To Play

You can play the game using anything that can consume a JSON API. Even the Swagger UI if you want.

## Order of Operations

Underworld Server keeps track of what a user's "current" player character is. Whenever you perform an action for a game, the current player character for that username is used to perform the action. If there is no current player character, it will be an error.

You can swap your player character at any time, check [here](./player_characters/current.md) for more information.

To do anything you first need to [generate a player character (PC)](./player_characters/generating.md). If this is your first player character, it will automatically be set as your current player.

After you have a PC, you can [generate a game](./game/generating.md). Once you have a game, almost every other action will take that game ID to [perform the action](./game/actions.md).
