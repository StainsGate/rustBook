mod reverse;

use reverse::reverse;

fn main() {
    // println!("{}", reverse("dynamic"))

    println!("{}", getNtp())
}

fn getNtp() -> String {
    let address = "jp.pool.ntp.org:123";
    let response: ntp::packet::Packet = ntp::request(address).unwrap();
    let ntp_time = response.transmit_time;
    return ntp_time.to_string();
}
