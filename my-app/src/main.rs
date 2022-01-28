use proc_macro::my_first_proc_macro;

fn main() {
    my_first_proc_macro!(prince);
    my_first_proc_macro!(alex);
    my_first_proc_macro!(jen);
    my_first_proc_macro!(nicola);
    my_first_proc_macro!(dustin);
    my_first_proc_macro!(marcus);
    println!("{}", jen());
    println!("{}", nicola());
    println!("{}", marcus());
}
