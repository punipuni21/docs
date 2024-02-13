use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

#[allow(dead_code)]
fn summary_thread_1(name: String, values: Vec<u64>) -> JoinHandle<u64> {
    let join_handle = thread::spawn(move || {
        let mut total: u64 = 0;
        for value in values {
            total += value;
            thread::sleep(Duration::from_secs(2));
            println!("{}: {}", name, total);
        }
        total
    });
    join_handle
}

#[allow(dead_code)]
pub fn thread_controller_1() {
    let thd1 = summary_thread_1(String::from("thd1"), vec![10, 20, 30, 40, 50]);
    let thd2 = summary_thread_1(String::from("thd2"), vec![100, 200, 300, 400, 500]);
    let result1 = thd1.join().map_err(|error| panic!("{:?}", error)).unwrap();
    let result2 = thd2.join().map_err(|error| panic!("{:?}", error)).unwrap();

    println!("total value of thd1 : {}", result1);
    println!("total value of thd2 : {}", result2);
}
