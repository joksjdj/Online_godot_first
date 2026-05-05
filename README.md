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
I also forgot to clean out cargo in some of the commits, creating unnececary big files.


# SQL setup
I copied https://github.com/joksjdj/Prosjekt_kortspill/blob/main/.sql as a base.
I made one table called players and made username uniqiue.


# Server setup
I start by copiyng from my other project for the http server to speed things up.
I copied the Cargo.toml and the main.rs.
https://github.com/joksjdj/Prosjekt_kortspill

The server is http based due to Godot limitations.
It uses post for security purposes.
I created a hashing function using rust standard std crate and fixed cors policy issues.

The cors issue was fixed by copilot, I simply copied the error message and fed it to it.

Prompts:
- Access to fetch at 'http://alexanderpi:8080/signup' from origin 'null' has been blocked by CORS policy: Response to preflight request doesn't pass access control check: No 'Access-Control-Allow-Origin' header is present on the requested resource.
- toml dependency