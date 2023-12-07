use counter::Counter;

pub fn part1() -> i64 {
    let input = include_str!(r"inputs\day7.txt");
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|s| Hand {
            hand: s.split_once(' ').unwrap().0.to_string(),
            hand_type: get_hand_type(s.split_once(' ').unwrap().0),
            bet: s.split_once(' ').unwrap().1.parse().unwrap(),
        })
        .collect();
    hands.sort_unstable();
    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, h)| acc + h.bet * (1 + i as i64))
}

pub fn part2() -> i64 {
    let input = include_str!(r"inputs\day7.txt");
    let mut hands: Vec<Hand2> = input
        .lines()
        .map(|s| Hand2 {
            hand: s.split_once(' ').unwrap().0.to_string(),
            hand_type: get_hand_type2(s.split_once(' ').unwrap().0),
            bet: s.split_once(' ').unwrap().1.parse().unwrap(),
        })
        .collect();
    hands.sort_unstable();
    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, h)| acc + h.bet * (1 + i as i64))
}

fn get_hand_type(hand: &str) -> HandType {
    let counts: Counter<char> = hand.chars().collect();
    let freqs_in_order = counts.most_common_ordered();
    if counts.len() == 1 {
        return HandType::FiveOfAKind;
    } else if freqs_in_order[0].1 == 4 {
        return HandType::FourOfAKind;
    } else if freqs_in_order[0].1 == 3 && freqs_in_order[1].1 == 2 {
        return HandType::FullHouse;
    } else if freqs_in_order[0].1 == 3 {
        return HandType::ThreeOfAKind;
    } else if freqs_in_order[0].1 == 2 && freqs_in_order[1].1 == 2 {
        return HandType::TwoPair;
    } else if freqs_in_order[0].1 == 2 {
        return HandType::OnePair;
    }
    HandType::HighCard
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq)]
struct Hand {
    hand: String,
    hand_type: HandType,
    bet: i64,
}

fn map_chars_to_hand_value(c: char) -> i64 {
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
        _ => -1,
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        }
        for (c_lhs, c_rhs) in self.hand.chars().zip(other.hand.chars()) {
            let v_lhs = map_chars_to_hand_value(c_lhs);
            let v_rhs = map_chars_to_hand_value(c_rhs);
            if v_lhs < v_rhs {
                return std::cmp::Ordering::Less;
            } else if v_lhs > v_rhs {
                return std::cmp::Ordering::Greater;
            }
        }
        std::cmp::Ordering::Equal
    }
}

fn get_hand_type2(hand: &str) -> HandType {
    let mut counts: Counter<char> = hand.chars().collect();
    let j_count = counts.remove(&'J').unwrap_or(0);
    let freqs_in_order = counts.most_common_ordered();
    if j_count == 5 || freqs_in_order[0].1 + j_count == 5 {
        return HandType::FiveOfAKind;
    } else if freqs_in_order[0].1 + j_count == 4 {
        return HandType::FourOfAKind;
    } else if freqs_in_order[0].1 + j_count == 3 && freqs_in_order[1].1 == 2 {
        return HandType::FullHouse;
    } else if freqs_in_order[0].1 + j_count == 3 {
        return HandType::ThreeOfAKind;
    } else if freqs_in_order[0].1 == 2 && freqs_in_order[1].1 + j_count == 2 {
        return HandType::TwoPair;
    } else if freqs_in_order[0].1 + j_count == 2 {
        return HandType::OnePair;
    }
    HandType::HighCard
}

#[derive(PartialEq, Eq)]
struct Hand2 {
    hand: String,
    hand_type: HandType,
    bet: i64,
}

fn map_chars_to_hand_value2(c: char) -> i64 {
    match c {
        'J' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => -1,
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        }
        for (c_lhs, c_rhs) in self.hand.chars().zip(other.hand.chars()) {
            let v_lhs = map_chars_to_hand_value2(c_lhs);
            let v_rhs = map_chars_to_hand_value2(c_rhs);
            if v_lhs < v_rhs {
                return std::cmp::Ordering::Less;
            } else if v_lhs > v_rhs {
                return std::cmp::Ordering::Greater;
            }
        }
        std::cmp::Ordering::Equal
    }
}
