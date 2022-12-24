mod reader;

use rand::Rng;

fn main() {
    let base_dir: &str = "/home/gkesh/.config/artsci";
    let ars_dir: &str = &format!("{}/ascii", base_dir);
    let ars_files: Vec<String> = reader::read_ars_dir(ars_dir);

    let ars_files_count: usize = ars_files.len();
    let rand_ars_selection: usize = rand::thread_rng().gen_range(1..=ars_files_count) - 1;

    println!("{}", reader::read_ars(&ars_files[rand_ars_selection]));
}