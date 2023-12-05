use console::Term;
use injrs::inject_windows::*;
use injrs::process_windows::*;
use crossterm::{cursor, execute, terminal};
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

fn clear_c(c_move: u16){

    if c_move == 0 {
        execute!(
            stdout(),
            cursor::MoveToColumn(0),
            terminal::Clear(terminal::ClearType::CurrentLine),
        )
        .unwrap();
    return();
    }

    execute!(
        stdout(),
        cursor::MoveUp(c_move),
        terminal::Clear(terminal::ClearType::FromCursorDown)
    )
    .unwrap();
}

fn main() {
    println!("[+] lockjaw injector");

    let term = Term::stdout();
    let title = "lockjaw injector";
    term.set_title(title);

    let name = "gmod.exe";
    let dll = "lockjaw.dll";
    let mut infinite = false;

    let animation = ["-", "/", "|", "\\"];
    let mut counter = 4;

    if !std::path::Path::new(dll).exists() {
        println!(
            "current dir: {}",
            std::env::current_dir().unwrap().display()
        );
        println!("err: {} not found, press ENTER to exit", dll);
        std::io::stdin().read_line(&mut String::new()).unwrap();
        return();
    }

    println!("toggle inf loop after injection: (y/n) - default: n");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    if input.trim() == "y" {
        infinite = true;
    }

    clear_c(2);

    loop {

        thread::sleep(Duration::from_secs(1));
        let p = Process::find_first_by_name(name);
        match p {

            None => {
                counter -= 1;
                if counter == 0 {
                    
                    counter = 4;
                }
                clear_c(0);

                print!(
                    "{} secs waiting process and trying again... {}",
                    counter,
                    animation[counter - 1]
                );
                stdout().flush().unwrap();
            }

            Some(p) => {
                clear_c(0);
                println!("[!] process found!");
                thread::sleep(Duration::from_secs(4));
                match p.inject(dll) {
                    Err(e) => {
                        println!("err: {}", e);
                    }

                    Ok(_) => {
                        println!("[+] successfully injected into process, PID: {}", p.pid);
                        loop {
                            thread::sleep(Duration::from_secs(1));
                            match Process::find_first_by_name(name) {
                                Some(_) => {}
                                None => {
                                    println!("[!] process closed, exiting...");
                                    break;
                                }
                            }
                        }
                    }
                }

                if infinite {
                    clear_c(3);
                } else {
                    break;
                }
            }
        }
    }

    print!("press ENTER to exit");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
