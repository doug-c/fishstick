use tokio::sync::broadcast;
use tokio_stream::{self as stream, StreamExt};

#[tokio::main]
/*
This code is spawns a new task that waits for two values to be sent down a channel, 
and asserts that these values are `10` and `20`.
*/

async fn main() {
    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();

    tokio::spawn(async move {
        assert_eq!(rx1.recv().await.unwrap(), 10);
        assert_eq!(rx1.recv().await.unwrap(), 20);
    });

    tokio::spawn(async move {
        assert_eq!(rx2.recv().await.unwrap(), 10);
        assert_eq!(rx2.recv().await.unwrap(), 20);
    });

    tx.send(10).unwrap();
    tx.send(20).unwrap();

    let data = vec![ 1, 2, 3]; 
    println!("data: {data:?}");
    // data is no longer available, it is owned by the closure.
    let dac = move || {
        println!(" captured {data:?} by value"); 
    };
    dac();

    let stream = stream::iter(1..=8);
    let mut evens = stream.filter_map(|x| {
        if x % 2 == 0 { Some(x + 1) } else { None }
    });

    assert_eq!(Some(3), evens.next().await);
    assert_eq!(Some(5), evens.next().await);
    assert_eq!(Some(7), evens.next().await);
    assert_eq!(Some(9), evens.next().await);
    assert_eq!(None, evens.next().await);
}
