# Backend Database Specification

The backend will use sqlite to store all state for storing players (and their
IDs, and paired state), and games in progress.

## Table definitions

```
// enable foreign_keys restrictions
PRAGMA foreign_keys = ON;

// fields should be self explanatory for the players table
CREATE TABLE players (id INTEGER PRIMARY KEY NOT NULL,
                      date_added TEXT NOT NULL,
                      game_id INTEGER,
                      FOREIGN KEY(game_id) REFERENCES games(id) ON DELETE CASCADE);

// "cyan_player" and "magenta_player" should correspond to an existing entry in
// table "players".

// "board" is as explained in backend_protocol_specification.md

// "status" is "0" for cyan's turn, "1" for magenta's turn, "2" for cyan won,
// "3" for magenta won, "4" for draw.

CREATE TABLE games (id INTEGER PRIMARY KEY NOT NULL,
                    cyan_player INTEGER UNIQUE,
                    magenta_player INTEGER UNIQUE,
                    date_added TEXT NOT NULL,
                    board TEXT NOT NULL,
                    status INTEGER NOT NULL,
                    FOREIGN KEY(cyan_player) REFERENCES players (id) ON DELETE SET NULL,
                    FOREIGN KEY(magenta_player) REFERENCES players (id) ON DELETE SET NULL);
```

"date" entries are used for garbage collection of the database. A predefined
length of time will be used to cleanup stale entries. Whenever an entry in the
"games" table is updated, the "date" entry will be updated as well.
