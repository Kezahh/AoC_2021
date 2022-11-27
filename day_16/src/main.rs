#[path = "../../generic/generic.rs"] mod generic;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone)]
struct Packet {
    packet_version: usize,
    type_id: usize,
    packet_number: usize,
    is_literal: bool,
    packets: Vec<usize>,
    length_type_id: usize,
    packet_count: usize,
    packet_length: usize,
    packet_center: Vec<usize>,
}

impl Packet {
    fn get_value(&self, all_packets: &Vec<Packet>) -> usize {
        if self.type_id == 4 {
            // literal
            return self.packet_number;
        } else if self.type_id == 0 {
            // Sum
            let mut sum = 0;
            for p_index in self.packets.iter() {
                sum += all_packets[*p_index].get_value(&all_packets);
            }
            return sum
        } else if self.type_id == 1 {
            // Product
            let mut product = 1;
            for p_index in self.packets.iter() {
                product *= all_packets[*p_index].get_value(&all_packets);
            }
            return product;
        } else if self.type_id == 2 {
            // Minimum
            let mut minimum = 0;
            let mut minimum_set = false;
            for p_index in self.packets.iter() {
                if !minimum_set {
                    minimum = all_packets[*p_index].get_value(&all_packets);
                    minimum_set = true;
                } else {
                    if all_packets[*p_index].get_value(&all_packets) < minimum {
                        minimum = all_packets[*p_index].get_value(&all_packets);
                    }
                }
            }
            return minimum;
        } else if self.type_id == 3 {
            // Maximum
            let mut maximum = 0;
            let mut maximum_set = false;
            for p_index in self.packets.iter() {
                if !maximum_set {
                    maximum = all_packets[*p_index].get_value(&all_packets);
                    maximum_set = true;
                } else {
                    if all_packets[*p_index].get_value(&all_packets) > maximum {
                        maximum = all_packets[*p_index].get_value(&all_packets);
                    }
                }
            }
            return maximum;
        } else if self.type_id == 5 {
            if all_packets[self.packets[0]].get_value(&all_packets) > all_packets[self.packets[1]].get_value(&all_packets) {
                return 1;
            } else {
                return 0;
            }
        } else if self.type_id == 6 {
            if all_packets[self.packets[0]].get_value(&all_packets) < all_packets[self.packets[1]].get_value(&all_packets) {
                return 1;
            } else {
                return 0;
            }
        } else if self.type_id == 7 {
            if all_packets[self.packets[0]].get_value(&all_packets) == all_packets[self.packets[1]].get_value(&all_packets) {
                return 1;
            } else {
                return 0;
            }
        }

        return 0;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input_packet = "D2FE28";
        let input_binary = hex_string_to_binary_vector(input_packet);

        let all_packets = decode_all_packets(&input_binary);

        assert!(all_packets.len() == 1);
        assert!(all_packets[0].packet_version == 6);
        assert!(all_packets[0].type_id == 4);
        assert!(all_packets[0].is_literal == true);
        assert!(all_packets[0].packet_number == 2021);
        assert!(all_packets[0].packets.len() == 0);
    }

    #[test]
    fn example_2() {
        let input_packet = "38006F45291200";
        let input_binary = hex_string_to_binary_vector(input_packet);

        let all_packets = decode_all_packets(&input_binary);
        let big_packet = all_packets[2].clone();

        println!("{:?}", all_packets);

        assert!(all_packets.len() == 3);
        assert!(big_packet.packet_version == 1);
        assert!(big_packet.type_id == 6);
        assert!(big_packet.length_type_id == 0);
        assert!(big_packet.packets[0] == 0);
        assert!(big_packet.packets[1] == 1);
        
    }

    #[test]
    fn example_3() {
        let input_packet = "EE00D40C823060";
        let input_binary = hex_string_to_binary_vector(input_packet);

        let all_packets = decode_all_packets(&input_binary);
        let big_packet = all_packets[3].clone();

        println!("{:?}", all_packets);

        assert!(all_packets.len() == 4);
        assert!(big_packet.packet_version == 7);
        assert!(big_packet.type_id == 3);
        assert!(big_packet.length_type_id == 1);
        assert!(big_packet.packets[0] == 0);
        assert!(big_packet.packets[1] == 1);
        assert!(big_packet.packets[2] == 2);
        assert!(all_packets[0].packet_number == 1);
        assert!(all_packets[1].packet_number == 2);
        assert!(all_packets[2].packet_number == 3);
    }

