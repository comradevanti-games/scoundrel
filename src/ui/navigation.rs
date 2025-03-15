pub fn go_right(slot: usize) -> usize {
    if slot == 3 { 0 } else { slot + 1 }
}

pub fn go_left(slot: usize) -> usize {
    if slot == 0 { 3 } else { slot - 1 }
}
