use cucumber::{given, World};

#[derive(Debug, Default, World)]
pub(crate) struct SetupWorld {}

#[given(expr = "migrations run")]
async fn migrations_run(_world: &mut SetupWorld) {
    todo!()
}
