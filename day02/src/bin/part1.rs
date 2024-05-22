fn main() -> std::io::Result<()> {
    let instructions = std::fs::read_to_string("input.txt").unwrap();
    let mut ins: Vec<String> = instructions.split(",").map(|s| s.to_string()).collect();

    ins[1] = "12".to_string();
    ins[2] = "2".to_string();

    for i in (0..ins.len() - 4).step_by(4) {
        let opr: usize = ins[i].parse().unwrap();
        let in1: usize = ins[ins[i+1].parse::<usize>().unwrap()].parse().unwrap();
        let in2: usize = ins[ins[i+2].parse::<usize>().unwrap()].parse().unwrap();
        let out1_addr = ins[i+3].parse::<usize>().unwrap();
        let out1 = ins.get_mut(out1_addr).unwrap();
        // println!("{} {} {} {}", opr, in1, in2, out1);
        if opr == 1 {
            *out1 = (in1 + in2).to_string();
        } else if opr == 2 {
            *out1 = (in1 * in2).to_string();
        } else if opr == 99 {
            break;
        } else {
            panic!("unknown operator");
        }
    }
    println!("{}", ins[0]);

    Ok(()) 
}
