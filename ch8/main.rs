pub mod basic_stats;
pub mod pig_latin;
pub mod departments;

fn main() {
    let lst = vec![4, 2, 1, 3, 4];
    let (median, mode) = basic_stats::find_stats(&lst);
    println!("Median: {}", median);
    println!("Mode: {}", mode);

    let word = String::from("apple");
    //let word = String::from("Здравствуйте");
    let pl = pig_latin::to_pig_latin(&word);
    println!("Word: {}, Pig latin: {}", word, pl);

    departments::run_interface();
}
