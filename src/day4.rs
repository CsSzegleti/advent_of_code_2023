use core::num;

pub struct Card {
    pub player_numbers: Vec<u32>,
    pub winning_numbers: Vec<u32>
}

impl Card {
    pub fn from(str: &str) -> Card {
        let raw_numbers: Vec<&str> = str.split(":").collect::<Vec<&str>>()[1].split("|").map(|piece| piece.trim()).collect();
        Card { player_numbers: to_number_vector(raw_numbers[0]), winning_numbers: to_number_vector(raw_numbers[1]) }
    }

    fn get_matching_count(&self) -> u32 {
        let mut count = 0;
        for number in &self.player_numbers  {
            if self.winning_numbers.contains(&number) {
                count +=1;
            }
        }

        count
    }

    fn get_card_worth(&self) -> u32 {
        let count = self.get_matching_count();
        if count == 0 {
            return 0;
        } else {
            return 2_u32.pow(count - 1);
        }
    }
}

fn to_number_vector(str: &str) -> Vec<u32> {
    let replaced = str.replace("  ", " "); 
    replaced.split(" ").map(|s| s.parse::<u32>().unwrap()).collect()
}

fn get_cards(input: &str) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();

    for line in input.lines() {
        cards.push(Card::from(&line));
    }

    cards
}

pub fn get_all_cards_worth(input: &str) -> u32 {
    let mut sum = 0;
    let cards = get_cards(input);
    for card in cards {
        sum += card.get_card_worth()
    }
    sum
}