    #[test]
    fn example_4() {
        let input_packet = "8A004A801A8002F478";
        let input_binary = hex_string_to_binary_vector(input_packet);

        let all_packets = decode_all_packets(&input_binary);

        println!("{:?}", all_packets);

        assert!(all_packets.len() == 4);
        assert!(all_packets[0].is_literal == true);
        assert!(all_packets[0].packet_version == 6);
        assert!(all_packets[1].packet_version == 5);
        assert!(all_packets[2].packet_version == 1);
        assert!(all_packets[3].packet_version == 4);
        assert!(get_version_sum(&all_packets) == 16);
    }

    #[test]
    fn example_5() {
        let input_packet = "620080001611562C8802118E34";
        let input_binary = hex_string_to_binary_vector(input_packet);

        let all_packets = decode_all_packets(&input_binary);

        println!("{:?}", all_packets);
        println!("Version sum = {}", get_version_sum(&all_packets));

        assert!(all_packets.len() == 7);
        assert!(all_packets[0].is_literal == true);
        assert!(all_packets[1].is_literal == true);
        assert!(all_packets[2].is_literal == false);
        assert!(all_packets[3].is_literal == true);
        assert!(all_packets[4].is_literal == true);
        assert!(all_packets[5].is_literal == false);
        assert!(all_packets[6].is_literal == false);
        assert!(get_version_sum(&all_packets) == 12);
    }

    #[test]
    fn example_6() {
        let input_packet = "C0015000016115A2E0802F182340";
        let input_binary = hex_string_to_binary_vector(input_packet);

        let all_packets = decode_all_packets(&input_binary);

        println!("{:?}", all_packets);
        println!("Version sum = {}", get_version_sum(&all_packets));

        assert!(all_packets.len() == 7);
        assert!(all_packets[0].is_literal == true);
        assert!(all_packets[1].is_literal == true);
        assert!(all_packets[2].is_literal == false);
        assert!(all_packets[3].is_literal == true);
        assert!(all_packets[4].is_literal == true);
        assert!(all_packets[5].is_literal == false);
        assert!(all_packets[6].is_literal == false);
        assert!(get_version_sum(&all_packets) == 23);
    }

    #[test]
    fn example_7() {
        let input_packet = "A0016C880162017C3686B18A3D4780";
        let input_binary = hex_string_to_binary_vector(input_packet);

        let all_packets = decode_all_packets(&input_binary);

        println!("{:?}", all_packets);
        println!("Version sum = {}", get_version_sum(&all_packets));

        assert!(all_packets.len() == 8);
        assert!(all_packets[0].is_literal == true);
        assert!(all_packets[1].is_literal == true);
        assert!(all_packets[2].is_literal == true);
        assert!(all_packets[3].is_literal == true);
        assert!(all_packets[4].is_literal == true);
        assert!(all_packets[5].is_literal == false);
        assert!(all_packets[6].is_literal == false);
        assert!(all_packets[7].is_literal == false);
        assert!(get_version_sum(&all_packets) == 31);
    }

    #[test]
    fn part_1() {
        let input_file_lines = generic::read_in_file("input.txt");
        let input_packet: &str = &*input_file_lines[0];
        let input_binary = hex_string_to_binary_vector(input_packet);

        let all_packets = decode_all_packets(&input_binary);
        println!("{:?}", all_packets);
        println!("Version sum = {}", get_version_sum(&all_packets));
    }

    #[test]
    fn example_8() {
        let input_packet = "C200B40A82";
        let input_binary = hex_string_to_binary_vector(input_packet);

        let all_packets = decode_all_packets(&input_binary);
        let packet_value = all_packets.last().unwrap().get_value(&all_packets);

        println!("{:?}", all_packets);
        println!("Packet value = {:?}", packet_value);

        assert!(all_packets.len() == 3);
        assert!(packet_value == 3);
    }

    #[test]
    fn example_9() {
        let input_packet = "04005AC33890";
        let input_binary = hex_string_to_binary_vector(input_packet);

        let all_packets = decode_all_packets(&input_binary);
        let packet_value = all_packets.last().unwrap().get_value(&all_packets);

        println!("{:?}", all_packets);
        println!("Packet value = {:?}", packet_value);

        assert!(all_packets.len() == 3);
        assert!(packet_value == 54);
    }

    #[test]
    fn example_10() {
        let input_packet = "880086C3E88112";
        let input_binary = hex_string_to_binary_vector(input_packet);

        let all_packets = decode_all_packets(&input_binary);
        let packet_value = all_packets.last().unwrap().get_value(&all_packets);

        println!("{:?}", all_packets);
        println!("Packet value = {:?}", packet_value);

        assert!(all_packets.len() == 4);
        assert!(packet_value == 7);
    }

