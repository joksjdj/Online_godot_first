## Logg test examen

# Plans
- Make a DB "done"
- Connect DB to a server "done"
- Give server ability to insert and fetch rows "done"
- Create hashing system "done"
- Improve unique username handling
- Connect server to game
- Create login for game
- Create leaderboard


# Git and GitHub
error: src refspec main does not match any
I copied this into copilot and found out that some older git versions uses master instead of main.


# SQL setup
I copied https://github.com/joksjdj/Prosjekt_kortspill/blob/main/.sql as a base.
I made one table called players and made username uniqiue.


# Server setup
I start by copiyng from my other project for the http server to speed things up.
I copied the Cargo.toml and the main.rs.
https://github.com/joksjdj/Prosjekt_kortspill

The server is http based due to Godot limitations.
It uses post for security purposes.


