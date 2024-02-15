use crossbeam::thread;
use std::sync::{Arc, Barrier};
use std::thread::Builder;
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

    #[allow(dead_code)]
    pub fn use_builder(&self) {
        thread::scope(|scope| {
            let handle1 = scope
                .builder()
                .name(String::from("sum1"))
                .stack_size(1024 * 3)
                .spawn(|_| {
                    self.summary(
                        std::thread::current().name().unwrap(),
                        vec![10, 20, 30, 40, 50],
                    )
                })
                .unwrap_or_else(|error| panic!("{:?}", error));
            let total1 = handle1.join().unwrap_or_else(|error| panic!("{:?}", error));
            println!("Total1: {}", total1);
        })
        .unwrap();
    }

    #[allow(dead_code)]
    fn summary_a(values: Vec<u64>) -> u64 {
        let mut total: u64 = 0;
        for value in values {
            total = total + value;
            std::thread::sleep(Duration::from_secs(2));
            println!("value of sum1 is {}", value);
        }
        total
    }

    #[allow(dead_code)]
    pub fn use_barrier() {
        let mut handles = Vec::with_capacity(3);
        let barrier = Arc::new(Barrier::new(3));
        let mut num: u64 = 0;
        while 2 >= num {
            let arc = Arc::clone(&barrier);
            handles.push(
                Builder::new()
                    .name(format!("{}{}", "summary", num))
                    .stack_size(1024 * 5)
                    .spawn(move || {
                        let data: Vec<u64> = vec![10 + num, 20 + num, 30 + num, 40 + num, 50 + num];
                        let result = Self::summary_a(data);
                        let wresult = arc.wait();
                        println!(
                            "{} finish:{}",
                            std::thread::current().name().unwrap(),
                            wresult.is_leader()
                        );
                        result
                    })
                    .unwrap_or_else(|error| panic!("{:?}", error)),
            );
            num += 1;
        }
        for handle in handles {
            let thread = handle.thread().clone();
            let result = handle.join().unwrap_or_else(|error| panic!("{:?}", error));
            println!("{}:{}", thread.name().unwrap(), result);
        }
    }
}

#[allow(dead_code)]
pub fn thread_controller_1() {
    let s = Summary::default();
    s.summary_thread();
}

#[allow(dead_code)]
pub fn thread_controller_2() {
    let s = Summary::default();
    s.use_builder();
}

#[allow(dead_code)]
pub fn thread_controller_3() {
    Summary::use_barrier();
}
