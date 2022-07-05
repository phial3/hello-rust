use std::cell::RefCell;
use std::thread;
use std::sync::Arc;
use std::cell::Cell;
use thread_local::ThreadLocal;

fn main() {
    // thread_local!(static FOO: RefCell<u32> = RefCell::new(1));
    //
    // FOO.with(|f| {
    //     assert_eq!(*f.borrow(), 1);
    //     *f.borrow_mut() = 2;
    // });
    //
    // // 每个线程开始时都会拿到线程局部变量的FOO的初始值
    // let t = thread::spawn(move || {
    //     FOO.with(|f| {
    //         assert_eq!(*f.borrow(), 1);
    //         *f.borrow_mut() = 3;
    //     });
    // });
    //
    // // 等待线程完成
    // t.join().unwrap();
    //
    // // 尽管子线程中修改为了3，我们在这里依然拥有main线程中的局部值：2
    // FOO.with(|f| {
    //     assert_eq!(*f.borrow(), 2);
    // });
    //
    // Foo::FOO.with(|x| println!("{:?}", x));


    let tls = Arc::new(ThreadLocal::new());

    // 创建多个线程
    for _ in 0..5 {
        let tls2 = tls.clone();
        thread::spawn(move || {
            // 将计数器加1
            let cell = tls2.get_or(|| Cell::new(0));
            cell.set(cell.get() + 1);
        }).join().unwrap();
    }

    // 一旦所有子线程结束，收集它们的线程局部变量中的计数器值，然后进行求和
    let tls = Arc::try_unwrap(tls).unwrap();
    let total = tls.into_iter().fold(0, |x, y| x + y.get());

    // 和为5
    assert_eq!(total, 5);
}

struct Foo;

impl Foo {
    thread_local! {
        static FOO: RefCell<usize> = RefCell::new(0);
    }
}
