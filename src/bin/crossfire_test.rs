extern crate crossfire;
use crossfire::*;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::bounded_async::<i32>(100);
    for _ in 0..10 {
        let _tx = tx.clone();
        tokio::spawn(async move {
            for i in 0i32..10 {
                let _ = _tx.send(i).await;
                sleep(Duration::from_millis(100)).await;
                println!("sent {}", i);
            }
        });
    }
    drop(tx);
    let mut inv = tokio::time::interval(Duration::from_millis(500));
    loop {
        tokio::select! {
            _ = inv.tick() =>{
                println!("tick");
            }
            r = rx.recv() => {
                if let Ok(_i) = r {
                    println!("recv {}", _i);
                } else {
                    println!("rx closed");
                    break;
                }
            }
        }
    }
}
