use std::{io::{self, stdout}, char};
use crossterm::{terminal::{enable_raw_mode},
    event::{Event, KeyCode, KeyModifiers, read, self}};

fn to_ctrl_byte(c:char) -> u8{
    let byte = c as u8;
    byte & 0b0001_1111
}

fn die(e: std::io::Error){
    panic!("{}",e);
}


pub struct Editor{

}

impl Editor{
    pub fn run(&self){
        let _ = enable_raw_mode();
        let _stdout = stdout();

        while let Ok(pressed_key) = read(){
            match pressed_key {
                Event::Key(event)  => {
                    match (event.modifiers, event.code) {
                        (KeyModifiers::CONTROL, KeyCode::Char('q')) | (_, KeyCode::Esc) => {
                            break;
                        },
    
                        (_, KeyCode::Char(c)) => {
                            if c.is_control(){
                                println!("{:?} \r", c as u8);
                            }else{
                                println!("{:?} ({}) \r", c as u8, c);
                            }
                        },
    
                        (_,_) => println!("{:?} \r", pressed_key),
                    }
                },
                _ => println!("{}","hahah")
            }
        }
    }
}