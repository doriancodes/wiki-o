use colored::Colorize;

pub fn header(description: String, header: String) {
    println!(
        "{}",
        format!("{} {}\n", &description.yellow(), &header.yellow().bold())
    )
}

pub fn text(body: String) {
    println!("{}", format!("{}", &body.yellow()))
}

pub fn added(content: String, file: String) {
    println!(
        "{}",
        format!(
            "{} {} to {}",
            "added".green().bold(),
            &content.green(),
            &file.green().italic()
        )
    )
}

pub fn deleted(is_file: bool, resource: String) {
    let resource_type = if is_file { "file:" } else { "directory:" };
    println!(
        "{} {} {}",
        "Deleted".red().italic(),
        resource_type.red().italic(),
        resource.red()
    )
}

pub fn show_config(description: String, text: String) {
    println!(
        "{}",
        format!("{} \n\n{}\n", &description.yellow().bold(), &text.yellow())
    )
}
