#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

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

static FORK_1: Mutex<ThreadModeRawMutex, ()> = Mutex::new(());
static FORK_2: Mutex<ThreadModeRawMutex, ()> = Mutex::new(());
static FORK_3: Mutex<ThreadModeRawMutex, ()> = Mutex::new(());
static FORK_4: Mutex<ThreadModeRawMutex, ()> = Mutex::new(());
static FORK_5: Mutex<ThreadModeRawMutex, ()> = Mutex::new(());

static RNG: Mutex<ThreadModeRawMutex, RoscRng> = Mutex::new(RoscRng);

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
        defmt::trace!("led on!");
        led.set_high();
        Timer::after(Duration::from_secs(1)).await;

        defmt::trace!("led off!");
        led.set_low();
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[embassy_executor::task]
async fn philosopher_1() {
    let mut activity = Activity::default();
    loop {
        match activity {
            Activity::Thinking => {
                let duration = RNG.lock().await.next_u32() as u8 / (u8::MAX / 32);
                defmt::println!("Philosopher 1 is thinking.");
                Timer::after(Duration::from_secs(duration as u64)).await;
                activity = Activity::Eating;
            }
            Activity::Eating => {
                defmt::println!("Philosopher 1 is hungry!");
                let duration = RNG.lock().await.next_u32() as u8 / (u8::MAX / 32);
                let f1 = FORK_1.lock().await;
                let f2 = FORK_2.lock().await;
                defmt::println!("Philosopher 1 is eating!");
                Timer::after(Duration::from_secs(duration as u64)).await;
                drop(f1);
                drop(f2);
                activity = Activity::Thinking;
            }
        }
    }
}

#[embassy_executor::task]
async fn philosopher_2() {
    let mut activity = Activity::default();
    loop {
        match activity {
            Activity::Thinking => {
                let duration = RNG.lock().await.next_u32() as u8 / (u8::MAX / 32);
                defmt::println!("Philosopher 2 is thinking.");
                Timer::after(Duration::from_secs(duration as u64)).await;
                activity = Activity::Eating;
            }
            Activity::Eating => {
                defmt::println!("Philosopher 2 is hungry!");
                let duration = RNG.lock().await.next_u32() as u8 / (u8::MAX / 32);
                let f2 = FORK_2.lock().await;
                let f3 = FORK_3.lock().await;
                defmt::println!("Philosopher 2 is eating!");
                Timer::after(Duration::from_secs(duration as u64)).await;
                drop(f2);
                drop(f3);
                activity = Activity::Thinking;
            }
        }
    }
}

#[embassy_executor::task]
async fn philosopher_3() {
    let mut activity = Activity::default();
    loop {
        match activity {
            Activity::Thinking => {
                let duration = RNG.lock().await.next_u32() as u8 / (u8::MAX / 32);
                defmt::println!("Philosopher 3 is thinking.");
                Timer::after(Duration::from_secs(duration as u64)).await;
                activity = Activity::Eating;
            }
            Activity::Eating => {
                defmt::println!("Philosopher 3 is hungry!");
                let duration = RNG.lock().await.next_u32() as u8 / (u8::MAX / 32);
                let f3 = FORK_3.lock().await;
                let f4 = FORK_4.lock().await;
                defmt::println!("Philosopher 3 is eating!");
                Timer::after(Duration::from_secs(duration as u64)).await;
                drop(f3);
                drop(f4);
                activity = Activity::Thinking;
            }
        }
    }
}

#[embassy_executor::task]
async fn philosopher_4() {
    let mut activity = Activity::default();
    loop {
        match activity {
            Activity::Thinking => {
                let duration = RNG.lock().await.next_u32() as u8 / (u8::MAX / 32);
                defmt::println!("Philosopher 4 is thinking.");
                Timer::after(Duration::from_secs(duration as u64)).await;
                activity = Activity::Eating;
            }
            Activity::Eating => {
                defmt::println!("Philosopher 4 is hungry!");
                let duration = RNG.lock().await.next_u32() as u8 / (u8::MAX / 32);
                let f4 = FORK_4.lock().await;
                let f5 = FORK_5.lock().await;
                defmt::println!("Philosopher 4 is eating!");
                Timer::after(Duration::from_secs(duration as u64)).await;
                drop(f4);
                drop(f5);
                activity = Activity::Thinking;
            }
        }
    }
}

#[embassy_executor::task]
async fn philosopher_5() {
    let mut activity = Activity::default();
    loop {
        match activity {
            Activity::Thinking => {
                let duration = RNG.lock().await.next_u32() as u8 / (u8::MAX / 2);
                defmt::println!("Philosopher 5 is thinking.");
                Timer::after(Duration::from_secs(duration as u64)).await;
                activity = Activity::Eating;
            }
            Activity::Eating => {
                defmt::println!("Philosopher 5 is hungry!");
                let duration = RNG.lock().await.next_u32() as u8 / (u8::MAX / 2);
                let f5 = FORK_5.lock().await;
                let f1 = FORK_1.lock().await;
                defmt::println!("Philosopher 5 is eating!");
                Timer::after(Duration::from_secs(duration as u64)).await;
                drop(f5);
                drop(f1);
                activity = Activity::Thinking;
            }
        }
    }
}
