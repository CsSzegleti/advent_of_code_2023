#[derive(PartialEq)]
enum Color {
    Red,
    Green,
    Blue
}

pub struct Draw {
    color: Color,
    pub pieces: u32,
}
struct Hand {
    pub draws: Vec<Draw>,
}

pub struct Game {
    id: u32,
    hands: Vec<Hand>,
}

impl Hand {
    fn create(string: &str) -> Hand {
        let color_strings: Vec<&str> = string.split(", ").collect();
        Hand {
            draws: color_strings.into_iter().map(Draw::from).collect()
        }
    }

    fn get_power(&self) -> u32 {
        let mut power: u32 = 1;
        for draw in &self.draws {
            power *= draw.pieces;
        }

        power
    }


}

impl Game {
    pub fn create(game_string: &str) -> Game {
        let game: Vec<&str> = game_string.split(':').collect();
        let hand_strings: Vec<&str> = game[1].split(';').collect();

        Game {
            id: game[0].split(' ').collect::<Vec<&str>>()[1].parse::<u32>().unwrap(),
            hands: hand_strings.into_iter().map(Hand::create).collect()
        }
    }

    fn get_max_draw(&self, color: Color) -> Draw {
        let mut max = 0;
        for hand in &self.hands {
            for draw in &hand.draws {
                if draw.pieces > max && draw.color == color {
                    max = draw.pieces;
                }
            }
        }

        Draw {
            color,
            pieces: max
        }
    }

    fn get_min_hand(&self) -> Hand {
        let mut draws: Vec<Draw> = vec![];

        draws.push(self.get_max_draw(Color::Red));
        draws.push(self.get_max_draw(Color::Green));
        draws.push(self.get_max_draw(Color::Blue));

        Hand {
            draws: draws.into_iter().filter( | draw | draw.pieces > 0 ).collect()
        }
    }

    pub fn is_possible(&self, max_red_cubes: u32, max_green_cubes: u32, max_blue_cubes: u32) -> bool {
        let mut is_possible = true;
        let mut hand_idx: usize = 0;
        
        while hand_idx < self.hands.len() && is_possible {
            let mut draw_idx: usize = 0;
            while draw_idx < self.hands[hand_idx].draws.len() && is_possible {
                let draw = &self.hands[hand_idx].draws[draw_idx];
                match draw.color {
                    Color::Blue => {
                        is_possible = draw.pieces <= max_blue_cubes;
                    },
                    Color::Green => {
                        is_possible = draw.pieces <= max_green_cubes;
                    },
                    Color::Red => {
                        is_possible = draw.pieces <= max_red_cubes;
                    }
                }
                draw_idx += 1;
            }
            hand_idx += 1;
        }

        return is_possible;
    }
}

impl Color {
    fn from(color_string: &str) -> Color {
        if color_string.eq_ignore_ascii_case("blue") {
            return Color::Blue;
        } else if color_string.eq_ignore_ascii_case("red") {
            return Color::Red
        } else if color_string.eq_ignore_ascii_case("green") {
            return Color::Green
        } else {
            panic!("No such color")
        }
    }
}

impl Draw {
    fn from(color_string: &str) -> Draw {
        let color_vec : Vec<&str> = color_string.trim().split(' ').collect();
        Draw {
            color: Color::from(color_vec[1]),
            pieces: color_vec[0].parse::<u32>().unwrap()
        }
    }
}

pub fn get_games(input: &str) -> Vec<Game> {
    let mut games = Vec::<Game>::new();

    for line in input.lines() {
        games.push(Game::create(line))
    }

    return games;
}

pub fn sum_possible_games(games: &Vec<Game>) -> u32 {
    let mut sum = 0;

    for game in games {
        if game.is_possible(12, 13, 14) {
            sum += game.id;
        }
    }
    sum
}

pub fn sum_min_hands_value(games: &Vec<Game>,) -> u32 {

    let mut sum = 0;

    for game in games {
        let min_hand = game.get_min_hand();
        sum += min_hand.get_power();
    }

    sum
}