    #[test]
    fn example_11() {
        let input_packet = "CE00C43D881120";
        let input_binary = hex_string_to_binary_vector(input_packet);

        let all_packets = decode_all_packets(&input_binary);
        let packet_value = all_packets.last().unwrap().get_value(&all_packets);

        println!("{:?}", all_packets);
        println!("Packet value = {:?}", packet_value);

        assert!(all_packets.len() == 4);
        assert!(packet_value == 9);
    }

    #[test]
    fn example_12() {
        let input_packet = "D8005AC2A8F0";
        let input_binary = hex_string_to_binary_vector(input_packet);

        let all_packets = decode_all_packets(&input_binary);
        let packet_value = all_packets.last().unwrap().get_value(&all_packets);

        println!("{:?}", all_packets);
        println!("Packet value = {:?}", packet_value);

        assert!(all_packets.len() == 3);
        assert!(packet_value == 1);
    }

    #[test]
    fn example_13() {
        let input_packet = "F600BC2D8F";
        let input_binary = hex_string_to_binary_vector(input_packet);

        let all_packets = decode_all_packets(&input_binary);
        let packet_value = all_packets.last().unwrap().get_value(&all_packets);

        println!("{:?}", all_packets);
        println!("Packet value = {:?}", packet_value);

        assert!(all_packets.len() == 3);
        assert!(packet_value == 0);
    }

    #[test]
    fn example_14() {
        let input_packet = "9C005AC2F8F0";
        let input_binary = hex_string_to_binary_vector(input_packet);

        let all_packets = decode_all_packets(&input_binary);
        let packet_value = all_packets.last().unwrap().get_value(&all_packets);

        println!("{:?}", all_packets);
        println!("Packet value = {:?}", packet_value);

        assert!(all_packets.len() == 3);
        assert!(packet_value == 0);
    }

    #[test]
    fn example_15() {
        let input_packet = "9C0141080250320F1802104A08";
        let input_binary = hex_string_to_binary_vector(input_packet);

        let all_packets = decode_all_packets(&input_binary);
        let packet_value = all_packets.last().unwrap().get_value(&all_packets);

        println!("{:?}", all_packets);
        println!("Packet value = {:?}", packet_value);

        assert!(all_packets.len() == 7);
        assert!(packet_value == 1);
    }

    #[test]
    fn part_2() {
        let input_file_lines = generic::read_in_file("input.txt");
        let input_packet: &str = &*input_file_lines[0];
        let input_binary = hex_string_to_binary_vector(input_packet);

        let all_packets = decode_all_packets(&input_binary);
        let packet_value = all_packets.last().unwrap().get_value(&all_packets);
        println!("{:?}", all_packets);
        println!("Packet value = {:?}", packet_value);
    }
}

fn get_version_sum(all_packets: &Vec<Packet>) -> usize {
    let mut sum:usize = 0;
    for p in all_packets {
        sum += p.packet_version;
    }

    return sum;
}

fn decode_all_packets(input_binary: &Vec<usize>) -> Vec<Packet> {
    let mut finished = false;
    let mut start_index = 0;
    let mut all_packets: Vec<Packet> = Vec::new();
    let mut packets_to_parse: Vec<usize> = Vec::new();

    while start_index <= input_binary.len() - 6 {
        let mut packet_version: usize = binary_vector_to_dec(input_binary[start_index..(start_index + 3)].to_vec());
        let mut type_id: usize = binary_vector_to_dec(input_binary[(start_index+3)..(start_index+6)].to_vec());

        if type_id == 4 {
            let new_packet: Packet;
            (start_index, new_packet) = decode_literal_packet(&input_binary, start_index);
            all_packets.push(new_packet);
        } else {
            let new_packet: Packet;
            (start_index, new_packet) = decode_operator_packet(&input_binary, start_index, &mut all_packets);
            all_packets.push(new_packet);
        }

        break;
        //println!("len = {}, start_index = {}", input_binary.len(), start_index);
    }

    return all_packets;
}

fn decode_packet_header(input_binary: &Vec<usize>, arg_start_index: usize) -> (usize, usize, usize) {
    // returns start_index, packet_version, type_id
    let mut start_index = arg_start_index;
    let packet_version: usize = binary_vector_to_dec(input_binary[start_index..(start_index + 3)].to_vec());
    start_index += 3;
    let type_id: usize = binary_vector_to_dec(input_binary[start_index..(start_index + 3)].to_vec());
    start_index += 3;

    return (start_index, packet_version, type_id);
}

