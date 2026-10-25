// bbtree-mmap: 把建好的树压缩写进文件,重启后直接 mmap 查询,不需要重建索引
use bbtree::unsync::BBTreeMap;
use bbtree_mmap::BBTreeMapFile;
use std::io;
use std::time::Instant;

fn main() -> io::Result<()> {
    let path = std::env::temp_dir().join("rr-bbtree-names.bbtree");

    // 建树,一次写到磁盘;条目必须按 key 升序
    let names: BBTreeMap<u64, String> = [
        (7, String::from("ferris")),
        (42, String::from("corro")),
        (1000, String::from("crab")),
    ]
    .into_iter()
    .collect();
    BBTreeMapFile::<u64, String>::create(&path, &names)?;

    // “重启”:不重建任何结构,mmap 文件后直接查
    let t = Instant::now();
    // SAFETY: 映射期间没有别人改这个文件
    let file = unsafe { BBTreeMapFile::<u64, String>::mmap(&path).open()? };
    let answer = file.get(&42).unwrap();
    let elapsed = t.elapsed();

    assert_eq!(answer, "corro");
    assert_eq!(file.len(), 3);
    // 批量查同样可用
    assert_eq!(file.get_many([&7, &999]), [Some("ferris"), None]);
    println!("first answer {answer:?} after {:?}", elapsed);

    std::fs::remove_file(&path)?;
    Ok(())
}
