# Dynamic Websocket Server

## Tech

- Rust driven WebSocket

  - may use Rocket?
  - may use Yew?
  - may use ws-rs?

## General Idea

### Client (Sapper/Svelte)

- configure socket response

  - [sampleRate] minimum delay between server messages
  - [valueRange] min | max values for the range of the value returned

- Subscribe to an array of imaginary Device [id]'s
- when receiving message of values from websocket it draws a canvas for each id


### WS-Server (Rust/Rocket)

- for Each Client

  - set [sampleRate] & [valueRange]
  - for each imaginary [id]

    - every [sampleRate] seconds

      - send random value in [valueRange] to client
