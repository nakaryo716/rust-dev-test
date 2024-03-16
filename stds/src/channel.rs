use core::time;
use std::{sync::mpsc, thread::{self, sleep}};

#[allow(dead_code)]
fn channel_ts() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap();
            sleep(time::Duration::from_secs(1));
        }
    });

   while let Some(v) = rx.recv().ok() {
        println!("{}", v);
   }


   let (tx_1, rx_1) = mpsc::channel();

   thread::spawn(move || {
        for i in 0..5 {
            let data = format!("data {}", i);
            tx_1.send(data).unwrap();
            sleep(time::Duration::from_secs(1));
        }
   });

   loop {
        let data = rx_1.try_recv();
        match data {
            Ok(v) => println!("{}", v),
            Err(e) => match e {
                mpsc::TryRecvError::Empty => continue,
                mpsc::TryRecvError::Disconnected => break,
            } 
        }
   }
}