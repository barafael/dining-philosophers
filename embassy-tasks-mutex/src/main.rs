#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::sync::atomic::{AtomicBool, Ordering};
use embassy_executor::Spawner;
use embassy_rp::{clocks::RoscRng, gpio};
use embassy_time::{Duration, Timer};
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};

use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};
use rand_core::RngCore;

#[derive(Debug, Clone, Default)]
pub enum Activity {
    #[default]
    Thinking,
    Eating,
}

static RUNNING: AtomicBool = AtomicBool::new(false);

static FORK_1: Mutex<ThreadModeRawMutex, ()> = Mutex::new(());
static FORK_2: Mutex<ThreadModeRawMutex, ()> = Mutex::new(());
static FORK_3: Mutex<ThreadModeRawMutex, ()> = Mutex::new(());
static FORK_4: Mutex<ThreadModeRawMutex, ()> = Mutex::new(());
static FORK_5: Mutex<ThreadModeRawMutex, ()> = Mutex::new(());

static RNG: Mutex<ThreadModeRawMutex, RoscRng> = Mutex::new(RoscRng);

const TIMEOUT: bool = true;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_25, Level::Low);

    let random = RNG.lock().await.next_u32();
    defmt::info!("Random number: {}", random);

    let philo_1 = philosopher_1();
    let philo_2 = philosopher_2();
    let philo_3 = philosopher_3();
    let philo_4 = philosopher_4();
    let philo_5 = philosopher_5();

    spawner.spawn(philo_1).unwrap();
    spawner.spawn(philo_2).unwrap();
    spawner.spawn(philo_3).unwrap();
    spawner.spawn(philo_4).unwrap();
    spawner.spawn(philo_5).unwrap();

    loop {
        if RUNNING.load(Ordering::SeqCst) && all_forks_locked() {
            defmt::println!(
                "Everybody is hungry but no one can eat. No thinking beyond this point."
            );
            RUNNING.store(false, Ordering::SeqCst);
        }
        if RUNNING.load(Ordering::SeqCst) {
            led.set_high();
        } else {
            led.set_low();
        }
        Timer::after(Duration::from_millis(50)).await;
    }
}

fn all_forks_locked() -> bool {
    FORK_1.try_lock().is_err()
        && FORK_2.try_lock().is_err()
        && FORK_3.try_lock().is_err()
        && FORK_4.try_lock().is_err()
        && FORK_5.try_lock().is_err()
}

macro_rules! philosopher {
    ( $f1:expr, $f2:expr, $n:expr, $timeout:expr) => {
        let mut activity = Activity::default();
        loop {
            match activity {
                Activity::Thinking => {
                    let duration = RNG.lock().await.next_u32() as u8 / (u8::MAX / 2);
                    defmt::println!("Philosopher {} is thinking.", $n);
                    Timer::after(Duration::from_millis(duration as u64)).await;
                    activity = Activity::Eating;
                }
                Activity::Eating => {
                    defmt::println!("Philosopher {} is hungry!", $n);
                    let eat_duration = RNG.lock().await.next_u32() as u8 / (u8::MAX / 2);
                    let wait_duration = RNG.lock().await.next_u32() as u8 / (u8::MAX / 32);
                    let f1 = $f1.lock().await;
                    let f2 = if !$timeout {
                        $f2.lock().await
                    } else {
                        let f2 = $f2.lock();
                        let timeout = embassy_time::with_timeout(
                            Duration::from_millis(wait_duration as u64),
                            f2,
                        );
                        match timeout.await {
                            Ok(f2) => f2,
                            Err(_e) => {
                                defmt::println!(
                                    "Philosopher {} gives up, goes back to thinking.",
                                    $n
                                );
                                activity = Activity::Thinking;
                                continue;
                            }
                        }
                    };
                    RUNNING.store(true, Ordering::SeqCst);
                    defmt::println!("Philosopher {} is eating!", $n);
                    Timer::after(Duration::from_millis(eat_duration as u64)).await;
                    drop(f1);
                    drop(f2);
                    activity = Activity::Thinking;
                }
            }
        }
    };
}

#[embassy_executor::task]
async fn philosopher_1() {
    philosopher!(FORK_1, FORK_2, 1, TIMEOUT);
}

#[embassy_executor::task]
async fn philosopher_2() {
    philosopher!(FORK_2, FORK_3, 2, TIMEOUT);
}

#[embassy_executor::task]
async fn philosopher_3() {
    philosopher!(FORK_3, FORK_4, 3, TIMEOUT);
}

#[embassy_executor::task]
async fn philosopher_4() {
    philosopher!(FORK_4, FORK_5, 4, TIMEOUT);
}

#[embassy_executor::task]
async fn philosopher_5() {
    philosopher!(FORK_5, FORK_1, 5, TIMEOUT);
}
