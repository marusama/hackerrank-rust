use std::io;
use std::vec;
use std::fmt;

pub fn main() {
    let mut s_triplet_a = String::new();
    let mut s_triplet_b = String::new();

    io::stdin().read_line(&mut s_triplet_a).ok().expect("read error");
    io::stdin().read_line(&mut s_triplet_b).ok().expect("read error");

    let triplet_a: Vec<i32> = s_triplet_a.trim().split(' ').map(|x| { x.parse().ok().expect("parse error") }).collect();
    let triplet_b: Vec<i32> = s_triplet_b.trim().split(' ').map(|x| { x.parse().ok().expect("parse error") }).collect();

    let score = compare(&triplet_a, &triplet_b);

    println!("{}", score);
}

#[derive(Debug, Eq, PartialEq)]
struct Score {
    a: i32,
    b: i32
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.a, self.b)
    }
}

fn compare(va: &Vec<i32>, vb: &Vec<i32>) -> Score {
    let mut score = Score { a: 0, b: 0 };

    for i in 0..va.len() {
        let a = va[i];
        let b = vb[i];
        if a > b {
            score.a += 1;
        }
        if b > a {
            score.b += 1;
        }
    }

    score
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_compare() {
        // arrange
        let a = vec![5, 6, 7];
        let b = vec![3, 6, 10];

        // act
        let score = super::compare(&a, &b);

        // assert
        assert_eq!(super::Score { a: 1, b: 1 }, score);
    }
}