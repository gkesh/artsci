mod reader;
mod sys;

use rand::Rng;

fn main() {
    let user: String = sys::get_user();
    let base_dir: &str = &format!("/home/{}/.config/artsci", user);
    println!("{}", base_dir);
    let ars_dir: &str = &format!("{}/ascii", base_dir);
    let ars_files: Vec<String> = reader::read_ars_dir(ars_dir);

    let ars_files_count: usize = ars_files.len();
    let rand_ars_selection: usize = rand::thread_rng().gen_range(1..=ars_files_count) - 1;

    println!("{}", reader::read_ars(&ars_files[rand_ars_selection]));
}