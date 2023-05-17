# Rusty Euchre

## How to Use
### Test
`cargo test`

## Game
Each `Game` has:
- `Deck`
- `Player`x4
### Deck
The `Deck` includes 24 `Cards` with the following `Ranks` and `Suits`:
- Nine, Ten, Jack, Queen, King, Ace
- Hearts, Diamonds, Clubs, Spades 
### Player
Each `Player` has a hand of cards starting with 5
#### Hand
Each `Hand` has 5 `Cards` to start and will decrease by 1 for each `Trick` played


## Order of operations

### Beginning of game
- [x] Create new `Game` 
  - [x] Create `Deck`
  - [x] Populate `Deck` with 24 `Cards`
  - [x] Shuffle the `Cards` in the `Deck`
- [x] Create 4 `Players`
  - [x] Create a `Hand` for each player (empty at first)
  - [x] Assign each player to a team
- [x] Deal 5 random `Cards` to `Hand` of each `Player`
- [x] Determine which `Player` goes first
- [x] Reveal the top `Card` off of the `Deck`
  - [x] Starting with the second `Player`, determine if the `Player` would like "order up" the `Card`.
  - [x] If a `Player` "orders up" this `Card`:
    - [x] The dealer takes the `Card` into their hand and discards a `Card`
    - [x] Skip to "Trump is designated"
- [x] If no `Player` "orders up" this `Card`:
  - [x] Each `Player` is given the option to select trump, besides the suit of the top `Card`.
  - [x] If a `Player` selects trump:
    - [x] Skip to "Trump is designated"
- [x] If no `Player` chooses trump, re-deal
- [x] Trump is designated
- [x] The makers and defenders are designated
- [x] Maker can decide to "Go Alone"
- [x] `Players` play `Cards` until no `Cards` are left in `Player` hands
  - [x] The `Player` to the left of the Dealer starts
  - [x] If a `Card` has already been played, the `Player` must follow suit
  - [x] Otherwise, the `Player` can choose any `Card` they wish
  - [x] A `Trick's` winner is determined by the `Card` with the highest value
- Gain points:
  - [x] If the makers win 3 or more `Tricks`, they earn 1 point
  - [x] If the makers win all 5 `Tricks`, they earn 2 points
  - [x] If the makers win all 5 `Tricks` and a member of the team opted to "go alone", they earn 4 points
  - [x] If the defenders win 3 or more `Tricks`, they earn 2 points
- If a team has 10 points or more:
  - The game is over
- If neither team has 10 points or more:
  - Shuffle the `Cards`
  - The next `Player` is now the Dealer
  - Return to the step: "Deal 5 random `Cards` to `Hand` of each `Player`"



## Strategies

### Card Values
Each `Card` will be assigned a value based on how many `Tricks` it can win.

For example, if trump is Hearts, the Jack of Hearts will win 100% of the time. So it would be worth 1 point.

If trump is hearts, the Nine of Diamonds would only be able to win a `Trick` if nobody else has a Diamond


#### Probability of "At Least One" Rule

If we want to know what the probability of a 9 of Hearts winning a `Trick` when trump is Diamonds:

- If the 9 of Hearts is led, there are 11/23 `Cards` that can beat it.
  - There is a 45.83% chance of each of the `Players` on the opposite team of having a higher `Card`.
  - There is a 54.17% chance of them NOT having a `Card` 
  - To get the probability of this `Card` being beat, we multiply the chances of each of the 2 `Players` probability of having one of them:
    - 54.17 * 54.17 =  29.34% - Probability of the 9 of Hearts WINNING a trick
    - 100 - 29.34 = 70.64% - Probability of the 9 of Hearts LOSING a trick
  - There is a 70.64% chance that one of your opponents has a higher `Card`
- If it is not led, there are 23/23 `Cards` that can beat it.
- 

### Which `Card` to Play
#### Beat your teammates high `Card`
#### Play trump to draw out trump from other `Players` `Hands`
#### When possible, lead "Big"

### Ordering Up Trump
- Only order up trump if the dealer is you or your teammate

### Calling Trump
#### Risky Calls
##### Count on your teammate for at least 1 `Trick`
#### Safe Calls
##### "Call it" with 3 trump and an off `Ace`

### Going Alone