use rand::{self, Rng};
use std::{fmt, io};


pub struct AbandonedCrate {
    num1 : u8,
    num2 : u8,
    num3 : u8,
    num4 : u8,
    remaining_attempts : u8,
}

impl fmt::Display for AbandonedCrate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {}", self.num1, self.num2, self.num3, self.num4)
    }
}

impl AbandonedCrate {
    fn new(num1 : u8, num2 : u8, num3 : u8, num4 : u8) -> AbandonedCrate {
        return AbandonedCrate { num1, num2, num3, num4, remaining_attempts : 10 };
    }
    fn generate() -> AbandonedCrate {
        let mut rng = rand::thread_rng();
        let num1 : u8 = rng.gen_range(0..10);
        let num2 : u8 = rng.gen_range(0..10);
        let num3 : u8 = rng.gen_range(0..10);
        let num4 : u8 = rng.gen_range(0..10);
        return AbandonedCrate::new(num1, num2, num3, num4);
    }
}

pub struct SolveAttempt {
    num1 : u8,
    num2 : u8,
    num3 : u8,
    num4 : u8,
    cor_num_inc_pos : u8,
    cor_num_cor_pos : u8,
}

impl fmt::Display for SolveAttempt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {}\ncorrect num incorrect pos : {}\ncorrect num correct pos : {}\n", self.num1, self.num2, self.num3, self.num4, self.cor_num_inc_pos, self.cor_num_inc_pos)
    }
}

impl SolveAttempt {
    fn new(num1 : u8, num2 : u8, num3 : u8, num4 : u8, cor_num_inc_pos : u8, cor_num_cor_pos : u8) -> SolveAttempt {
        return SolveAttempt { num1, num2, num3, num4, cor_num_inc_pos, cor_num_cor_pos };
    }
    fn from_string(string : String) -> Result<SolveAttempt, ()> {
        let keywords : Vec<&str> = string.split(" ").collect();
        if keywords.len() != 6 {return Err(());}
        let num1 : u8 = keywords[0].parse().unwrap();
        let num2 : u8 = keywords[1].parse().unwrap();
        let num3 : u8 = keywords[2].parse().unwrap();
        let num4 : u8 = keywords[3].parse().unwrap();
        let cor_num_inc_pos : u8 = keywords[4].parse().unwrap();
        let cor_num_cor_pos : u8 = keywords[5].parse().unwrap();
        return Ok(SolveAttempt::new(num1, num2, num3, num4, cor_num_inc_pos, cor_num_cor_pos))
    }
}

fn solve() {
    let mut probability_vec : Vec<Vec<f64>> = vec![vec![0.0; 10]; 4];

}

#[cfg(test)]
mod test {
    use crate::{AbandonedCrate, SolveAttempt};
    use rand::Rng;
    #[test]
    fn random_auto_solve() {
        let mut cnip_probability_vec : Vec<Vec<f64>> = vec![vec![0.0; 10]; 4];
        let mut cncp_probability_vec : Vec<Vec<f64>> = vec![vec![0.0; 10]; 4];
        let mut random_abandoned_crate : AbandonedCrate = AbandonedCrate::generate();
        println!("{}", random_abandoned_crate);
        while random_abandoned_crate.remaining_attempts > 0 {
            let mut rng = rand::thread_rng();
            let num1 : u8 = rng.gen_range(0..10);
            let num2 : u8 = rng.gen_range(0..10);
            let num3 : u8 = rng.gen_range(0..10);
            let num4 : u8 = rng.gen_range(0..10);
            let mut cor_num_inc_pos : u8 = 0;
            let mut cor_num_cor_pos : u8 = 0;

            if num1 == random_abandoned_crate.num1 {cor_num_cor_pos += 1}
            else if num1 == random_abandoned_crate.num2 {cor_num_inc_pos += 1}
            else if num1 == random_abandoned_crate.num3 {cor_num_inc_pos += 1}
            else if num1 == random_abandoned_crate.num4 {cor_num_inc_pos += 1};

            if num2 == random_abandoned_crate.num2 {cor_num_cor_pos += 1}
            else if num2 == random_abandoned_crate.num1 {cor_num_inc_pos += 1}
            else if num2 == random_abandoned_crate.num3 {cor_num_inc_pos += 1}
            else if num2 == random_abandoned_crate.num4 {cor_num_inc_pos += 1};

            if num3 == random_abandoned_crate.num3 {cor_num_cor_pos += 1}
            else if num3 == random_abandoned_crate.num2 {cor_num_inc_pos += 1}
            else if num3 == random_abandoned_crate.num1 {cor_num_inc_pos += 1}
            else if num3 == random_abandoned_crate.num4 {cor_num_inc_pos += 1};

            if num4 == random_abandoned_crate.num4 {cor_num_cor_pos += 1}
            else if num4 == random_abandoned_crate.num2 {cor_num_inc_pos += 1}
            else if num4 == random_abandoned_crate.num3 {cor_num_inc_pos += 1}
            else if num4 == random_abandoned_crate.num1 {cor_num_inc_pos += 1};

            let solve_attempt : SolveAttempt = SolveAttempt::new(num1, num2, num3, num4, cor_num_inc_pos, cor_num_cor_pos);
            println!("{}",  solve_attempt);

            let _temp_cnip : f64 = f64::from(solve_attempt.cor_num_inc_pos);
            let _temp_cncp : f64 = f64::from(solve_attempt.cor_num_cor_pos);

            cnip_probability_vec[0][usize::from(num1)] += (_temp_cnip)/4.0;
            cnip_probability_vec[1][usize::from(num2)] += (_temp_cnip)/4.0;
            cnip_probability_vec[2][usize::from(num3)] += (_temp_cnip)/4.0;
            cnip_probability_vec[3][usize::from(num4)] += (_temp_cnip)/4.0;

            cncp_probability_vec[0][usize::from(num1)] += (_temp_cncp)/4.0;
            cncp_probability_vec[1][usize::from(num2)] += (_temp_cncp)/4.0;
            cncp_probability_vec[2][usize::from(num3)] += (_temp_cncp)/4.0;
            cncp_probability_vec[3][usize::from(num4)] += (_temp_cncp)/4.0;




            random_abandoned_crate.remaining_attempts -= 1;
        }
    }
}

fn main() {
    solve();
}
