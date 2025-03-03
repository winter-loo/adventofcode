fn get_answer(orig_ins: &Vec<String>) -> Option<(usize, usize)> {
    for i in 0..=99 {
        for j in 0..=99 {
            let mut ins = orig_ins.clone();

            for k in (0..ins.len() - 4).step_by(4) {
                let opr: usize = ins[k].parse().unwrap();
                let mut in1: usize = ins[ins[k+1].parse::<usize>().unwrap()].parse().unwrap();
                let mut in2: usize = ins[ins[k+2].parse::<usize>().unwrap()].parse().unwrap();
                let out1_addr = ins[k+3].parse::<usize>().unwrap();

                if k == 0 {
                    in1 = i;
                    in2 = j;
                    ins[1] = i.to_string();
                    ins[2] = j.to_string();
                }

                if opr == 1 {
                    ins[out1_addr] = (in1 + in2).to_string();
                } else if opr == 2 {
                    ins[out1_addr] = (in1 * in2).to_string();
                } else if opr == 99 {
                    break;
                } else {
                    panic!("unknown operator");
                }
            }
            if ins[0].parse::<usize>().unwrap() == 19690720 {
                return Some((i, j));
            }
            println!("{}", ins[0]);
        }
    }
    return None;
}

fn main() {
    let instructions = std::fs::read_to_string("input.txt").unwrap();
    let orig_ins: Vec<String> = instructions.split(",").map(|s| s.to_string()).collect();
    if let Some((noun, verb)) = get_answer(&orig_ins) {
        println!("noun = {}, verb = {}, answser = {}", noun, verb, 100 * noun + verb);
    }
}
