use tokio::{clock, timer};
use std::time::Duration;
use std::ops::RangeInclusive;

/// A pendulum. Nice, heavy and reliable.
///
///  |
///  |
///  |
/// ( )
///
async fn pendulum(period: u64) {
    tokio::timer::delay(clock::now() + Duration::from_secs(period)).await;
}

/// Tick, tock goes the clock.
///   __________       __________
///  /________ /|     /________ /|
/// |   XII , | |    |   XII   | |
/// |     ,'  | |    |         | |
/// |IX  * III| |    |IX  *----| |
/// |     `.  | |    |     `.  | |
/// |____VI___| | -> |____VI___| |
/// |    /    | |    |    \    | |
/// |   /     | |    |     \   | |
/// |  /      | |    |      \  | |
/// |( )      | |    |      ( )| |
/// |_________|/     |_________|/
///
async fn pendulum_clock(index: isize) {
    let mut left = true;
    loop {
        pendulum(1).await;
        if left {
            println!("tick: {}", index);
        } else {
            println!("tock: {}", index);
        }
        left = !left;
    }
}

///   ______    ______    ______    ______    ______    ______    ______
///  /____ /|  /____ /|  /____ /|  /____ /|  /____ /|  /____ /|  /____ /|
/// |     | | |     | | |     | | |     | | |     | | |     | | |     | |
/// |  |  | | |  /  | | |  \  | | |  /  | | |  \  | | |  /  | | |  \  | |
/// |  0  | | | 0   | | |   0 | | | 0   | | |   0 | | | 0   | | |   0 | |
/// |_____|/  |_____|/  |_____|/  |_____|/  |_____|/  |_____|/  |_____|/
///
async fn pendulum_museum(range: RangeInclusive<isize>, phase_offset: Duration) {
    for i in range {
        timer::delay(clock::now() + phase_offset).await;
        tokio::spawn(pendulum_clock(i));
    }
}

///  _-_
/// (◎ ◎)  Stare deep into my eyes.
///  \_/
#[tokio::main]
async fn main() {
    let range = -20..=10;
    let phase_offset = Duration::from_millis(1000 / range.size_hint().0 as u64 - 1);
    println!("range = {:?}", range);
    println!("phase_offset = {:?}", phase_offset);
    pendulum_museum(range, phase_offset).await;

    // TODO: I'd love to avoid this.
    loop {}
}
