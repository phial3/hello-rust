use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 使用`Mutex`结构体的关联函数创建新的互斥锁实例
    let m = Mutex::new(5);
    // {
    //     // 获取锁，然后deref为`m`的引用
    //     // lock返回的是Result
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    //     // 锁自动被drop
    // }
    //
    // println!("m = {:?}", m);

    // let mut num = m.lock().unwrap();
    // *num = 6;
    // // 锁还没有被 drop 就尝试申请下一个锁，导致主线程阻塞
    // drop(num); // 手动 drop num ，可以让 num1 申请到下个锁
    // let mut num1 = m.lock().unwrap();
    // *num1 = 7;
    // drop(num1); // 手动 drop num1 ，观察打印结果的不同
    //
    // println!("m = {:?}", m);

    // 通过`Rc`实现`Mutex`的多所有权
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        // 创建子线程，并将`Mutex`的所有权拷贝传入到子线程中
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    // 等待所有子线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 输出最终的计数结果
    println!("Result: {}", *counter.lock().unwrap());
}
