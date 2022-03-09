# How to Test

  - Have Rust installed via rustup.
  - Have the wasm target installed.
      - `rustup target add wasm32-unknown-unknown`
  - Have `trunk` installed and in PATH.
      - `cargo install trunk`
  - Serve the front-end via trunk.
      - `trunk serve`
        - This should serve a local server at `127.0.0.1:8080`

# Other Notes

The webserver hosting this `front_end` is [here](https://asdm.seodisparate.com).

Note that this website may not host the latest version of this `front_end`.
