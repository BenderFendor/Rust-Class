use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::vec;

use tokio::runtime::Runtime;
use tokio::sync::broadcast;
use tokio::sync::mpsc as tokio_mpsc;
use tokio::time::sleep;
use tokio_mpsc::error::SendError;

pub fn q1() -> i32 {
    // TODO: Create a new channel using `mpsc::channel()`

    

    // TODO: Send the following message
    // The `send()` method returns a Result<(), SendError<T>>
    let message: i32 = 42;

    // TODO: Receive the message.
    // The `recv()` method blocks the current thread until a message is received.
    // Then it returns a Result<T, RecvError>.

    // TODO: Return the recieved message
    todo!()
}

#[test]
fn test_q1_basic_send_receive() {
    assert_eq!(q1(), 42, "Q1 Failed: Should receive the integer 42.");
}

pub fn q2() -> String {
    // TODO: Do the same thing as above: create a channel
    // Use it to send and receive a String message "Hello World"
    // Then return the String

    let (sender,reciever) = mpsc::channel();

    let message = "Hello World".to_string();

    sender.send(message).unwrap();

   let s = reciever.recv().unwrap();

   s
}

#[test]
fn test_q2_send_owned_data() {
    assert_eq!(
        q2(),
        "Hello World".to_string(),
        "Q2 Failed: Should successfully receive the owned String."
    );
}

pub fn q3() -> Vec<i32> {
    // TODO: Create a new channel

    let (tx,rx) = mpsc::channel();

    // TODO: Clone the sender with `clone()`

    let new_tx = tx.clone();


    // TODO: Send the i32 100 from the original channel and the i32 200 from the clone

    tx.send(100).unwrap();

    new_tx.send(200).unwrap();

    // TODO: Receive both the messages and collect the results in a vector, then return it

    let mut v = Vec::new();

    for recieved_msg in rx {
        v.push(recieved_msg)
    }

    v
}

#[test]
fn test_q3_clone_sender() {
    let mut result = q3();
    // The order is not guaranteed, so sort before asserting.
    result.sort();
    assert_eq!(
        result,
        vec![100, 200],
        "Q3 Failed: Should receive messages from both the original and cloned sender."
    );
}

pub fn q4() -> Vec<String> {
    // TODO: Create a channel

    let (tx,rx) = mpsc::channel();

    let names = vec!["Alice", "Bob", "Charlie"];

    // TODO: Send all the names over the channel, one after another
    
    for n in names {
        tx.send(n.to_string()).unwrap()
    }
    // TODO: Recieve all the messages and return it

    drop(tx);

    let mut recieved_names = Vec::new();
    for names in rx {
        recieved_names.push(names);
    }
    recieved_names
}

#[test]
fn test_q4_iterating_receiver() {
    assert_eq!(
        q4(),
        vec![
            "Alice".to_string(),
            "Bob".to_string(),
            "Charlie".to_string()
        ],
        "Q4 Failed: The receiver iterator should collect all messages."
    );
}

pub fn q5() -> i32 {
    let (tx, rx) = mpsc::channel::<i32>();
    let message = 500;

    // TODO: Spawn a new thread send the message from this thread.
    // Note that you do not need to clone the Sender

    // TODO: Receive the message in the main thread and return it
    todo!()
}

#[test]
fn test_q5_send_from_thread() {
    let received = q5();
    assert_eq!(received, 500, "Q5 Failed: Main thread should block and successfully receive the message sent from the spawned thread.");
}

pub fn q6() -> Vec<i32> {
    let (tx, rx) = mpsc::channel();
    let num_threads = 5;

    // TODO: Create 5 threads and give each a clone of the Sender
    // TODO: Have the threads send the numbers 1 through 5

    // Here we drop the original sender.
    // If the original sender is not dropped, the receiver's iteration won't terminate.
    drop(tx);

    // TODO: Join the threads using their handles

    // 4. Collect all messages from the receiver using the iterator.
    let mut received_messages: Vec<i32> = rx.into_iter().collect();

    // The order is not guaranteed, so we sort it for testing purposes.
    received_messages.sort();
    received_messages
}

