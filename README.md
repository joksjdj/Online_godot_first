## Logg test examen

# Plans
- Make a DB
- Connect DB to a server
- Give server ability to insert and fetch rows
- Connect server to game
- Create login for game
- Create hashing system
- Create leaderboard

I start by copiyng from my other project for the http server to speed things up.
I copied the Cargo.toml and the main.rs.
https://github.com/joksjdj/Prosjekt_kortspill


# error: src refspec main does not match any
I copied this into copilot and found out that some older git versions uses master instead of main.

# wrong number of parameters: 2 expected 1
https://docs.rs/actix-web/latest/actix_web/web/struct.Path.html


# SQL setup
I copied https://github.com/joksjdj/Prosjekt_kortspill/blob/main/.sql as a base.