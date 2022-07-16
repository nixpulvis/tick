use std::time::Duration;
use std::ops::RangeInclusive;

mod sleep;
use crate::sleep::sleep;

mod executor;
use crate::executor::{spawn, run};

/// A pendulum. Nice, heavy and reliable.
///
///  |
///  |
///  |
/// ( )
///
async fn pendulum(period: Duration) {
    sleep(period).await;
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
async fn pendulum_clock(period: Duration, index: isize) {
    let mut left = true;
    loop {
        pendulum(period).await;
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
/// |  /  | | |  \  | | |  /  | | |  |  | | |  \  | | |  /  | | |  \  | |
/// | O   | | |   O | | | O   | | |  O  | | |   O | | | O   | | |   O | |
/// |_____|/  |_____|/  |_____|/  |_____|/  |_____|/  |_____|/  |_____|/
///   -3        -2        -1         0         1         2         3
/// ------------------------------ (◎ ◎) --------------------------------
///
async fn pendulum_museum1(
    period: Duration,
    count: usize,
    separation: Duration)
{
    // 0 is broken :(
    for n in 1..=(count/2) {
        sleep(separation).await;
        spawn(pendulum_clock(period, n as isize));
        spawn(pendulum_clock(period, -(n as isize)));
    }
}

///   ______    ______    ______    ______    ______    ______    ______
///  /____ /|  /____ /|  /____ /|  /____ /|  /____ /|  /____ /|  /____ /|
/// |     | | |     | | |     | | |     | | |     | | |     | | |     | |
/// |  /  | | |  |  | | |  \  | | |  |  | | |  /  | | |  |  | | |  \  | |
/// | O   | | |  O  | | |   O | | |  O  | | | O   | | |  O  | | |   O | |
/// |_____|/  |_____|/  |_____|/  |_____|/  |_____|/  |_____|/  |_____|/
///   -3        -2        -1         0         1         2         3
/// --------------------------------------------------------------- (◎ ◎)
///
async fn pendulum_museum2(
    period: Duration,
    range: RangeInclusive<isize>,
    separation: Duration)
{
    // I've been told they fixed 0.
    for i in range {
        sleep(separation).await;
        spawn(pendulum_clock(period, i));
    }
}

// TODO: Replace with updated main from below.
fn main() {
    run(async {
        let period = Duration::from_secs(1);
        let count = 6;
        let separation = Duration::from_millis(333);
        println!("period = {:?}", period);
        println!("count = {:?}", count);
        println!("separation = {:?}", separation);
        pendulum_museum1(period, count, separation).await;
    });

    loop {}
}

/////  _-_
///// (◎ ◎)  Stare deep into my eyes.
/////  \_/
//#[tokio::main]
//async fn old_main() {

//    // Exbibit 1: Sitting and thinking.

//    // Uniform is:
//    // - period = separation * count / 2
//    // - separation = period / 2 * count
//    //
//    // It's nice because they all tick, and tock together.

//    let period = Duration::from_secs(1);
//    let count = 6;
//    let separation = Duration::from_millis(333);
//    println!("period = {:?}", period);
//    println!("count = {:?}", count);
//    println!("separation = {:?}", separation);
//    pendulum_museum1(period, count, separation).await;

//    // TODO: I'd love to avoid this, but it's nice to avoid commenting out the
//    // rest of the code below :P
//    loop {}

//    let period = Duration::from_millis(250);
//    let count = 50;
//    let separation = Duration::from_millis(10);
//    println!("period = {:?}", period);
//    println!("count = {:?}", count);
//    println!("separation = {:?}", separation);
//    pendulum_museum1(period, count, separation).await;

//    // Under is:
//    // - period > separation * count / 2
//    let period = Duration::from_secs(1);
//    let count = 20;
//    let separation = Duration::from_millis(20);
//    println!("period = {:?}", period);
//    println!("count = {:?}", count);
//    println!("separation = {:?}", separation);
//    pendulum_museum1(period, count, separation).await;

//    // Over
//    // - period < separation * count / 2
//    let period = Duration::from_secs(1);
//    let count = 10;
//    let separation = Duration::from_millis(500);
//    println!("period = {:?}", period);
//    println!("count = {:?}", count);
//    println!("separation = {:?}", separation);
//    pendulum_museum1(period, count, separation).await;


//    // Exbibit 2: Running down the room.


//    // Uniform here is defined by `delay`.
//    let period = Duration::from_millis(500);
//    let range = 0..=6;
//    let delay = period / range.size_hint().0 as u32;
//    println!("period = {:?}", range);
//    println!("range = {:?}", range);
//    println!("delay = {:?}", delay);
//    pendulum_museum2(period, range, delay).await;

//    // Ran too fast.
//    let period = Duration::from_millis(333);
//    let range = -3..=3;
//    let delay = Duration::from_millis(20);
//    println!("range = {:?}", range);
//    println!("delay = {:?}", delay);
//    pendulum_museum2(period, range, delay).await;

//    // Ran too slow.
//    let period = Duration::from_millis(333);
//    let range = -3..=3;
//    let delay = Duration::from_millis(125);
//    println!("range = {:?}", range);
//    println!("delay = {:?}", delay);
//    pendulum_museum2(period, range, delay).await;


//    // Exbibit 3: The bench next to the bathroom.


//    pendulum_museum1(Duration::from_millis(750),
//                     75,
//                     Duration::from_millis(15)).await;
//    pendulum_museum2(Duration::from_millis(1492),
//                     0..=75,
//                     Duration::from_millis(43)).await;


//    // Exhibit 4: Leaving the building.


//    pendulum_museum1(Duration::from_secs(2),
//                     2,
//                     Duration::from_secs(0)).await;
//    pendulum_museum2(Duration::from_secs(3),
//                     0..=1,
//                     Duration::from_millis(125)).await;
//}
