mod shared;

fn main() {
    let lines = shared::get_lines().enumerate();
    let mut marker = lines.take(3).collect();
    
}
