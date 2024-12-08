use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Profession {
    OfficeAdmin,
    Academic,
    Lawyer,
    Teacher,
}

#[derive(Debug)]
struct Staff {
    profession: Profession,
    role: String,
    years_of_experience: u8,
}

fn determine_aps_level(staff: &Staff) -> &'static str {
    match staff.profession {
        Profession::OfficeAdmin => match staff.role.as_str() {
            "Intern" => "APS 1-2",
            "Administrator" => "APS 3-5",
            "Senior Administrator" => "APS 5-8",
            "Office Manager" => "EL1 8-10",
            "Director" => "EL2 10-13",
            "CEO" => "SES",
            _ => "Unknown Position",
        },
        Profession::Academic => match staff.role.as_str() {
            "Research Assistant" => "APS 3-5",
            "PhD Candidate" => "APS 5-8",
            "Post-Doc Researcher" => "EL1 8-10",
            "Senior Lecturer" => "EL2 10-13",
            "Dean" => "SES",
            _ => "Unknown Position",
        },
        Profession::Lawyer => {
            if staff.role.contains("Paralegal") {
                "APS 1-2"
            } else if staff.role.contains("Junior Associate") {
                "APS 3-5"
            } else if staff.role.contains("Associate") {
                if staff.years_of_experience < 5 {
                    "APS 5-8"
                } else if staff.years_of_experience <= 8 {
                    "EL1 8-10"
                } else {
                    "EL2 10-13"
                }
            } else if staff.role.contains("Partner") {
                "SES"
            } else {
                "Unknown Position"
            }
        }
        Profession::Teacher => match staff.role.as_str() {
            "Placement" => "APS 1-2",
            "Classroom Teacher" => "APS 3-5",
            "Snr Teacher" => "APS 5-8",
            "Leading Teacher" => "EL1 8-10",
            "Deputy Principal" => "EL2 10-13",
            "Principal" => "SES",
            _ => "Unknown Position",
        },
    }
}

fn main() {
    let staff = Staff {
        profession: Profession::Lawyer,
        role: "Associate".to_string(),
        years_of_experience: 6,
    };

    let aps_level = determine_aps_level(&staff);
    println!("The staff's APS level is: {}", aps_level);
}
