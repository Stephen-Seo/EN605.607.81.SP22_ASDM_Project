# Backend Database Specification

The backend will use sqlite to store all state for storing players (and their
IDs, and paired state), and games in progress.

## Table definitions

```
// enable foreign_keys restrictions
PRAGMA foreign_keys = ON;

// fields should be self explanatory for the players table

// "phrase" is used to connect players with identical "phrase" text to make it
// easier to connect with the player one wants to play with

CREATE TABLE players (id INTEGER PRIMARY KEY NOT NULL,
                      date_added TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                      game_id INTEGER,
                      phrase TEXT,
                      FOREIGN KEY(game_id) REFERENCES games(id) ON DELETE CASCADE);

// "cyan_player" and "magenta_player" should correspond to an existing entry in
// table "players".

// "board" is as explained in backend_protocol_specification.md

// "status" is "0" for cyan's turn, "1" for magenta's turn, "2" for cyan won,
// "3" for magenta won, "4" for draw.

CREATE TABLE games (id INTEGER PRIMARY KEY NOT NULL,
                    cyan_player INTEGER UNIQUE,
                    magenta_player INTEGER UNIQUE,
                    date_added TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    board TEXT NOT NULL,
                    status INTEGER NOT NULL,
                    turn_time_start TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    FOREIGN KEY(cyan_player) REFERENCES players (id) ON DELETE SET NULL,
                    FOREIGN KEY(magenta_player) REFERENCES players (id) ON DELETE SET NULL);

// "type" is one of the four possible emotes

CREATE TABLE emotes (id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                     type TEXT NOT NULL,
                     date_added TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                     receiver_id INTEGER NOT NULL,
                     FOREIGN KEY(receiver_id) REFERENCES players (id) ON DELETE CASCADE);
```

"date" entries are used for garbage collection of the database. A predefined
length of time will be used to cleanup stale entries. Whenever an entry in the
"games" table is updated, the "date" entry will be updated as well.
