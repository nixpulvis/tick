use std::pin::Pin;
use std::time::{Instant, Duration};
use std::future::Future;
use std::task::{Poll, Context};

pub struct Sleep {
    until: Instant,
}

impl Future for Sleep {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.until > Instant::now() {
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

pub fn sleep(dur: Duration) -> Sleep {
    sleep_until(Instant::now() + dur)
}
pub fn sleep_until(until: Instant) -> Sleep {
    Sleep { until }
}
