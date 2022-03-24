# Final-Project
	~Group Name: TFG (The Four Gamblers)
	~Group member names and NetIDs: Edward Huang(zihaoh3), Zhilan Wang(zhilanw2), Jiyang Xu(jiyangx3),
 
	~Project Introduction:
	  Our goal is to implement a Texas Hold'em game in the Rust language. Hold'em is subdivided into regular table (Cash, cash game), single table
	  tournament (SNG) and multi-table tournament (MTT) gameplay. Although there are many different types of poker, the basic rules of poker usually
	  remain the same. It is a game that tests the mind and strategy. However, in this project, we only simulate the process of the game and find the
	  winner (i.e. the one with the largest hand combination) according to the original official Texas Hold'em rules of the game. We chose this theme
	  because we played a lot of games of this game during spring break and we were a little bit addicted to it (well, a lot), so we wanted to
	  implement it in rust language.

	~System Overview:
	  Our blueprint is to make the game run by writing 4 (maybe more) rust files: simulating the cards (card.rs), simulating the deal (poke.rs),
	  determining the cards (play.rs), and starting the game (game.rs). Here, the role of poker.rs is to create a new pile of cards (except for the
	  joker, so 52 cards in total), and the role of poker.rs is to deal two hands to each player, depending on the number of players (no less than 2
	  and no more than 10), and to set the 5 public cards at the same time. The main purpose of the project is to determine the highest card
	  combination for each player based on the combination of their hand and the public cards, and the player with the highest card combination wins.
	  The role of the start game is to simulate a game process out, and is also the main part.

	~Possible Challenges:
	  This project is basically a challenge for the four of us because none of us have ever worked on a project from scratch (usually MP has a starter
	  code), we didn't choose a difficult project like machine learning or AI, and we're not sure we'll end up with a full game, but we'll do our best.
	  The other potential challenge is that we need to do a better job of handling the poker stacks and dealing cards. "Randomness" is the main thing,
	  and we need to make sure that the dealing and shuffling phases allow the cards to be disordered and completely random.

	~References:
	  https://users.rust-lang.org/t/beginner-terminal-single-player-poker-game-code-review/45509/2
	  https://github.com/NikolayIT/TexasHoldemGameEngine
  
