use rayon::prelude::*;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use thread::JoinHandle;

#[derive(PartialEq, Debug)]
pub enum WorkerState<R> {
    Values(R),
    Complete,
}

pub struct Worker<R> {
    receiver: Receiver<R>,
    finish_sender: Sender<()>,
    handle: JoinHandle<()>,
}

impl<R> Worker<R> {
    pub fn new<S, W>(state: Vec<S>, worker: W, chunk_size: usize) -> Self
    where
        S: Send + 'static + Clone,
        W: Fn(S) -> R + Send + 'static + std::marker::Sync,
        R: Send + 'static,
    {
        // Create a channel and send to a thread
        let (tx, rx) = mpsc::channel();
        let (finish_sender, finish_receive) = mpsc::channel();

        let thread = thread::spawn(move || {
            // Chop the state into multiple parts
            let chunks = state.chunks(chunk_size);
            for chunk in chunks {
                // Check to see whether we continue the loop
                if let Ok(_) = finish_receive.try_recv() {
                    break;
                }

                let this_chunk: Vec<(S, Sender<R>)> =
                    chunk.into_iter().map(|i| (i.clone(), tx.clone())).collect();

                // iterate over each of the parts using rayon
                this_chunk.into_par_iter().for_each(|(item, tsender)| {
                    let _ = tsender.send(worker(item));
                });
            }
        });

        Self {
            receiver: rx,
            handle: thread,
            finish_sender,
        }
    }

    pub fn fetch(&self) -> WorkerState<Vec<R>> {
        let mut vals = vec![];
        loop {
            let result = self.receiver.try_recv();
            match result {
                Ok(val) => {
                    vals.push(val);
                }
                Err(err) => match err {
                    mpsc::TryRecvError::Empty => {
                        return WorkerState::Values(vals);
                    }
                    mpsc::TryRecvError::Disconnected => {
                        if vals.is_empty() {
                            return WorkerState::Complete;
                        } else {
                            return WorkerState::Values(vals);
                        }
                    }
                },
            }
        }
    }

    pub fn finish(self) {
        // Send a cancel message
        let _ = self.finish_sender.send(());

        // Wait for the thread to finish
        self.handle.join().unwrap();
    }
}

#[test]
fn create_generator() {
    use std::time::Duration;

    let vals = vec![1, 2, 3];
    let worker = Worker::new(vals.clone(), |val| val + 1, 2);
    thread::sleep(Duration::from_micros(1000));
    let results = worker.fetch();

    match results {
        WorkerState::Values(mut values) => {
            values.sort();
            assert_eq!(values, vec![2, 3, 4]);
        }
        WorkerState::Complete => {
            panic!("Shouldn't be complete until the values are collected");
        }
    }

    let complete_msg = worker.fetch();
    assert_eq!(complete_msg, WorkerState::Complete);
}
