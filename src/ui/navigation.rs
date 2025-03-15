pub fn go_right(slot: u8) -> u8 {
    if slot == 3 { 0 } else { slot + 1 }
}

pub fn go_left(slot: u8) -> u8 {
    if slot == 0 { 3 } else { slot - 1 }
}
