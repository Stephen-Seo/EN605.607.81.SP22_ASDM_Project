# Four-Line Dropper: A project developed via Agile methodologies

Note that this project was made for a course for the JHU Engineering for
Professionals program. (Named "Agile Software Development Methods".)

The directory `front_end` holds a Rust project for the "front\_end" code. Or in
other words, it holds the client-side code for the project. The MVP (Minimum
Viable Product) started with just the front\_end, and if all the goals are met,
then the "front\_end" may connect to a "back\_end" for networked gameplay.

The directory `back_end` holds a Rust project for the "back\_end" code. It holds
the server-side code for the project. It is mainly used to handle "Networked
Multiplayer" mode for the game.

The directory `specifications` holds defined specifications. It currently holds
the back-end specifications (database and protocol).

The directory `spreadsheets` hold LibreOffice Calc documents that are
spreadsheets organizing the work. There is a document for User Stories, a
document for the Product Backlogs, and there will be a document for each Sprint.

The directory `retrospectives` holds the retrospectives of Sprints 3 and onward.

The directory `plans` contains the release plans.

The directory `pictures` holds pertinent images to the project. It includes the
"Simple Model" of the project.

## Tags

The git repository is tagged per Sprint and per Day.

## What is Four-Line Dropper?

Four-Line Dropper is a game where two players take turns dropping tokens into
a board. Making a line of four tokens long horizontally, vertically, or
diagonally is the win condition of the game. If the board fills up with no
four-line matches, then the game ends in a draw. The game is called "Four-Line
Dropper" to avoid clashing with the game's original name that is trademarked
(this game is a clone of an existing game).

## Other Things to Know

In multiplayer mode, if a player takes too long to make a move, the game AI
will choose for you automatically (at hardest game AI difficulty).

# Link to a hosted instance

[I have hosted an instance of the front-end/back-end here.](https://asdm.seodisparate.com)
