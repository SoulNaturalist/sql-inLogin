mod request;
use std::time::Instant;


fn timer<F>(func: F) -> ()
where
    F: FnOnce() -> (),
{
    let start = Instant::now();
    func();
    let end = Instant::now();
    let duration = end - start;
    println!("Duration: {:?}", duration);
}
fn main() {
    timer(|| {
        println!("{:?}", request::req_with_sql_inject(["1","2","3","4","5"], "https://vk.com/feed" ));
    });
}
