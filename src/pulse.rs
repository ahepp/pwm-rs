use std::boxed::Box;
use std::error::Error;
use std::time::Duration;

use tokio::sync::mpsc::Sender;
use tokio::time::sleep;

pub async fn do_pulse(
    file: Sender<u8>,
    duration: Duration,
) -> Result<(), Box<dyn Error>> {
    file.send(49).await?; // ascii 1
    sleep(duration).await;
    file.send(48).await?;
    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use tokio::sync::mpsc::channel;

    #[tokio::test]
    async fn pulse_writes_one_by_tick() {
        let tick = Duration::from_millis(10);

        let (tx, mut rx) = channel(8);
        let duration = Duration::from_millis(100);
        let future = do_pulse(tx, duration);

        (_, _) = tokio::join!(future, async {
            sleep(tick).await;
            assert_eq!(rx.try_recv(), Ok(49));
        });
    }

    #[tokio::test]
    async fn pulse_writes_zero_by_tick_after_duration() {
        let tick = Duration::from_millis(10);

        let (tx, mut rx) = channel(8);
        let duration = Duration::from_millis(100);
        let future = do_pulse(tx, duration);

        (_, _) = tokio::join!(future, async {
            sleep(duration + tick).await;
            let _ = rx.try_recv();
            assert_eq!(rx.try_recv(), Ok(48));
        });
    }

    #[tokio::test]
    #[should_panic]
    async fn pulse_writes_nothing_between_first_tick_and_one_before_duration() {
        let tick = Duration::from_millis(10);

        let (tx, mut rx) = channel(8);
        let duration = Duration::from_millis(100);
        let future = do_pulse(tx, duration);

        (_, _) = tokio::join!(future, async {
            sleep(tick).await;
            let _ = rx.try_recv();
            sleep(duration - tick - tick).await;
            rx.try_recv().unwrap();
        });
    }
}
