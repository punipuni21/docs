use crossbeam::thread;
use std::time::Duration;

#[derive(Debug, Default)]
pub struct Summary;
impl Summary {
    #[allow(dead_code)]
    fn summary(&self, name: &str, values: Vec<u64>) -> u64 {
        let mut total: u64 = 0;
        for value in values {
            total = total + value;
            std::thread::sleep(Duration::from_secs(2));
            println!("value of {} is {}", name, value);
        }
        total
    }

    #[allow(dead_code)]
    pub fn summary_thread(&self) {
        thread::scope(|scope| {
            let handle1 = scope.spawn(|_| self.summary("sum1", vec![10, 20, 30, 40, 50]));
            let handle2 = scope.spawn(|_| self.summary("sum2", vec![100, 200, 300, 400, 500]));

            let total1 = handle1.join().unwrap_or_else(|error| panic!("{:?}", error));
            let total2 = handle2.join().unwrap_or_else(|error| panic!("{:?}", error));

            println!("Total1: {}", total1);
            println!("Total2: {}", total2);
        })
        .unwrap();
    }
}

#[allow(dead_code)]
pub fn thread_controller_1() {
    let s = Summary::default();
    s.summary_thread();
}
