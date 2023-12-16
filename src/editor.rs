use std::{io::{self, stdout, Write, Stdout}, char, fmt::Error};
use crossterm::{terminal::{enable_raw_mode,Clear,ClearType},
    event::{Event, KeyCode, KeyModifiers, read, self}, style::Stylize, ExecutableCommand};

fn to_ctrl_byte(c:char) -> u8{
    let byte = c as u8;
    byte & 0b0001_1111
}

fn die(e: std::io::Error){
    panic!("{}",e);
}


pub struct Editor{
    should_quit: bool,
    // _stdout: io::Stdout,
}

impl Editor{
    pub fn default() -> Self{
        Self{
            should_quit:false,
            // _stdout:stdout()
        }
    }

    pub fn run(&mut self){
        let _ = enable_raw_mode();
        let _stdout = stdout();

        loop {
            if let Err(error) = self.refresh_screen(){
                die(error);
            }

            if self.should_quit{
                break;
            }
            if let Err(error) = self.process_keypress(){
                die(error);
            }
        }
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error>{
        // print!("\x1b[2J");
        stdout().execute(Clear(ClearType::All)).ok();
        io::stdout().flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error>{
        let pressed_key = Self::read_key()?;
        match pressed_key {
            Event::Key(key) => match (key.modifiers, key.code){
                (KeyModifiers::CONTROL, KeyCode::Char('q')) => Ok(self.should_quit = true),
                    // panic!("Program End.");
                    
                (_, KeyCode::Char(c)) => {
                    if c.is_control(){
                        Ok(println!("{:?} \r", c as u8))
                    }else{
                        Ok(println!("{:?} ({}) \r", c as u8, c))
                    }
                },

                (_,_) => Ok(println!("{:?} \r", pressed_key)),

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