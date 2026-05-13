// use art::kinds::PrimaryColor;
// use art::utils::mix;

use art::PrimaryColor;
use art::mix;
// ^ We can do this due to the re-exports we added in our lib.rs

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