#[test]
fn test_q6_multiple_threaded_producers() {
    let result = q6();
    assert_eq!(
        result,
        vec![1, 2, 3, 4, 5],
        "Q6 Failed: Should receive all 5 messages from the 5 producers."
    );
}

#[derive(Debug, PartialEq)]
pub struct TaskReport {
    task_id: u32,
    status: String,
    duration_ms: u64,
}

pub fn q7() -> TaskReport {
    // TODO: Create a channel that sends the custom TaskReport type above

    // TODO: Send, recieve, and return the following TaskReport
    let report = TaskReport {
        task_id: 101,
        status: String::from("Completed"),
        duration_ms: 1500,
    };

    todo!()
}

#[test]
fn test_q7_send_custom_struct() {
    let expected = TaskReport {
        task_id: 101,
        status: String::from("Completed"),
        duration_ms: 1500,
    };
    let received = q7();
    assert_eq!(
        received, expected,
        "Q7 Failed: Should successfully send and receive the custom TaskReport struct."
    );
}

pub async fn q8() -> String {
    // Create a Tokio unbounded channel.
    let (tx, mut rx) = tokio_mpsc::unbounded_channel::<String>();

    let message = String::from("Async message received!");

    // TODO: Send the message (same as before).
    // Unbounded senders never await/block.

    // TODO: Receive and return the message. Tokio receivers use `.recv().await`.
    todo!()
}

#[tokio::test]
async fn test_q8_tokio_unbounded() {
    let expected = "Async message received!".to_string();
    let received = q8().await; // Must use .await now
    assert_eq!(received, expected, "Q8 Failed: Should successfully send and receive a message using Tokio's unbounded channel.");
}

pub async fn q9() -> i32 {
    // Create a bounded channel with capacity 1.
    let (tx, mut rx) = tokio_mpsc::channel::<i32>(1);
    let mut successful_sends = 0;

    // TODO: spawn a thread that uses a receiver to receive messages
    tokio::spawn(async move {
        // Receive the first message
        let recieved = rx.recv().await;

        // TODO: If `recieved` `is_some()`, revieve the second message
    });

    // TODO: Send the first message.
    // Bounded channels need to use `send(message).await`
    let sent_1 = tx.send(1).await;

    // TODO: Send the second message.
    let sent_2: Result<(), SendError<i32>> = todo!();

    if sent_1.is_ok() {
        successful_sends += 1;
    }
    if sent_2.is_ok() {
        successful_sends += 1;
    }

    successful_sends
}

#[tokio::test]
async fn test_q9_tokio_bounded_backpressure() {
    let expected = 2;
    let received = q9().await; // Must use .await now
    assert_eq!(received, expected, "Q9 Failed: Both messages should be sent successfully, demonstrating the sender awaiting backpressure resolution.");
}

pub async fn q10() -> i32 {
    // Create a broadcast channel with a capacity of 10 recievers
    let (tx, _rx_orig) = broadcast::channel::<i32>(10);

    // Clone a reciever thread
    let mut rx_clone_a = tx.subscribe();
    // TODO: clone another reciever/subscriber

    // Spawn a thread to send two messages.
    let sender_handle = tokio::spawn(async move {
        tx.send(50).unwrap();
        tx.send(51).unwrap();
    });

    // We create a thread to recieve two messages
    let rec_a_handle = tokio::spawn(async move {
        let mut count = 0;
        // The receiver must await the message.
        if rx_clone_a.recv().await.is_ok() {
            count += 1;
        }
        if rx_clone_a.recv().await.is_ok() {
            count += 1;
        }
        count
    });

    // TODO: Create a new thread to receive another two messages
    // Return the sum of the successfully received threads, as above

    // Ensure the sender task completes.
    sender_handle.await.unwrap();

    // TODO: Sum results from both receivers.
    rec_a_handle.await.unwrap()
}

#[tokio::test]
async fn test_q10_tokio_broadcast() {
    let expected = 4; // 2 messages * 2 receivers
    let received = q10().await; // Must use .await now
    assert_eq!(received, expected, "Q10 Failed: Both receivers should have received both messages, totaling 4 successful receives.");
}
