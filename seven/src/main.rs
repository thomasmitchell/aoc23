fn main() {
    println!("Hello, world!");
}

enum Rank { Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace }

#[derive(PartialEq)]
enum ScoreType { HighCard, OnePair, TwoPair, ThreeOfKind, FullHouse, FourOfKind, FiveOfKind }

struct Hand { cards: [u32; Rank::Ace as usize + 1] }

impl Hand {
    fn insert(&mut self, card: Rank) {
        self.cards[card as usize] += 1;
    }

    fn scoreType(&self) -> ScoreType {
        let mut score = ScoreType::HighCard;
        for rankCount in self.cards {
            score = match rankCount {
                2 => match score {
                    ScoreType::OnePair => ScoreType::TwoPair,
                    ScoreType::ThreeOfKind => ScoreType::FullHouse,
                    _ => ScoreType::OnePair
                },
                3 => match score {
                    ScoreType::OnePair => ScoreType::FullHouse,
                    _ => ScoreType::ThreeOfKind
                }
                4 => ScoreType::FourOfKind,
                5 => ScoreType::FiveOfKind,
                _ => score,
            }
        }

        return score;
    } 

    fn lessThan(&self, compareTo: &Hand) {

    }
}