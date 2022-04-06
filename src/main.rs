use crate::cards::*;
use crate::player::*;
use std::cmp;
use std::collections::HashMap;
use std::ptr;

// BlackJack and Table structures

pub struct Table<'a> {
    pub players: HashMap<usize, &'a Player>,
    pub cards: HashMap<usize, Deck>,
}

impl<'a> Table<'a> {
    pub fn new(seats: usize) -> Table<'a> {
        Table {
            players: HashMap::with_capacity(seats),
            cards: HashMap::with_capacity(seats),
        }
    }
}

pub struct BlackJack<'a> {
    max_number_of_players: usize,
    table: Table<'a>,
}

// Game logic

impl<'a> BlackJack<'a> {
    pub fn new(max_number_of_players: usize) -> BlackJack<'a> {
        BlackJack {
            max_number_of_players: cmp::min(7, cmp::max(1, max_number_of_players)),
            table: Table::new(max_number_of_players),
        }
    }

    pub fn add_player(&mut self, player: &'a Player, seat: usize) -> usize {
        if self.table.players.contains_key(&seat) {
            // Seat is taken
            return 0;
        }
        if self.max_number_of_players < seat {
            // Seat does not exist
            return 0;
        }
        let free_seats = (1..self.max_number_of_players + 1)
            .filter(|x| !self.table.players.contains_key(&x))
            .collect::<Vec<usize>>();
        if free_seats.is_empty() {
            // There are no free seats available (this should not be possible)
            return 0;
        }
        let player_seat = if seat == 0 {
            // Select the players seat
            free_seats.first().unwrap()
        } else {
            &seat
        };
        self.table.players.insert(player_seat.to_owned(), player);
        self.table
            .cards
            .insert(player_seat.to_owned(), Deck::new_empty());
        player_seat.to_owned()
    }

    pub fn remove_player(&mut self, player: &'a Player, seat: usize) {
        if 0 == seat {
            // No seat provided, remove the player from all seats
            self.table
                .players
                .retain(|_, player_value| ptr::eq(player, *player_value))
        } else {
            // Location provided, remove all items from specific location
            if self.table.players.contains_key(&seat) {
                let found_player = self.table.players.get(&seat).unwrap();
                if ptr::eq(found_player, &player) {
                    self.table.players.remove(&seat);
                }
            }
        }
    }

    pub fn play_game(&mut self, start_bet: f32) {
        // Check if there are players present
        if self.table.players.len() < 1 {
            return;
        }
        // Shuffle cards
        let dealer_deck = Deck::new(3);
        self._play_game(dealer_deck, start_bet)
    }

    fn _play_game(&mut self, mut dealer_deck: Deck, start_bet: f32) {
        let mut seats = Vec::new();
        // Only play with players who have sufficient funds
        for seat in 1..self.max_number_of_players + 1 {
            if self.table.players.contains_key(&seat) {
                // Take the credits
                if self.table.players.get_mut(&seat).unwrap().credits() >= start_bet {
                    // self.table.players.get_mut(&seat).unwrap().take_credits(&start_bet);
                    seats.push(seat);
                }
            }
        }
        // Deal first cards (dealer last)
        let mut dealer = Deck::new_empty();
        for seat in &seats {
            self.table
                .cards
                .get_mut(&seat)
                .unwrap()
                .push(dealer_deck.pop().unwrap());
        }
        dealer.push(dealer_deck.pop().unwrap());
        for seat in &seats {
            self.table
                .cards
                .get_mut(&seat)
                .unwrap()
                .push(dealer_deck.pop().unwrap());
        }
        dealer.push(dealer_deck.pop().unwrap());
        // Players play
        for seat in &seats {
            let mut quit = false;
            while !quit {
                if self.table.cards.get(&seat).unwrap().score() >= 21 {
                    quit = true;
                } else {
                    match self
                        .table
                        .players
                        .get(&seat)
                        .unwrap()
                        .action(seat, &self.table)
                    {
                        PlayerAction::Hit => {
                            self.table
                                .cards
                                .get_mut(&seat)
                                .unwrap()
                                .push(dealer_deck.pop().unwrap());
                        }
                        PlayerAction::Stand => {
                            quit = true;
                        }
                    }
                }
            }
        }
        // Dealers play
        while dealer.score() <= 17 {
            dealer.push(dealer_deck.pop().unwrap());
        }
        // Evaluate all scores and pay bets
        let dealer_score = dealer.score();
        for seat in &seats {
            let mut win_factor = 0;
            let player_score = self.table.cards.get(seat).unwrap().score();
            println!("{:?}", player_score);
            if 21 < player_score {
                // Player busted, no wins
                // table[seat]['player'].message('Busted! You loose.')
                win_factor = 0;
            } else {
                if 21 < dealer_score {
                    // Dealer busted, player wins
                    self.table
                        .players
                        .get(&seat)
                        .unwrap()
                        .message("Dealer busted. You win!");
                    win_factor = 2;
                } else if dealer_score == player_score {
                    // Tie
                    self.table
                        .players
                        .get(&seat)
                        .unwrap()
                        .message("You have the same score as the dealer. No winner.");
                    win_factor = 1;
                } else if dealer_score < player_score {
                    // Player wins
                    self.table.players.get(&seat).unwrap().message("You win!");
                    win_factor = 2;
                } else {
                    // Dealer wins
                    self.table
                        .players
                        .get(&seat)
                        .unwrap()
                        .message("Dealer wins!");
                }
            }
            // self.table.players.get_mut(&seat).unwrap().give_credits(&(start_bet * win_factor as f32));
        }
    }
}
beginner
rust
playing-cards