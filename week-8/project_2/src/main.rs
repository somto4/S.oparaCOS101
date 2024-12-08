struct Interviewee {
    name: String,
    years_of_experience: u32,
}

fn main() {
    // List of interviewees
    let interviewees = vec![
        Interviewee {
            name: "Alice".to_string(),
            years_of_experience: 5,
        },
        Interviewee {
            name: "Bob".to_string(),
            years_of_experience: 8,
        },
        Interviewee {
            name: "Charlie".to_string(),
            years_of_experience: 10,
        },
        Interviewee {
            name: "Diana".to_string(),
            years_of_experience: 7,
        },
    ];

    // Find the interviewee with the most experience
    let most_experienced = interviewees.iter().max_by_key(|i| i.years_of_experience);

    match most_experienced {
        Some(interviewee) => println!(
            "The candidate with the highest experience is {} with {} years.",
            interviewee.name, interviewee.years_of_experience
        ),
        None => println!("No interviewees were found."),
    }
}
