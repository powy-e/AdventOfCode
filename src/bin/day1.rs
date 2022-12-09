use std::io;

fn main() {
    let mut entry: String = String::new();

    let mut entry_sum: Vec<usize> = Vec::new();
    let mut total = 0;
    io::stdin().read_line(&mut entry).expect("Error");
    while !entry.is_empty() {
        if entry == "\n" {
            entry_sum.push(total);
            total = 0;
        } else {
            let entry_num: usize = entry.trim().parse().expect("NotNumber");
            total += entry_num;
        }

        entry.clear();
        io::stdin().read_line(&mut entry).expect("Erro");
    }
    entry_sum.sort_unstable();

    let mut sum = 0;
    for (i, num) in entry_sum.iter().rev().enumerate().take(3) {
        println!("{i}:\t{num}");
        sum += num;
    }
    println!("Sum:\t{sum}");
}
