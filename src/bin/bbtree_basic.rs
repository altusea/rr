// bbtree::unsync::BBTreeMap: std::BTreeMap 的 drop-in 替代品
// 文章: https://blog.xiangpeng.systems/posts/bbtree/
use bbtree::unsync::BBTreeMap;
use bbtree::{InlineKey, InlineWord, WithInlineKey};
use std::collections::BTreeMap;

fn main() {
    // 1. 和 std 的 BTreeMap 同一套 API,同样的结果
    let mut fast: BBTreeMap<u64, &str> = BBTreeMap::new();
    let mut std_map: BTreeMap<u64, &str> = BTreeMap::new();
    for (k, v) in [(3, "three"), (1, "one"), (4, "four"), (2, "two")] {
        fast.insert(k, v);
        std_map.insert(k, v);
    }
    fast.insert(2, "TWO");
    std_map.insert(2, "TWO");

    assert_eq!(fast.get(&2), std_map.get(&2));
    assert_eq!(fast.remove(&1), std_map.remove(&1));
    assert_eq!(fast.len(), std_map.len());
    assert_eq!(
        fast.iter().collect::<Vec<_>>(),
        std_map.iter().collect::<Vec<_>>()
    );
    assert_eq!(
        fast.range(2..=3).map(|(k, v)| (*k, *v)).collect::<Vec<_>>(),
        std_map
            .range(2..=3)
            .map(|(k, v)| (*k, *v))
            .collect::<Vec<_>>()
    );
    println!("drop-in: {:?}", fast.iter().collect::<Vec<_>>());

    // 2. 自定义 key 实现 InlineKey:节点里存每个 key 的前 8 字节,
    //    一次 cache miss 就能比较一整排 key,只有打平时才读完整 key
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
    struct OrderKey {
        customer: u32,
        order: u32,
    }

    impl InlineKey for OrderKey {
        fn inline_key_at(&self, offset: usize) -> InlineWord {
            // 编码 = customer 在前、order 在后的大端 8 字节;
            // 第 offset 字节起的 8 字节窗口 = 整个编码左移 8*offset,
            // 越过末尾补 0,顺序与 Ord 一致
            let enc = (u64::from(self.customer) << 32) | u64::from(self.order);
            enc.checked_shl(offset as u32 * 8).unwrap_or(0)
        }
    }

    let mut orders: BBTreeMap<OrderKey, &str, WithInlineKey> =
        BBTreeMap::with_strategy(WithInlineKey);
    orders.insert(
        OrderKey {
            customer: 7,
            order: 2,
        },
        "c7-o2",
    );
    orders.insert(
        OrderKey {
            customer: 7,
            order: 1,
        },
        "c7-o1",
    );
    orders.insert(
        OrderKey {
            customer: 3,
            order: 9,
        },
        "c3-o9",
    );

    assert_eq!(
        orders.get(&OrderKey {
            customer: 7,
            order: 2
        }),
        Some(&"c7-o2")
    );
    // 按 (customer, order) 排序
    println!(
        "inline key order: {:?}",
        orders.values().collect::<Vec<_>>()
    );
    assert_eq!(
        orders.values().copied().collect::<Vec<_>>(),
        ["c3-o9", "c7-o1", "c7-o2"]
    );
}
