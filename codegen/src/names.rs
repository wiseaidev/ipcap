pub fn name_to_enum_name(name: &str) -> String {
    let before_coma = name.split(",").collect::<Vec<&str>>()[0];
    String::from_iter(
        remove_content_in_brackets(before_coma).chars()
            .filter(|char| char.is_alphabetic())
    )
}

fn remove_content_in_brackets(content: &str) -> String {
    let mut is_open = false;
    let mut result = String::new();

    for char in content.chars() {
        if is_open {
            if char == ')' { is_open = false }
        } else {
            if char == '(' {
                is_open = true
            } else { result.push(char) }
        }
    }

    result
}