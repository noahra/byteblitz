use std::error::Error;
use std::convert::From;
use std::fmt::Display;
use std::fs;
use anyhow::Result;
use crossterm::{
  event::{self, Event::Key, KeyCode::Char},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
  prelude::{CrosstermBackend, Terminal, Frame},
  widgets::{Block, Borders, Table, Row},
    layout::{Constraint, Layout},
};


pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    

    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;


    startup()?;
    let status = generate_ui(config);
    shutdown()?;
    status?;

       // shutdown down: reset terminal back to original state
       crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
       crossterm::terminal::disable_raw_mode()?;
   


    Ok(())
}

fn print_converted_elements_with_padding<T: Display>(elements: &Vec<T>, element: T) {
    print!("{:10} ", element);
    if (elements.len() % 8) == 0 {
        println!();
    }
}

pub fn convert_to_u32(bytes: [u8; 4]) -> Option<u32> {
    if bytes.len() >= 4 {
        let b = [bytes[0], bytes[1], bytes[2], bytes[3]];
        Some(u32::from_be_bytes(b))
    } else {
        None
    }
}
fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;
    Ok(())
  }



  fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
  }

  fn ui(app: &App, f: &mut Frame) {
    let constraints = [
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ];

    // Create a layout
    let layout = Layout::default()
        .constraints(constraints.as_ref())
        .split(f.size());

    // Convert the u32 numbers to strings and collect them into a Vec
    let number_strings: Vec<String> = app.converted_numbers.iter().map(|n| n.to_string()).collect();

    // Split the number strings into rows for the table
    let rows = number_strings.chunks(4).map(|chunk| Row::new(chunk.to_vec()));

    // Create a table widget
    let table = Table::new(rows, &constraints)
        .block(Block::default().title("Converted Numbers").borders(Borders::ALL));

    // Render the table in the layout
    f.render_widget(table, layout[0]);
  }

  fn update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
      if let Key(key) = event::read()? {
        if key.kind == event::KeyEventKind::Press {
          match key.code {
            Char('j') => app.counter += 1,
            Char('k') => app.counter -= 1,
            Char('q') => app.should_quit = true,
            Char('x') => {
                // Attempt to read and convert the next 4 bytes
                if let Err(e) = app.read_and_convert_next_u32() {
                    eprintln!("Error: {}", e);
                
                }
            }
            _ => {},
          }
        }
      }
    }
    Ok(())
  }
 struct App {
    counter: i64,
    should_quit: bool,
    contents: Vec<u8>,        
    converted_numbers: Vec<u32>, 
    index: usize,
}
impl App {
    fn read_and_convert_next_u32(&mut self) -> Result<(), Box<dyn Error>> {
        if self.index + 4 <= self.contents.len() {
            let bytes = [
                self.contents[self.index],
                self.contents[self.index + 1],
                self.contents[self.index + 2],
                self.contents[self.index + 3],
            ];
            self.index += 4;

            if let Some(u32_number) = convert_to_u32(bytes) {
                self.converted_numbers.push(u32_number);
            } else {
                return Err("Failed to convert bytes to u32.".into());
            }
        } else {
            return Err("Not enough bytes left to read.".into());
        }
        Ok(())
    }
}






  fn generate_ui(config: Config) -> Result<(), Box<dyn Error>> {
    // ratatui terminal
    let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    let bytes_read = fs::read(config.file_path)?;
    let mut u32_numbers = Vec::new();
    
    add_bytes_as_u32(&bytes_read, 0, 1600, &mut u32_numbers)?;
    let mut app = App { counter: 0, should_quit: false, contents: bytes_read, converted_numbers:u32_numbers, index:1600};
  
    loop {
      t.draw(|f| {
        ui(&app, f);
      })?;
  
      // application update
      update(&mut app)?;
  
      // application exit
      if app.should_quit {
        break;
      }
    }
  
    Ok(())
  }
  

  fn add_bytes_as_u32(
    bytes: &[u8], 
    start: usize, 
    num_bytes: usize, 
    u32_numbers: &mut Vec<u32>
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if the range is within the byte array's bounds and is a multiple of 4
    if start + num_bytes > bytes.len() || num_bytes % 4 != 0 {
        return Err(From::from("Range is out of bounds or not aligned to 4 bytes."));
    }

    // Iterate over the bytes in steps of 4
    for i in (start..start + num_bytes).step_by(4) {
        let chunk = [
            bytes[i],
            bytes[i + 1],
            bytes[i + 2],
            bytes[i + 3],
        ];
        if let Some(u32_integer) = convert_to_u32(chunk) {
            u32_numbers.push(u32_integer);
        } else {
            return Err(From::from("Failed to convert bytes to u32."));
        }
    }

    Ok(())
}

  



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u32_conversion() {
        let result = convert_to_u32([0x89, 0x50, 0x4E, 0x47]);
        assert_eq!(result, Some(2303741511));
    }
}
