use std::fmt::Debug;
use std::ptr;
use std::pin::Pin;
use std::future::Future;
use std::task::{Poll, Context, Waker, RawWaker, RawWakerVTable};

static RAW_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
    |data| RawWaker::new(ptr::null(), &RAW_WAKER_VTABLE),
    |data| {},
    |data| {},
    |data| {},
);

pub fn spawn<T>(u: impl Future<Output = T>) {

}

pub fn run<T: Debug>(mut task: impl Future<Output = T>) -> T {
    unsafe {
        let raw_waker = RawWaker::new(ptr::null(), &RAW_WAKER_VTABLE);
        // dbg!(&raw_waker);
        let waker = Waker::from_raw(raw_waker);
        // dbg!(&waker);
        let mut context = Context::from_waker(&waker);
        // dbg!(&context);
        let mut future = Pin::new_unchecked(&mut task);

        loop {
            // match dbg!(future.as_mut().poll(&mut context)) {
            match future.as_mut().poll(&mut context) {
                Poll::Ready(r) => return r,
                Poll::Pending => continue,
            }
        }
    }
}

use std::time::Duration;
use crate::sleep::sleep;
#[test]
fn test_executor() {
    async fn foo() -> i32 { 1 }
    async fn bar() -> i32 { 2 }
    let result = run(async {
        let i = foo().await;
        sleep(Duration::from_secs(1)).await;
        let j = bar().await;
        i + j
    });
    assert_eq!(3, result);
}

