fn experiment(mut memory: Vec<u64>, noun: u64, verb: u64) -> u64 {
    let mut index = 0;
    memory[1] = noun;
    memory[2] = verb;

    loop {
        let clone = memory.clone();
        match memory[index] {
            1 => {
                memory[clone[index + 3] as usize] = clone[clone[index + 1] as usize] + clone[clone[index + 2] as usize]
            },
            2 => {
                memory[clone[index + 3] as usize] = clone[clone[index + 1] as usize] * clone[clone[index + 2] as usize]
            }
            99 => {
                break;
            }
            _ => {
                panic!("WTF! {:?}", memory);
            }
        }

        index += 4;
    }

    memory[0]
}
fn main() {
    let memory : Vec<u64> = std::fs::read_to_string("inputs.txt").unwrap().split(",").map(|input| input.clone().parse::<u64>().unwrap()).collect();

    for noun in 1..99 {
        for verb in 1..99 {
            if experiment(memory.clone(), noun, verb) == 19690720 {
                print!("{}", 100 * noun + verb);
                break;
            }
        }
    }
}
