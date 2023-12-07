#[derive(Debug, PartialEq, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}
struct Hand {
    cards: [u8; 5],
    hand_type: HandType,
}

impl HandType {
    fn calculate(hand: &[u8; 5]) -> HandType {
        let mut counts = [0; 15];
        for card in hand {
            counts[*card as usize] += 1;
        }
        let mut counts = counts.into_iter().collect::<Vec<_>>();
        let js = counts.remove(1); // Remove the jokers
        counts.sort_unstable();
        counts.reverse();
        counts[0] += js; // Add the jokers to the biggest group
        match counts[0..5] {
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [2, 1, 1, 1, 0] => HandType::OnePair,
            [2, 2, 1, 0, 0] => HandType::TwoPair,
            [3, 1, 1, 0, 0] => HandType::ThreeOfAKind,
            [3, 2, 0, 0, 0] => HandType::FullHouse,
            [4, 1, 0, 0, 0] => HandType::FourOfAKind,
            [5, 0, 0, 0, 0] => HandType::FiveOfAKind,
            _ => unreachable!(),
        }
    }

    fn strength(&self) -> u8 {
        match self {
            HandType::HighCard => 1,
            HandType::OnePair => 2,
            HandType::TwoPair => 3,
            HandType::ThreeOfAKind => 4,
            HandType::FullHouse => 5,
            HandType::FourOfAKind => 6,
            HandType::FiveOfAKind => 7,
        }
    }
}

impl Hand {
    fn parse(input: &str, j_value: u8) -> Hand {
        let mut cards = [0; 5];
        for (i, card) in input.chars().enumerate() {
            let card = match card {
                'T' => 10,
                'J' => j_value,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                c => c.to_digit(10).unwrap() as u8,
            };
            cards[i] = card;
        }
        let hand_type = HandType::calculate(&cards);
        Hand { cards, hand_type }
    }
}

fn parse(input: &str, j_value: u8) -> Vec<(Hand, usize)> {
    input.lines()
        .map(|line| {
            line.split_once(' ')
                .map(|(hand, bid)| (Hand::parse(hand, j_value), bid.parse().unwrap()))
                .unwrap()
        })
        .collect()
}

fn compare_hands(a: &Hand, b: &Hand) -> std::cmp::Ordering {
    let a_strength = a.hand_type.strength();
    let b_strength = b.hand_type.strength();
    if a_strength == b_strength {
        a.cards.cmp(&b.cards)
    } else {
        a_strength.cmp(&b_strength)
    }
}

fn total_winnings(input: &str, j_value: u8) -> usize {
    let mut hands = parse(input, j_value);
    hands.sort_unstable_by(|(a, _), (b, _)| compare_hands(a, b));
    hands.iter().enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum()
}

fn part1(input: &str) -> usize {
    total_winnings(input, 11)
}

fn part2(input: &str) -> usize {
    total_winnings(input, 1)
}

fn main() {
    let input = include_str!("../../input/day07");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_calculate_hand_type() {
        assert_eq!(HandType::calculate(&[2, 2, 2, 2, 2]), HandType::FiveOfAKind);
        assert_eq!(HandType::calculate(&[2, 2, 2, 2, 3]), HandType::FourOfAKind);
        assert_eq!(HandType::calculate(&[2, 2, 2, 3, 3]), HandType::FullHouse);
        assert_eq!(HandType::calculate(&[6, 2, 3, 3, 3]), HandType::ThreeOfAKind);
        assert_eq!(HandType::calculate(&[2, 3, 3, 4, 4]), HandType::TwoPair);
        assert_eq!(HandType::calculate(&[6, 2, 3, 4, 4]), HandType::OnePair);
        assert_eq!(HandType::calculate(&[6, 2, 3, 4, 5]), HandType::HighCard);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 5905);
    }
}