use crate::util::parse_lines;
use std::cmp::Ordering;
use std::str::FromStr;

pub struct Solution;

impl crate::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let mut hands: Vec<HandBid> = parse_lines(&input).collect();
        hands.sort_by(|a, b| a.hand.cmp(&b.hand));
        sum_winnings(hands)
    }

    fn solve_2(&self, input: String) -> String {
        let mut hands: Vec<HandBid> = parse_lines(&input).collect();
        for hand in &mut hands {
            hand.hand.translate_to_part_2();
        }
        hands.sort_by(|a, b| a.hand.cmp(&b.hand));
        sum_winnings(hands)
    }
}

fn sum_winnings(hands: Vec<HandBid>) -> String {
    hands
        .into_iter()
        .enumerate()
        .map(|(i, hb)| {
            let rank = (i + 1) as u32;
            rank * hb.bid
        })
        .sum::<u32>()
        .to_string()
}

struct HandBid {
    hand: Hand,
    bid: u32,
}

impl FromStr for HandBid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand_str, bid_str) = s.split_once(' ').unwrap();
        let cards = hand_str
            .chars()
            .map(card_value)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let hand = Hand {
            cards,
            joker_rule: false,
        };
        let bid = bid_str.parse().unwrap();
        Ok(HandBid { hand, bid })
    }
}

fn card_value(c: char) -> u8 {
    match c {
        '2' => 0,
        '3' => 1,
        '4' => 2,
        '5' => 3,
        '6' => 4,
        '7' => 5,
        '8' => 6,
        '9' => 7,
        'T' => 8,
        'J' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("card {c}"),
    }
}

#[derive(Eq, PartialEq)]
struct Hand {
    cards: [u8; 5],
    joker_rule: bool,
}

impl Hand {
    fn translate_to_part_2(&mut self) {
        self.joker_rule = true;
        for i in 0..5 {
            match self.cards[i].cmp(&9) {
                Ordering::Less => self.cards[i] += 1,
                Ordering::Equal => self.cards[i] = 0,
                Ordering::Greater => {}
            }
        }
    }

    fn get_type(&self) -> Type {
        let mut groups = [0u8; 13];
        for card in self.cards {
            groups[card as usize] += 1;
        }

        if self.joker_rule {
            let j = std::mem::take(&mut groups[0]);
            let highest = groups.iter_mut().max().unwrap();
            *highest += j;
        }

        let highest = std::mem::take(groups.iter_mut().max().unwrap());
        let second = groups.into_iter().max().unwrap();

        if highest == 5 {
            Type::FiveOfKind
        } else if highest == 4 {
            Type::FourOfKind
        } else if highest == 3 && second == 2 {
            Type::FullHouse
        } else if highest == 3 {
            Type::ThreeOfKind
        } else if highest == 2 && second == 2 {
            Type::TwoPair
        } else if highest == 2 {
            Type::OnePair
        } else {
            Type::HighCard
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }

        let type_cmp = self.get_type().cmp(&other.get_type());
        if type_cmp == Ordering::Equal {
            self.cards.cmp(&other.cards)
        } else {
            type_cmp
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day7::Solution;
    use crate::init_test_logging;

    #[test]
    fn ex2() {
        use crate::Solution;
        init_test_logging();

        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            .to_string();

        let res = Solution.solve_2(input);

        assert_eq!("5905", res);
    }
}
