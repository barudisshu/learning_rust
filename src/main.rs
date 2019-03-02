//!
//!
//!

struct CommunicationChannel {
    address: String,
    port: u16,
}
impl Drop for CommunicationChannel {
    fn drop(&mut self) {
        println!("Closing port {}:{}", self.address, self.port);
    }
}
impl CommunicationChannel {
    fn create(address: &str, port: u16) -> CommunicationChannel {
        println!("Operning port {}:{}", address, port);
        CommunicationChannel {
            address: address.to_string(),
            port: port,
        }
    }
    fn send(&self, msg: &str) {
        println!("Sent to {}:{} the message '{}'", self.address, self.port, msg);
    }
}
fn main() {
    let channel = CommunicationChannel::create("usb4", 879);
    channel.send("Message 1");
    {
        let channel = CommunicationChannel::create("eth1", 12000);
        channel.send("Message 2");
    }
    channel.send("Message 3");
}
