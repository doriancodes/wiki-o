use colored::Colorize;

pub fn header(description: String, header: String) {
    println!(
        "{} {}\n",
        &description.bright_blue(),
        &header.bright_blue().bold()
    )
}

pub fn text(body: String) {
    println!("{}", &body.bright_blue())
}

pub fn added(content: String, file: String) {
    println!(
        "{} {} to {}",
        "added".green().bold(),
        &content.green(),
        &file.green().italic()
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
        "{} \n\n{}\n",
        &description.bright_blue().bold(),
        &text.bright_blue()
    )
}
