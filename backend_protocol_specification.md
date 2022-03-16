# Backend Protocol

The backend will be available at https://asdm.seodisparate.com/api .  
The frontend will send http GET/POST requests to the URL with JSON as the body
of the request, and the backend will respond with JSON.

## Requests

1. Request ID and Pairing

```
    {
        "type": "pairing_request",
    }
```

2. Check pairing status

```
    {
        "type": "check_pairing",
        "id": "id given by backend",
    }
```

3. Request Turn action

```
    {
        "id": "id given by backend",
        "type": "place_token",
        "position": 7,
    }
```

4. Request Whose Turn

```
    {
        "id": "id given by backend",
        "type": "whose_turn",
    }
```

5. Disconnect

```
    {
        "id": "id given by backend",
        "type": "disconnect",
    }
```

6. Request Board State:

```
    {
        "id": "id given by backend",
        "type": "request_board_state",
    }
```

7. Request Game State:

```
    {
        "id": "id given by backend",
        "type": "game_state",
    }
```

## Responses

1. Request ID Response

```
    {
        "type": "pairing_response",
        "id": "id set by backend",
        "status": "waiting",
    }
```

```
    {
        "type": "pairing_response",
        "id": "id set by backend",
        "status": "paired",
        "color": "cyan", // or "magenta"
    }
```

If there are too many players such that a new game cannot be feasibly started,
then the back-end will respond with "too\_many\_players".
```
    {
        "type": "pairing_response",
        "status": "too_many_players"
    }
```

2. Check pairing status

```
    {
        "type": "pairing_response",
        "status": "waiting", // or "unknown_id"
    }
```

```
    {
        "type": "pairing_response",
        "status": "paired",
        "color": "magenta", // or "cyan"
    }   
```

3. Request Turn action Response

```
    {
        "type": "place_token",
        "status": "not_paired_yet", // or "accepted", "illegal",
                                    // "not_your_turn", "game_ended",
                                    // "unknown_id"
    }   
```

4. Request Whose Turn Response

```
    {
        "type": "whose_turn",
        "status": "cyan", // or "magenta", "not_paired_yet", "unknown_id",
                          // "game_ended"
    }
```

5. Disconnect Response

```
    {
        "type": "disconnect",
        "status": "ok", // or "unknown_id"
    }
```

6. Request Board State Response

```
    {
        "type": "board_state",
        "status": "in_progress", // or "game_ended"
        "board": [
            "e",
            "e",
            ... // 56 entries in the array where the index of the array
                // correspond to position on the board (0-55). Each entry is
                // either: "e", "c", or "m".
                // "e" -> empty
                // "c" -> cyan
                // "m" -> magenta
        ],
    }
```

```
    {
        "type": "board_state",
        "status": "unknown_id", // or "not_paired"
    }
```

7. Request Game State Response

```
    {
        "type": "game_state",
        "status": "not_paired", // or "in_progress", "unknown_id"
    }
```

Note that the backend will stop keeping track of the game once both players have
successfully requested the Game State once after the game has ended. Thus,
future requests may return "unknown\_id" as the "status".
```
    {
        "type": "game_state",
        "status": "cyan_won", // or "magenta_won", or "draw"
    }
```

Note that if a player has disconnected, the other player will receive a "status"
of "opponent\_disconnected". Future requests will return "unknown\_id".
```
    {
        "type": "game_state",
        "status": "opponent_disconnected", // or "unknown_id"
    }
```

8. Failure Response

When request "type" is not handled by the back-end, it returns with
"invalid\_type".
```
    {
        "type": "invalid_type"
    }
```

When JSON is missing a required value, it returns with "invalid\_json".
```
    {
        "type": "invalid_json"
    }
```

When the back-end hasn't yet implemented handling a specific type, it returns
"unimplemented".
```
    {
        "type": "unimplemented"
    }
```
