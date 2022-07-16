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

#[test]
fn test_executor() {
    let mut task = Box::pin(async { 1 });

    unsafe {
        let raw_waker = RawWaker::new(ptr::null(), &RAW_WAKER_VTABLE);
        dbg!(&raw_waker);
        let waker = Waker::from_raw(raw_waker);
        dbg!(&waker);
        let mut context = Context::from_waker(&waker);
        dbg!(&context);
        loop {
            match dbg!(Pin::new_unchecked(&mut task).poll(&mut context)) {
                Poll::Ready(r) => {
                    assert_eq!(1, r);
                    break;
                },
                Poll::Pending => continue,
            }
        }
    }
}

