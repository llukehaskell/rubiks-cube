// start 7:27

use colored::Colorize;
use std::io;


fn main() {
    // println!("{}", "Hello, world!".purple());
    let mut c: [u8; 54] = [0,0,0, 0,0,0, 0,0,0, 4,4,4, 1,1,1, 2,2,2, 3,3,3, 4,4,4, 1,1,1, 2,2,2, 3,3,3, 4,4,4, 1,1,1, 2,2,2, 3,3,3, 5,5,5, 5,5,5, 5,5,5];
    
    // print!("{}","◼".purple());

    let mut steps:Vec<String> = Vec::new();

    let mut turn: String = String::new();
    loop {
        turn.clear();
        io::stdin().read_line(&mut turn).expect("failed to read input");
        
        if turn.trim() == "quit" {
            break;
        }
        
        match turn.trim() {
            "r"  => {turn_r(&mut c); steps.push(turn.trim().to_string());},
            "l"  => {turn_l(&mut c); steps.push(turn.trim().to_string());},
            "u"  => {turn_u(&mut c); steps.push(turn.trim().to_string());},
            "d"  => {turn_d(&mut c); steps.push(turn.trim().to_string());},
            "f"  => {turn_f(&mut c); steps.push(turn.trim().to_string());},
            "b"  => {turn_b(&mut c); steps.push(turn.trim().to_string());},
            "r'" => {turn_rp(&mut c); steps.push(turn.trim().to_string());},
            "l'" => {turn_lp(&mut c); steps.push(turn.trim().to_string());},
            "u'" => {turn_up(&mut c); steps.push(turn.trim().to_string());},
            "d'" => {turn_dp(&mut c); steps.push(turn.trim().to_string());},
            "f'" => {turn_fp(&mut c); steps.push(turn.trim().to_string());},
            "b'" => {turn_bp(&mut c); steps.push(turn.trim().to_string());}
            "reset" => {
                c = [0,0,0, 0,0,0, 0,0,0, 4,4,4, 1,1,1, 2,2,2, 3,3,3, 4,4,4, 1,1,1, 2,2,2, 3,3,3, 4,4,4, 1,1,1, 2,2,2, 3,3,3, 5,5,5, 5,5,5, 5,5,5];
                steps.clear();
            },
            "steps" => print_steps(steps.clone()),
            _ => print!("invalid turn\n")
        }
        print_cube(c);
    }
}

fn print_cube(c: [u8; 54]){
    print!   ("      {} {} {}            \n", c[0], c[1], c[2]);
    print!   ("      {} {} {}            \n", c[3], c[4], c[5]);
    print!   ("      {} {} {}            \n", c[6], c[7], c[8]);
    print!("{} {} {} {} {} {} {} {} {} {} {} {}\n",  c[9], c[10], c[11], c[12], c[13], c[14], c[15], c[16], c[17], c[18], c[19], c[20]);
    print!("{} {} {} {} {} {} {} {} {} {} {} {}\n", c[21], c[22], c[23], c[24], c[25], c[26], c[27], c[28], c[29], c[30], c[31], c[32]);
    print!("{} {} {} {} {} {} {} {} {} {} {} {}\n", c[33], c[34], c[35], c[36], c[37], c[38], c[39], c[40], c[41], c[42], c[43], c[44]);
    print!   ("      {} {} {}            \n", c[45], c[46], c[47]);
    print!   ("      {} {} {}            \n", c[48], c[49], c[50]);
    print!   ("      {} {} {}            \n", c[51], c[52], c[53]);
}

fn print_steps(steps: Vec<String>){
    for x in 0..steps.len() {
        print!("{}, ", steps[x]);
    }
    print!("\n");
}

fn shift (c: &mut [u8; 54], w:usize, x:usize, y:usize, z:usize){
    let mut tmp1:u8 = 8;
    let mut tmp2:u8 = 8;
    tmp1 = c[x];
    c[x] = c[w];
    tmp2 = c[y];
    c[y] = tmp1;
    tmp1 = c[z];
    c[z] = tmp2;
    c[w] = tmp1;
}

