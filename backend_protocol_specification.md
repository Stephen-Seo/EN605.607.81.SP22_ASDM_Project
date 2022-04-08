# Backend Protocol

The backend will be available at https://asdm.seodisparate.com/api .  
The frontend will send http POST requests to the URL with JSON as the body
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

4. Disconnect

```
    {
        "id": "id given by backend",
        "type": "disconnect",
    }
```

5. Request Game State:

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
        "type": "pairing_status",
        "status": "waiting", // or "unknown_id"
    }
```

```
    {
        "type": "pairing_status",
        "status": "paired",
        "color": "magenta", // or "cyan"
    }   
```

3. Request Turn action Response

```
    {
        "type": "place_token",
        "status": "not_paired_yet", // or "accepted", "illegal",
                                    // "not_your_turn", "game_ended_draw",
                                    // "game_ended_cyan_won",
                                    // "game_ended_magenta_won", "unknown_id"
        "board": "abcdefg..."       // see protocol 5 response for details
    }   
```

4. Disconnect Response

```
    {
        "type": "disconnect",
        "status": "ok", // or "unknown_id"
    }
```

5. Request Game State Response

```
    {
        "type": "game_state",
        "status": "not_paired", // or "unknown_id", "cyan_turn", "magenta_turn",
                                // "cyan_won", "magenta_won", "draw",
                                // "opponent_disconnected", "internal_error"

        // "board" may not be in the response if "unknown_id" is the status
        "board": "abcdefg..." // 56-char long string with mapping:
                              // a - empty
                              // b - cyan
                              // c - magenta
                              // d - cyan winning piece
                              // e - magenta winning piece
                              // f - cyan placed
                              // g - magenta placed
                              // h - cyan winning and placed piece
                              // i - magenta winning and placed piece
    }
```

Note that the backend will stop keeping track of the game once both players have
successfully requested the Game State once after the game has ended. Thus,
future requests may return "unknown\_id" as the "status".

Note that if a player has disconnected, the other player will receive a "status"
of "opponent\_disconnected". Future requests will return "unknown\_id".
