use core::panic;
use std::{io::{stdout, Write}, time::Duration};

use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;

use crossterm::{
    ExecutableCommand,
    cursor::position,
    cursor::{self, MoveLeft, MoveRight, MoveDown, MoveUp, SavePosition, RestorePosition},
    event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen},
    Result, QueueableCommand,
    style::{Print}
};

const HELP: &str = r#"EventStream based on futures_util::Stream with tokio
 - Keyboard, mouse and terminal resize events enabled
 - Prints "." every second if there's no event
 - Hit "c" to print current cursor position
 - Use Esc to quit
"#;

struct game {
    board: Vec<Vec<char>>,
}

fn make_board(x: usize, y: usize) -> Vec<Vec<char>> {
    vec![vec!['.'; x]; y]
}

fn print_board() -> () {

}

async fn print_events() {
    let mut stdout = stdout();
    let mut reader = EventStream::new();

    loop {
        let mut delay = Delay::new(Duration::from_millis(1_000)).fuse();
        let mut event = reader.next().fuse();

        let size_res = size();
        let term_x;
        let term_y;
        match size_res {
            Ok(v) => {
                term_x = v.0;
                term_y = v.1;
            },
            Err(e) => panic!("Scary."),
        }


        let mut state = game {
            board: make_board(term_x as usize, term_y as usize),
        };

        select! {
            _ = delay => {
                execute!(stdout, SavePosition);
                for (x, row) in state.board.iter_mut().enumerate() {
                    let mut a_row: String = "".to_string();
                    row.iter()
                        .map(|x| a_row += &x.to_string())
                        .for_each(drop);
                    println!("{}\r",a_row);
                }
                execute!(stdout, RestorePosition);
            },
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        if event == Event::Key(KeyCode::Char('c').into()) {
                            println!("Cursor position: {:?}", position());
                        }

                        if event == Event::Key(KeyCode::Char('t').into()) {
                            println!("moved cursor");
                        }

                        if event == Event::Key(KeyCode::Char('h').into()) {
                            stdout.queue(MoveLeft(1));
                            stdout.flush();
                        }
                        if event == Event::Key(KeyCode::Left.into()) {
                            stdout.queue(MoveLeft(1));
                            stdout.flush();
                        }
                        if event == Event::Key(KeyCode::Right.into()) {
                            stdout.queue(MoveRight(1));
                            stdout.flush();
                        }
                        if event == Event::Key(KeyCode::Char('j').into()) {
                            stdout.queue(MoveDown(1));
                            stdout.flush();
                        }

                        if event == Event::Key(KeyCode::Char('k').into()) {
                            stdout.queue(MoveUp(1));
                            stdout.flush();
                        }

                        if event == Event::Key(KeyCode::Char('l').into()) {
                            stdout.queue(MoveRight(1)).expect("something!");
                            stdout.flush();
                        }
                        
                        if event == Event::Key(KeyCode::Enter.into()) {
                            stdout.queue(Print("BENIS".to_string()));
                            stdout.flush();
                            let res = execute!(stdout, EnterAlternateScreen);
                        }
                        if event == Event::Key(KeyCode::Char('0').into()) {
                            execute!(stdout, LeaveAlternateScreen);
                        }

                        if event == Event::Key(KeyCode::Esc.into()) {
                            break;
                        }
                    }
                    Some(Err(e)) => println!("Error: {:?}\r", e),
                    None => break,
                }
            }
        };
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", HELP);

    enable_raw_mode()?;


    print_events().await;


    disable_raw_mode();
    Ok(())
}