use rand::{self, Rng};
use std::{fmt, io, vec};
mod simulation;


pub struct AbandonedCrate {
    num1 : usize,
    num2 : usize,
    num3 : usize,
    num4 : usize,
    remaining_attempts : usize,
}

impl fmt::Display for AbandonedCrate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {}", self.num1, self.num2, self.num3, self.num4)
    }
}

impl AbandonedCrate {
    fn new(num1 : usize, num2 : usize, num3 : usize, num4 : usize) -> AbandonedCrate {
        return AbandonedCrate { num1, num2, num3, num4, remaining_attempts : 10 };
    }
    fn generate() -> AbandonedCrate {
        let mut rng = rand::thread_rng();
        let num1 : usize = rng.gen_range(0..10);
        let num2 : usize = rng.gen_range(0..10);
        let num3 : usize = rng.gen_range(0..10);
        let num4 : usize = rng.gen_range(0..10);
        return AbandonedCrate::new(num1, num2, num3, num4);
    }
}

pub struct SolveAttempt {
    num1 : usize,
    num2 : usize,
    num3 : usize,
    num4 : usize,
    cor_num_inc_pos : usize,
    cor_num_cor_pos : usize,
}

impl fmt::Display for SolveAttempt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {}\ncorrect num incorrect pos : {}\ncorrect num correct pos : {}\n", self.num1, self.num2, self.num3, self.num4, self.cor_num_inc_pos, self.cor_num_inc_pos)
    }
}

impl SolveAttempt {
    fn new(num1 : usize, num2 : usize, num3 : usize, num4 : usize, cor_num_inc_pos : usize, cor_num_cor_pos : usize) -> SolveAttempt {
        return SolveAttempt { num1, num2, num3, num4, cor_num_inc_pos, cor_num_cor_pos };
    }
    fn from_string(string : String) -> Result<SolveAttempt, ()> {
        let keywords : Vec<&str> = string.split(" ").collect();
        if keywords.len() != 6 {return Err(());}
        let num1 : usize = keywords[0].parse().unwrap();
        let num2 : usize = keywords[1].parse().unwrap();
        let num3 : usize = keywords[2].parse().unwrap();
        let num4 : usize = keywords[3].parse().unwrap();
        let cor_num_inc_pos : usize = keywords[4].parse().unwrap();
        let cor_num_cor_pos : usize = keywords[5].parse().unwrap();
        return Ok(SolveAttempt::new(num1, num2, num3, num4, cor_num_inc_pos, cor_num_cor_pos))
    }
}


fn main() {
    simulation::simulate();
}






































// depracated 
/*
#[cfg(test)]
mod test {
    use crate::{AbandonedCrate, SolveAttempt, find_max_index};
    use rand::Rng;
    #[test]
    fn random_auto_solve() {
        let mut cnip_probability_vec : Vec<Vec<f64>> = vec![vec![0.0; 10]; 4];
        let mut cncp_probability_vec : Vec<Vec<f64>> = vec![vec![0.0; 10]; 4];
        let mut random_abandoned_crate : AbandonedCrate = AbandonedCrate::generate();
        println!("{}", random_abandoned_crate);
        while random_abandoned_crate.remaining_attempts > 0 {
            let mut rng = rand::thread_rng();
            let num1 : usize = rng.gen_range(0..10);
            let num2 : usize = rng.gen_range(0..10);
            let num3 : usize = rng.gen_range(0..10);
            let num4 : usize = rng.gen_range(0..10);
            let mut cor_num_inc_pos : usize = 0;
            let mut cor_num_cor_pos : usize = 0;

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

        println!("{} {} {} {}",
        find_max_index(&cnip_probability_vec[0]),
        find_max_index(&cnip_probability_vec[1]),
        find_max_index(&cnip_probability_vec[2]),
        find_max_index(&cnip_probability_vec[3])
        );


        println!("{} {} {} {}",
        find_max_index(&cncp_probability_vec[0]),
        find_max_index(&cncp_probability_vec[1]),
        find_max_index(&cncp_probability_vec[2]),
        find_max_index(&cncp_probability_vec[3])
        );
    }
}

fn find_max_index<T>(vector : &Vec<T>) -> usize
where T : PartialOrd
{
    let length : usize = vector.len();
    let mut max_index : usize = 0;

    for index in 1..length {
        if vector[index] > vector[max_index] {
            max_index = index;
        }
    }

    return max_index;
} 
*/