fn decode_operator_packet(input_binary: &Vec<usize>, arg_start_index: usize, all_packets: &mut Vec<Packet>) -> (usize, Packet) {
    // start_index should be 0 and include the header.
    let mut start_index: usize;
    let packet_version: usize;
    let type_id: usize;
    (start_index, packet_version, type_id) = decode_packet_header(&input_binary, arg_start_index);

    let mut sub_packets: Vec<usize> = Vec::new();

    let length_type_id: usize = input_binary[start_index];
    let mut packet_count = 0;
    let mut packet_length = 0;
    start_index += 1;
    if length_type_id == 0 {
        let sub_packet_length = binary_vector_to_dec(input_binary[start_index..(start_index + 15)].to_vec());
        packet_length = sub_packet_length;
        start_index += 15;

        let end_index: usize = start_index + sub_packet_length;
        while (start_index < (end_index - 6)) {     //6 is len of a header. no point continuing if no header.
            let temp_packet_version: usize;
            let temp_type_id: usize;
            (start_index, temp_packet_version, temp_type_id) = decode_packet_header(&input_binary, start_index);
            start_index -= 6;

            if temp_type_id == 4 {
                let new_packet: Packet;
                (start_index, new_packet) = decode_literal_packet(&input_binary, start_index);
                all_packets.push(new_packet);
                sub_packets.push(all_packets.len() - 1);
            } else {
                let new_packet: Packet;
                println!("doing the operator packet");
                (start_index, new_packet) = decode_operator_packet(&input_binary, start_index, all_packets);
                all_packets.push(new_packet);
                sub_packets.push(all_packets.len() - 1);
            }
        }
    } else if length_type_id == 1 {
        let packet_count = binary_vector_to_dec(input_binary[start_index..(start_index + 11)].to_vec());
        start_index += 11;

        for i in 0..packet_count {
            let temp_packet_version: usize;
            let temp_type_id: usize;
            (start_index, temp_packet_version, temp_type_id) = decode_packet_header(&input_binary, start_index);
            start_index -= 6;

            if temp_type_id == 4 {
                let new_packet: Packet;
                (start_index, new_packet) = decode_literal_packet(&input_binary, start_index);
                all_packets.push(new_packet);
                sub_packets.push(all_packets.len() - 1);
            } else {
                let new_packet: Packet;
                println!("doing the operator packet");
                (start_index, new_packet) = decode_operator_packet(&input_binary, start_index, all_packets);
                all_packets.push(new_packet);
                sub_packets.push(all_packets.len() - 1);
            }
        }
    }
    
    return (start_index, Packet{
        packet_version: packet_version,
        type_id: type_id,
        packet_number: 0,
        is_literal: false,
        packets: sub_packets.clone(),
        length_type_id: length_type_id,
        packet_count: packet_count,
        packet_length: packet_length,
        packet_center: Vec::new(),
    });
}

fn decode_literal_packet(input_binary: &Vec<usize>, arg_start_index: usize) -> (usize, Packet) {
    // start_index should be 0 and include the header.
    let mut start_index: usize;
    let packet_version: usize;
    let type_id: usize;
    (start_index, packet_version, type_id) = decode_packet_header(&input_binary, arg_start_index);

    let mut packet_number: Vec<usize> = Vec::new();
    let mut start_bit: usize = input_binary[start_index];
    while start_bit != 0 {
        packet_number.append(&mut input_binary[(start_index+1)..(start_index+5)].to_vec());
        start_index += 5;
        start_bit = input_binary[start_index];
    }
    packet_number.append(&mut input_binary[(start_index+1)..(start_index+5)].to_vec());
    start_index += 5;

    return (start_index, Packet {
        packet_version: packet_version,
        type_id: type_id,
        packet_number: binary_vector_to_dec(packet_number),
        is_literal: true,
        packets: Vec::new(),
        length_type_id: 0,
        packet_count: 0,
        packet_length: 0,
        packet_center: Vec::new(),
    });
}

fn hex_string_to_binary_vector(hex_string: &str) -> Vec<usize> {
    let mut output_vector: Vec<usize> = Vec::new();
    for c in hex_string.chars() {
        let mut char_num: usize = (c as usize) - 48;  //48 is ascii for '0'
        if char_num > 9 {
            // character is a letter
            char_num = (c as usize) - 65 + 10;  //65 is 'A' in ascii.
        }
        for i in (0..4).rev() {
            output_vector.push((char_num >> i) & 1);
        }
    }
    return output_vector;
}

fn binary_vector_to_dec(binary_vector: Vec<usize>) -> usize {
    let mut sum = 0;
    for i in 0..binary_vector.len() {
        sum += binary_vector[i] << (binary_vector.len() - i - 1);
    }
    return sum;
}