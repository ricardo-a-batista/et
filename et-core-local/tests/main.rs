use setup::*;

use cucumber::World;

mod setup;

#[tokio::main]
async fn main() {
    SetupWorld::cucumber()
        .fail_on_skipped()
        .run("./tests/features/setup.feature")
        .await;
}
