use pi_bit_engine::BitByte;

fn main() {
    let byte = BitByte::new(0b0000_0101);

    println!("Value: {}", byte.value());
    println!("Binary: {:08b}", byte.value());

    let result = byte.set_bit(3);

    println!("Result: {:08b}", result.value());
}
