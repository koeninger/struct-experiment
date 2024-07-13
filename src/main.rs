#[macro_use]
extern crate hello_world_macro;

ok_struct!(Buy, checksum, global_seq_num, tstamp);
ok_struct!(Ping, checksum);
ok_struct!(Wtf, tstamp, checksum);

fn main() {
    let b = Buy { global_seq_num: 23, tstamp: 42, ..Default::default() };
    println!("{b:?}");

    let p: Ping = Default::default();
    println!("{p:?}");

    let w = Wtf { tstamp: 42, ..Default::default() };
    println!("{w:?}");
}
