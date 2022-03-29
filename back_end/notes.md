Use something like:

    curl -X POST 127.0.0.1:1237 -H 'Content-Type: application/json' -d '{"type": "pairing_request"}'

To debug the backend. See protocol specifications for what should work.
