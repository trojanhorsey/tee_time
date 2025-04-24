use std::env;

pub fn main_menu_from_args() {
    eprintln!("📺 Inside main_menu_from_args()");

    // Always show the menu:
    eprintln!(
        r#"
        ████████╗███████╗███████╗████████╗██╗███╗   ███╗███████╗
        ╚══██╔══╝██╔════╝██╔════╝╚══██╔══╝██║████╗ ████║██╔════╝
           ██║   █████╗  █████╗     ██║   ██║██╔████╔██║█████╗  
           ██║   ██╔══╝  ██╔══╝     ██║   ██║██║╚██╔╝██║██╔══╝  
           ██║   ███████╗███████║   ██║   ██║██║ ╚═╝ ██║███████╗
           ╚═╝   ╚══════╝╚══════╝   ╚═╝   ╚═╝╚═╝     ╚═╝╚══════╝
        "#
    );

    eprintln!("\nSelect a TEE backend:");
    eprintln!("[1] Trustonic");
    eprintln!("[q] Quit\n");

    // Only take action if an argument was passed:
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return;
    }

    match args[1].as_str() {
        "1" => super::trustonic::trustonic_menu(),
        "q" => eprintln!("Exiting."),
        _ => eprintln!("Invalid argument. Use '1' or 'q'."),
    }
}
