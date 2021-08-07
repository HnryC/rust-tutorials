use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() -> Result<(), ()> {
    println!("Basic multithread");

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Spawned thread, {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Main thread, {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    let vec = vec![3, 5, 2, 4, 3, 2];

    thread::spawn(move || {
        for v in vec {
            println!("{}", v);
        }
    });

    handle.join().unwrap();

    println!("Message sending");
    let (tx, rx) = mpsc::channel();
    let tx_clone = tx.clone();

    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10)); //wait for 1/2 secconds
        tx_clone.send("Started").unwrap();
        thread::sleep(Duration::from_millis(1000)); //wait for 1/2 secconds
        tx_clone.send("Finished").unwrap();
    });

    if let Ok(i) = rx.try_recv() {
        println!("Quickly recived {}", i);
    } else {
        println!("Slight delay reciving {}", rx.recv().unwrap());
    }
    handle.join().unwrap();


    println!("{}", rx.recv().unwrap());

    let tx_clone = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            "hi",
            "the",
        ];

        for val in vals {
            tx_clone.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let tx_clone = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            "from",
            "threads",
        ];

        for val in vals {
            tx_clone.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    Ok(())
}
