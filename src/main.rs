#[macro_use]
extern crate hello_world_macro;

#[derive(Hello, HelloAlt)]
struct Example;

biscuit!(jam, jelly);

ok_struct!(Buy, checksum, global_seq_num, tstamp);
ok_struct!(Ping, checksum);
ok_struct!(Wtf, tstamp, checksum);

fn main() {
    let e = Example {};
    e.hello_world();

    let b: Biscuit = Default::default();
    println!("{b:?}");

    let b = Buy { global_seq_num: 23, ..Default::default() };
    println!("{b:?}");
}
