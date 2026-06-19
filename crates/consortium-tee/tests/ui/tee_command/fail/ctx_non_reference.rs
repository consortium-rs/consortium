use consortium_tee::tee_command;

#[tee_command(ctx)]
fn ping(value: u32) -> u32 {
    value
}

fn main() {}
