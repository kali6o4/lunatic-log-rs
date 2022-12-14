use lunatic_log::{
    info,
    subscriber::{fmt::FmtSubscriber, multiple::MultipleSubscribers},
    LevelFilter,
};

fn main() {
    // Setup multiple subscribers
    let subscriber = MultipleSubscribers::new()
        .add_subscriber(FmtSubscriber::new(LevelFilter::Info))
        .add_subscriber(FmtSubscriber::new(LevelFilter::Info));

    // Initialize multiple subscribers
    lunatic_log::init(subscriber);

    // Log message
    info!("Hello, {}", "World");

    // Wait for events to propagate
    lunatic::sleep(std::time::Duration::from_millis(50));
}
