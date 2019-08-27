use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread;

#[derive(Debug)]
enum Message {
    Job(String),
    Terminate,
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let counter: Arc<RwLock<HashMap<char, Mutex<usize>>>> = Arc::new(RwLock::new(HashMap::new()));

    let (sender, receiver) = mpsc::channel();

    let receiver = Arc::new(Mutex::new(receiver));

    let threads: Vec<_> = (0..worker_count)
        .map(|_| {
            let rev = Arc::clone(&receiver);
            let counter = Arc::clone(&counter);

            thread::spawn(move || loop {
                let message = rev.lock().unwrap().recv().unwrap();
                match message {
                    Message::Job(string) => {
                        for letter in string.chars().filter(|c| c.is_alphabetic()) {
                            let map = counter.read().unwrap();

                            if let Some(x) = map.get(&letter) {
                                let mut element = x.lock().expect("test");

                                *element += 1;
                                continue;
                            }

                            drop(map);
                            let mut map = counter.write().unwrap();

                            map.insert(letter, Mutex::new(1));
                            drop(map);
                        }
                    }
                    Message::Terminate => break,
                };
            })
        })
        .collect();

    for string in input {
        sender
            .send(Message::NewJob(string.to_lowercase().to_string()))
            .unwrap();
    }

    for _ in 0..worker_count {
        sender.send(Message::Terminate).unwrap();
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let counter = Arc::try_unwrap(counter).unwrap().into_inner().unwrap();
    let mut result: HashMap<char, usize> = HashMap::new();
    for (k, v) in counter.into_iter() {
        result.insert(k, v.into_inner().unwrap());
    }

    result
}
