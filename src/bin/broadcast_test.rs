use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (brd_tx, mut brd_rx) = broadcast::channel(100);

    
    for i in (0..10) {
        let mut rx = brd_tx.subscribe();
        tokio::spawn(async move {
            loop {
                if let Ok(msg) = rx.recv().await {
                    println!("bye from task {}", i);
                    break;
                }
            }
        });
    }

    ctrlc::set_handler(move || {
        brd_tx.send(()).unwrap();
    }).unwrap();

    loop {
        if let Ok(msg) = brd_rx.recv().await {
            println!("bye");
            break;
        }
    }
}