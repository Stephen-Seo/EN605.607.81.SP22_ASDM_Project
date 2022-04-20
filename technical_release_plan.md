# Disclaimer

This Release Plan was created in April, a few weeks after the project weas
actually started. However, the intended plan for releases has not changed during
the creation of the project.

# Release Plan

## First Stage: Web-app Minimum Viable Product front-end

The first release will be the Minimum Viable Product (MVP) of the front-end.
This front-end will display the game board and other pertinent info.

## Second Stage: Develop the front-end MVP

The front-end MVP will be incrementally and iteratively improved until the
following features are finished:
  - Local Multiplayer (Two people playing the game on the same device)
  - Single Player (One person playing against an AI of varying difficulties)

## Third Stage: Define the back-end protocol

In preparation for Networked Multiplayer for the project, the back-end must
have a protocol defined that both the front-end and back-end must use to
communicate properly.

## Fourth Stage: Define the back-end database

The back-end must store state and keep track of ongoing Networked Multiplayer
games. Thus, it is required to set up the database schema that the back-end will
use for this purpose.

## Fifth Stage: Develop the back-end software

The back-end will be created based on the back-end protocol and back-end
database schema. Note that the protocol and database schema may change as
necessary due to further discoveries and feedback regarding the development of
the back-end

## Sixth Stage: Connect the front-end to the back-end

The front-end will be improved upon to enable the Networked Multiplayer feature
by communicating with the back-end as described by the back-end protocol. This
stage will probably also encounter bugs that may arise in either the back-end or
front-end as this feature is being developed, so bug fixing will probably occur
as well.

## Seventh Stage: Refinement

At this point, all the main features of Four-Line Dropper are implemented.
What's left is bug-fixing and implementation of additional extra features.
