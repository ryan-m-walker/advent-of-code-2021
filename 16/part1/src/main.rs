fn main() {
    let input = include_str!("./input.txt");
    let mut reader = PacketReader::from_hex(input);
    reader.read_packet();
}

struct PacketReader {
    pointer: usize,
    data: Vec<u32>,
}

impl PacketReader {
    fn from_hex(hex: &str) -> Self {
        let data = parse_hex(hex);
        dbg!(&data);
        Self { data, pointer: 0 }
    }

    fn read_packet(&mut self) {
        self.read_header();
    }

    fn read_header(&mut self) {
        let version = self.get_bits(self.pointer, 3);
        self.pointer += 3;
        let type_id = self.get_bits(self.pointer, 3);
        self.pointer += 3;

        // self.pointer += 6;

        // dbg!(self.pointer);

        dbg!(version);
        dbg!(type_id);
    }

    fn get_bits(&self, pointer: usize, count: u32) -> String {
        self.data[pointer..pointer + count as usize]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("")
    }

    // fn read_bit(&self) -> u32 {
    // let bit = self.data[self.pointer];
    // self.pointer += 1;
    // bit
    // }
}

fn parse_hex(hex: &str) -> Vec<u32> {
    hex.chars()
        .map(|c| match c {
            '0' => vec![0, 0, 0, 0],
            '1' => vec![0, 0, 0, 1],
            '2' => vec![0, 0, 1, 0],
            '3' => vec![0, 0, 1, 1],
            '4' => vec![0, 1, 0, 0],
            '5' => vec![0, 1, 0, 1],
            '6' => vec![0, 1, 1, 0],
            '7' => vec![0, 1, 1, 1],
            '8' => vec![1, 0, 0, 0],
            '9' => vec![1, 0, 0, 1],
            'A' => vec![1, 0, 1, 0],
            'B' => vec![1, 0, 1, 1],
            'C' => vec![1, 1, 0, 0],
            'D' => vec![1, 1, 0, 1],
            'E' => vec![1, 1, 1, 0],
            'F' => vec![1, 1, 1, 1],
            _ => panic!("unexpected hex character"),
        })
        .flatten()
        .collect()
}
