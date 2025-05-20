# Tic Tac Toe

## Requirements

* Console-Based
* A1..C3 grid
* Winner detection
* No computer player

## Design Ideas

* enum `Player` (X, O)
* enum `GameState`
  * `InProgress`
  * `Won(Player)`
  * `Draw`
* enum `ClaimError`
  * `CellAlreadyClaimed`
  * `GameAlreadyWon`
  * `GameAlreadyDrawn`
  * impl `Display` for `ClaimError`
* struct `Board`: Stores the state of the game including content of the cells
  1. `[[Option<Player>; 3]; 3]`
  2. Other options might be:
     * `[Option<Player>; 9]`
     * `Vec<Vec<Option<Player>>>`
     * `Vec<Option<Player>>`
     * `HashMap<Coordinates, Player>`
     * `Vec<(Coordinates, Player)>`
  3. `fn claim(Coordinates, Player) -> Result<GameState, ClaimError>`
  4. Something like serialize, sheet_to_string, etc.; as an alternative, implement the `Into<String>` trait
  5. Helper function with which I get the content of a specific cell. How?
     `fn get_state_of_coordinates(Coordinates) -> Option<Player>`
     Implement the `Index` trait accepting a `Coordinates` object as our indexer
* struct `Game`: Stores the board and current player
  * Add helper functions like `current_player`
* struct `Coordinates`: Identifies a cell on the board (3x3 matrix)
  * struct with row and column as `usize`
  * tuple struct `struct Coordinates(usize, usize)`
  * impl `FromStr`?
  * Create a `new` function that panics if the coordinates are invalid
  * Additional traits?