fn spin (c: &mut [u8; 54], l:usize, m:usize, n:usize, o:usize, p:usize, q:usize, r:usize, s:usize){
    //l, n, p, r -> corner pieces starting from topleft going clockwise/counterclockwise
    let mut tmp1:u8 = 8;
    let mut tmp2:u8 = 8;
    tmp1 = c[n];
    c[n] = c[l];
    tmp2 = c[p];
    c[p] = tmp1;
    tmp1 = c[r];
    c[r] = tmp2;
    c[l] = tmp1;
    
    //m, o, q, s -> edge pieces starting from top going clockwise/counterclockwise
    tmp1 = c[o];
    c[o] = c[m];
    tmp2 = c[q];
    c[q] = tmp1;
    tmp1 = c[s];
    c[s] = tmp2;
    c[m] = tmp1;
}

fn turn_r (c: &mut [u8; 54]){
    shift(&mut *c, 14, 2, 42, 47);
    shift(&mut *c, 26, 5, 30, 50);
    shift(&mut *c, 38, 8, 18, 53);
    spin(&mut *c, 15, 16, 17, 29, 41, 40, 39, 27);
}

fn turn_l (c: &mut [u8; 54]){
    shift(&mut *c, 12, 45, 44, 0);
    shift(&mut *c, 24, 48, 32, 3);
    shift(&mut *c, 36, 51, 20, 6);
    spin(&mut *c, 9, 10, 11, 23, 35, 34, 33, 21);
}

fn turn_u (c: &mut [u8; 54]){
    shift(&mut *c, 14, 11, 20, 17);
    shift(&mut *c, 13, 10, 19, 16);
    shift(&mut *c, 12, 9, 18, 15);
    spin(&mut *c, 0, 1, 2, 5, 8, 7, 6, 3);
}

fn turn_d (c: &mut [u8; 54]){
    shift(&mut *c, 36, 39, 42, 33);
    shift(&mut *c, 37, 40, 43, 34);
    shift(&mut *c, 38, 41, 44, 35);
    spin(&mut *c, 45, 46, 47, 50, 53, 52, 51, 48);
}

fn turn_f (c: &mut [u8; 54]){
    shift(&mut *c, 6, 15, 47, 35);
    shift(&mut *c, 7, 27, 46, 23);
    shift(&mut *c, 8, 39, 45, 11);
    spin(&mut *c, 12, 13, 14, 26, 38, 37, 36, 24);
}

fn turn_b (c: &mut [u8; 54]){
    shift(&mut *c, 2, 9, 51, 41);
    shift(&mut *c, 1, 21, 52, 29);
    shift(&mut *c, 0, 33, 53, 17);
    spin(&mut *c, 18, 19, 20, 32, 44, 43, 42, 30);
}

fn turn_rp (c: &mut [u8; 54]){
    shift(&mut *c, 14, 47, 42, 2);
    shift(&mut *c, 26, 50, 30, 5);
    shift(&mut *c, 38, 53, 18, 8);
    spin(&mut *c, 15, 27, 39, 40, 41, 29, 17, 16);
}

fn turn_lp (c: &mut [u8; 54]){
    shift(&mut *c, 12, 0, 44, 45);
    shift(&mut *c, 24, 3, 32, 48);
    shift(&mut *c, 36, 6, 20, 51);
    spin(&mut *c, 9, 21, 33, 34, 35, 23, 11, 10);
}

fn turn_up (c: &mut [u8; 54]){
    shift(&mut *c, 14, 17, 20, 11);
    shift(&mut *c, 13, 16, 19, 10);
    shift(&mut *c, 12, 15, 18, 9);
    spin(&mut *c, 0, 3, 6, 7, 8, 5, 2, 1);
}

fn turn_dp (c: &mut [u8; 54]){
    shift(&mut *c, 36, 33, 42, 39);
    shift(&mut *c, 37, 34, 43, 40);
    shift(&mut *c, 38, 35, 44, 41);
    spin(&mut *c, 45, 48, 51, 52, 53, 50, 47, 46);
}

fn turn_fp (c: &mut [u8; 54]){
    shift(&mut *c, 6, 35, 47, 15);
    shift(&mut *c, 7, 23, 46, 27);
    shift(&mut *c, 8, 11, 45, 39);
    spin(&mut *c, 12, 24, 36, 37, 38, 26, 14, 13);
}

fn turn_bp (c: &mut [u8; 54]){
    shift(&mut *c, 2, 41, 51, 9);
    shift(&mut *c, 1, 29, 52, 21);
    shift(&mut *c, 0, 17, 53, 33);
    spin(&mut *c, 18, 30, 42, 43, 44, 32, 20, 19);
}

