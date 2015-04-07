struct GenericPacket {
    packet_id    : u32,
    packet_size  : u32,
    packet_crc   : u32,
    packet_data  : [u8; 8]
}

fn main() {
println!("Hello!");

let my_pkt = GenericPacket{ packet_id   : 256,
                            packet_size : 8,
                            packet_crc  : 0xdeadbeef,
                            packet_data : [ 0xF,
                                            0xE,
                                            0xE,
                                            0xD,
                                            0xC,
                                            0xA,
                                            0x7,
                                            0x5
                                          ]
                          };

println!("{} {} {} {}", my_pkt.packet_id,
                        my_pkt.packet_size,
                        my_pkt.packet_crc,
                        my_pkt.packet_data[0] );


match parse_type(my_pkt.packet_id) {
    Ok(id)   => println!("ID:  {:?}", id ),
    Err(err) => println!("ERR: {:?}", err),
}



}

fn parse_type(pid: u32) -> Result<u32, i32>{
    match pid {
        255 => Ok(pid),
        _   => Err(-1*(pid as i32) )
    }
}