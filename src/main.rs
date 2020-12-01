// #[async_std::main]
fn main() {
    run()
}

fn run() {
    let input: usize = std::env::args()
        .nth(1).unwrap()
        .parse().unwrap();
    println!("fib({}) = {}", input, fib_rayon(input));
}

fn fib_rayon(n: usize) -> usize {
    if n <= 2 {
        1
    } else {
        let (a, b) = rayon::join(
            || fib_rayon(n-1),
            || fib_rayon(n-2),
        );
        a + b
    }
}

// use std::future::Future;
// use futures::FutureExt;
// use std::pin::Pin;
// fn fib(n: usize) -> Pin<Box<dyn Future<Output=usize> + Send>> {
//     async move {
//         if n <= 2 {
//             1
//         } else {
//             let a = async_std::task::spawn(fib(n-1));
//             let b = async_std::task::spawn(fib(n-2));
//             let (a, b) = futures::join!(a, b);
//             a + b
//         }
//     }.boxed()
// }
