extern crate chrono;
use chrono::*;
fn main() {
    loop {
        #[cfg(feature="default")]
        let time = chrono::Local::now();
        
        #[cfg(feature="UTC")]
        let time = chrono::UTC::now();

        #[cfg(feature="debug")]
        println!("{}h {}m {}s", time.hour(), time.minute(), time.second());

        if (time.year().to_string() == "2017") {
            println!("Goodbye 2016!"); std::thread::sleep_ms(10000); break;
        } else { std::thread::sleep_ms(1000);continue; }
    }
}
