use std::io::{self, Read};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, RecvTimeoutError, Sender};
use std::sync::Arc;
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;

static CLOSING: AtomicBool = AtomicBool::new(false);

fn spawn_stdin_channel() -> Receiver<char> {
    let (tx, rx) = channel::<char>();
    spawn(move || {
        for maybe_byte in io::stdin().lock().bytes() {
            if CLOSING.load(Ordering::Relaxed) {
                drop(tx);
                return;
            }
            match maybe_byte {
                Ok(b) => {
                    tx.send(b as char).unwrap();
                }
                Err(_) => {
                    eprintln!("Something went wrong while reading stdin");
                    drop(tx);
                    return;
                }
            }
        }
    });
    rx
}

fn spawn_player(player_number: u64, tx: Arc<Sender<u64>>) -> std::thread::JoinHandle<()> {
    spawn(move || {
        let mut j = player_number * 10;
        loop {
            if CLOSING.load(Ordering::Relaxed) {
                drop(tx);
                return;
            }
            sleep(Duration::new(1, 0));
            tx.send(j).unwrap();
            println!("i'm {player_number}, i've just send {j}");
            j += 1;
        }
    })
}

fn spawn_game_referee(rx: Receiver<char>, timeout: Duration) -> JoinHandle<()> {
    loop {
        match rx.recv_timeout(timeout) {
            Ok(key) => {
                if 'q' == key {
                    let _ =
                        CLOSING.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed);
                }
            }
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => {
                println!("We are draining all the messages in the channel, keep calm")
            }
        }
    }
}

fn main() {
    let (tx, rx) = channel::<u64>();
    let tx = Arc::new(tx);
    let stdin_channel = spawn_stdin_channel();
    println!("starting this game!\nWrite \"q\" to exit the game");
    for i in 1..10 {
        spawn_player(i, tx.clone());
    }
    drop(tx);
    spawn_game_referee(stdin_channel, Duration::from_millis(50));

    while let Ok(num) = rx.recv() {
        println!("i'm the main thread, got {num}")
    }
    println!("Thank you for playing");
}
