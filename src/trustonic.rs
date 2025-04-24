use std::io::{stdin, stdout, Write};

pub fn trustonic_menu() {
    loop {
        println!("\n[Trustonic TEE Interface]");
        println!("[1] Open session to a UUID");
        println!("[2] Send TCI command");
        println!("[3] Wait for notification");
        println!("[4] Dump response buffer");
        println!("[b] Back");

        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => println!("TODO: open session"),
            "2" => println!("TODO: send TCI command"),
            "3" => println!("TODO: wait for notification"),
            "4" => println!("TODO: dump response"),
            "b" => break,
            _ => println!("Invalid input."),
        }
    }
}
