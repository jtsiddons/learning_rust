fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_hat(),
        7 => remove_hat(),
        other => {
            println!("{}", other);
            move_player(other);
        }
    }
}

fn move_player(num_spaces: u8) {}
fn add_hat() {}
fn remove_hat() {}
