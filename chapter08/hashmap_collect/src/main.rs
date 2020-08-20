use std:: collections::HashMap;

fn main() {
    let teams = vec![
        String::from("Blue"),
        String::from("Yellow"),
    ];
    let initial_scores = vec![
        10,
        50,
    ];

    let mut score: HashMap<_, _> =
        teams.iter().zip(initial_scores.iter()).collect();

    for (t, s) in &score {
        println!("{}: {}", t, s);
    }
}
