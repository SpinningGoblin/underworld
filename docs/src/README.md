# Welcome to the World of Underworld Server

Underworld Server is a single-player dungeon crawl played entirely via a JSON web API. Generate a character for yourself to play, generate a game, and perform actions in the game to progress.

## Early Days Warning
This whole thing is very, very early days. Maybe not even an alpha. Here's some of what's missing:
  - Any kind of auth. Right now everything is just usernames, but there's no user data.
  - Progression. There is no "progression" for a character yet beyond just seeing what else pops up.
  - Combat systems. Everything is very early here, and might change at any time.
  - More. Of everything. Spells, items, etc. I've been focusing on everything else and this is only something I do in my spare time.
#### More warnings
  - Breaking changes. The game is early, I expect lots of these.
  - Data disappearing. Goes along with the above, I might need to wipe out data at any point. Or if someone just creates thousands of player characters or games, I'll just dump the entire DB.

## Open API
The current swagger UI for the server can be found at `<BASE_URL>/swagger_ui`, where `<BASE_URL>` is the url of the server, likely where you're reading this. The JSON spec can be found at `<BASE_URL>/spec`.

## Git Repos
The server and the "core" logic are both in different git repositories. If you'd like to spin up your own server, or use the core code for your own thing, go ahead.
  - [Server code](https://github.com/derrickp/underworld_server)
  - [Core code](https://github.com/derrickp/underworld_core)

### Note on URLs in the docs
All urls mentioned in the docs assume they are from the root of the `<BASE_URL>`.
