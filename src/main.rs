// fn main() {
//     println!("Hello, world!");
// }

const MINUTES_IN_HOUR: u32 = 60;
const SECONDS_IN_MINUTES: u32 = 60;
const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTES * MINUTES_IN_HOUR;
fn main(){
    let total = 30;
    let total_em_segundos = total * SECONDS_IN_HOUR;

    print!("Trabalhou { } segundos", total_em_segundos)
}