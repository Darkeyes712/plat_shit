fn main() {
    println!("{:?}", plants("VVVVWWWWVVVV", "Bob"));
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];

    let student_index = students.iter().position(|&s| s == student).unwrap();
    let diagram = diagram.replace("\n", "");
    let first_row_start = 0;
    let second_row_start = diagram.len() / 2;

    let mut result = Vec::new();
    result.push(get_plant_name(
        diagram
            .chars()
            .nth(first_row_start + student_index * 2)
            .unwrap(),
    ));
    result.push(get_plant_name(
        diagram
            .chars()
            .nth(first_row_start + student_index * 2 + 1)
            .unwrap(),
    ));
    result.push(get_plant_name(
        diagram
            .chars()
            .nth(second_row_start + student_index * 2)
            .unwrap(),
    ));
    result.push(get_plant_name(
        diagram
            .chars()
            .nth(second_row_start + student_index * 2 + 1)
            .unwrap(),
    ));

    result
}

fn get_plant_name(c: char) -> &'static str {
    match c {
        'V' => "violets",
        'R' => "radishes",
        'C' => "clover",
        'G' => "grass",
        _ => panic!("Invalid plant character"),
    }
}
