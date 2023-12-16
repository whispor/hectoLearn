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
    should_quit: bool,
}

impl Editor{
    pub fn default() -> Self{
        Self{should_quit:false}
    }

    pub fn run(&self){
        let _ = enable_raw_mode();
        let _stdout = stdout();

        // while let Ok(pressed_key) = read(){
        //     match pressed_key {
        //         Event::Key(event)  => {
        //             match (event.modifiers, event.code) {
        //                 (KeyModifiers::CONTROL, KeyCode::Char('q')) | (_, KeyCode::Esc) => {
        //                     break;
        //                 },
    
        //                 (_, KeyCode::Char(c)) => {
        //                     if c.is_control(){
        //                         println!("{:?} \r", c as u8);
        //                     }else{
        //                         println!("{:?} ({}) \r", c as u8, c);
        //                     }
        //                 },
    
        //                 (_,_) => println!("{:?} \r", pressed_key),
        //             }
        //         },
        //         _ => println!("{}","hahah")
        //     }
        // }

        loop {
            if let Err(error) = self.process_keypress(){
                die(error);
            }
        }
    }


    fn process_keypress(&self) -> Result<(), std::io::Error>{
        let pressed_key = Self::read_key()?;
        match pressed_key {
            Event::Key(key) => match (key.modifiers, key.code){
                (KeyModifiers::CONTROL, KeyCode::Char('q')) => panic!("Program End."),
                (_, _) => Ok(())
            },
            _ => Ok(())
        }

    }

    fn read_key() -> Result<Event, std::io::Error>{
        loop {
            let event = read();
            if let Ok(Event::Key(_)) = event{
                return event
            }
        }
    }
}