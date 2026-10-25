// bbtree::BBTreeMap: 多线程共享的 B+ 树
// 读者只读版本号、不写任何 cache line,所以读扩展性好;
// 值只存在叶子里,读值通过 get_copied / read 闭包拿,不交出 &V
use bbtree::BBTreeMap;
use std::ops::ControlFlow;
use std::thread;

fn main() {
    let map: BBTreeMap<u64, u64> = BBTreeMap::new();
    const THREADS: u64 = 8;
    const PER_THREAD: u64 = 1000;

    // 写:多线程并发插入,insert 返回 Result(分配失败可恢复)
    thread::scope(|scope| {
        for t in 0..THREADS {
            let map = &map;
            scope.spawn(move || {
                for i in 0..PER_THREAD {
                    map.insert(t * PER_THREAD + i, i).unwrap();
                }
            });
        }
    });
    assert!(!map.is_empty());

    // 读:拷贝值,无锁、不阻塞写
    assert_eq!(map.get_copied(&0), Some(0));
    assert_eq!(map.get_copied(&4097), Some(97)); // t=4, i=97
    assert_eq!(map.get_copied(&(THREADS * PER_THREAD)), None);

    // 读:借用闭包,避免拷贝
    assert_eq!(map.read(&42, |_k, v| v + 1), Some(43));

    // 批量读:一次调用给多个 key,树内部把每次 cache miss 当成 Future 交错执行
    assert_eq!(
        map.get_many_copied([&0, &4097, &u64::MAX]),
        [Some(0), Some(97), None]
    );

    // 范围扫描:闭包遍历,提前退出返回 Break 的值
    let first_three: Vec<u64> = {
        let mut out = Vec::new();
        let _ = map.scan(.., |_k, v| {
            out.push(*v);
            if out.len() == 3 {
                ControlFlow::Break(())
            } else {
                ControlFlow::Continue(())
            }
        });
        out
    };
    println!("first three values in key order: {first_three:?}");
    assert_eq!(first_three, [0, 1, 2]);

    // 范围迭代器:克隆条目,迭代器本身不持锁
    let sum: u64 = map.range_cloned(0..100).map(|(_, v)| v).sum();
    assert_eq!(sum, (0..100).sum());
    println!("range 0..100 sum = {sum}");
}
