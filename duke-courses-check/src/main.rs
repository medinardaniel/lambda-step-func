extern crate csv;
extern crate lambda_runtime as lambda;
extern crate serde;
extern crate serde_json;

use csv::Writer;
use lambda::{handler_fn, Context};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Course {
    course_id: u32,
    class_nbr: u32,
    subject: String,
    catalog: String,
    descr: String,
    pi_name: String,
    course_long_descr: String,
    term_descr: String,
}

#[derive(Debug, Serialize)]
struct CsvCourse {
    course_id: u32,
    class_nbr: u32,
    subject: String,
    catalog: String,
    descr: String,
    pi_name: String,
    course_long_descr: String,
    term_descr: String,
}

#[tokio::main]
async fn main() {
    let handler = handler_fn(lambda_handler);
    lambda::run(handler).await("Error running lambda handler");
    Ok(()).expect("REASON")
}

async fn lambda_handler(_: serde_json::Value, _: Context) -> Result<String, lambda::Error> {
    // Declare the data
    let courses = vec![
        Course {
            course_id: 19,
            class_nbr: 4127,
            subject: "AMES".to_string(),
            catalog: "165S".to_string(),
            descr: "THE WORLD OF JAPANESE POP CULT".to_string(),
            pi_name: "Ching,Leo".to_string(),
            course_long_descr: "An examination of modern Japanese culture through a variety of media including literary texts, cultural representations, and films. Different material each year.".to_string(),
            term_descr: "2024 Fall Term".to_string(),
        },
        Course {
            course_id: 22,
            class_nbr: 9034,
            subject: "LIT".to_string(),
            catalog: "290".to_string(),
            descr: "SP TOP INT'L LIT & CULTURE".to_string(),
            pi_name: "Black,Taylor H".to_string(),
            course_long_descr: "Topics in international culture, examined through literary texts, film, and other media. Organized according to trends, topics, and genres.".to_string(),
            term_descr: "2024 Fall Term".to_string(),
        },
        Course {
            course_id: 64,
            class_nbr: 8131,
            subject: "AAAS".to_string(),
            catalog: "89S".to_string(),
            descr: "FIRST-YEAR SEMINAR (TOP)".to_string(),
            pi_name: "Mottahedeh,Negar".to_string(),
            course_long_descr: "Topics vary each semester offered.".to_string(),
            term_descr: "2024 Fall Term".to_string(),
        },
        Course {
            course_id: 75,
            class_nbr: 8987,
            subject: "AAAS".to_string(),
            catalog: "190S".to_string(),
            descr: "SPECIAL TOPICS".to_string(),
            pi_name: "Bello,Damilare Ibrahim".to_string(),
            course_long_descr: "Seminar version of African & African American Studies 190.".to_string(),
            term_descr: "2024 Fall Term".to_string(),
        },
        Course {
            course_id: 81,
            class_nbr: 3091,
            subject: "AAAS".to_string(),
            catalog: "102".to_string(),
            descr: "INTRO AFR-AMER STUDIES".to_string(),
            pi_name: "Li,Stephanie S".to_string(),
            course_long_descr: "A range of disciplinary perspectives on key topics in African American Studies: slavery and abolitionism, theories of race and racism, gender and race, the era of Jim Crow, cultural expressions, political and intellectual thought, African American freedom struggles from the seventeenth through the twentieth centuries, and race and public policy.".to_string(),
            term_descr: "2024 Fall Term".to_string(),
        },
        Course {
            course_id: 82,
            class_nbr: 3093,
            subject: "AAAS".to_string(),
            catalog: "103".to_string(),
            descr: "INTRO TO AFRICAN STUDIES".to_string(),
            pi_name: "Mkhize,Khwezi".to_string(),
            course_long_descr: "A range of disciplinary perspectives on key topics in contemporary African Studies: nationalism and pan-Africanism, imperialism and colonialism, genocide and famine, development and democratization, art and music, age and gender.".to_string(),
            term_descr: "2024 Fall Term".to_string(),
        },
        Course {
            course_id: 176,
            class_nbr: 8585,
            subject: "AAAS".to_string(),
            catalog: "390".to_string(),
            descr: "SPECIAL TOPICS".to_string(),
            pi_name: "Chavis,Benjamin F".to_string(),
            course_long_descr: "Topics vary from semester to semester.".to_string(),
            term_descr: "2024 Fall Term".to_string(),
        },
        Course {
            course_id: 208,
            class_nbr: 6427,
            subject: "AMXTIAN".to_string(),
            catalog: "756".to_string(),
            descr: "AMERICAN CHRISTIANITY".to_string(),
            pi_name: "Dixie,Quinton H".to_string(),
            course_long_descr: "A consideration of the nature of Christianity in America and the history of its development.".to_string(),
            term_descr: "2024 Fall Term".to_string(),
        },
        Course {
            course_id: 227,
            class_nbr: 6435,
            subject: "AMXTIAN".to_string(),
            catalog: "999".to_string(),
            descr: "DIRECTED STUDY".to_string(),
            pi_name: "Staff,Departmental".to_string(),
            course_long_descr: "Directed Study.".to_string(),
            term_descr: "2024 Fall Term".to_string(),
        },
    ];

    // Convert Course structs to CsvCourse structs
    let csv_courses: Vec<CsvCourse> = courses.iter().map(|course| {
        CsvCourse {
            course_id: course.course_id,
            class_nbr: course.class_nbr,
            subject: course.subject.clone(),
            catalog: course.catalog.clone(),
            descr: course.descr.clone(),
            pi_name: course.pi_name.clone(),
            course_long_descr: course.course_long_descr.clone(),
            term_descr: course.term_descr.clone(),
        }
    }).collect();

    // Serialize CsvCourse structs to CSV format
    let mut writer = Writer::from_writer(vec![]);
    for csv_course in csv_courses {
        writer.serialize(csv_course)?;
    }

    // Convert CSV writer to string
    let csv_string = String::from_utf8(writer.into_inner().unwrap()).unwrap();

    Ok(csv_string)
}
