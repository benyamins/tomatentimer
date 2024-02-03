mod gui_frameworks;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Wrong number of arguments!");
        println!("tomatentimer <gui-framework>");
    }

    let selected_framework = &args[1];
    match selected_framework.as_str() {
        "iced" => match gui_frameworks::iced::run() {
            Ok(_) => println!("running `iced` frontend"),
            Err(error) => eprintln!("Error runing `iced` frontend {:?}", error)
        },
        "egui" => match gui_frameworks::egui::run() {
            Ok(_) => println!("running `egui` frontend"),
            Err(error) => eprintln!("Error runing `egui` frontend {:?}", error)
        },
        _ => eprintln!("`{}` not suported", selected_framework)
    }
}
