use std::*;

static mut T:[usize;15] = [0;15];
static mut P:[usize;15] = [0;15];
static mut dp:[usize;15] = [0;15];
static mut max_cnt:usize = 0;

fn main() {
    unsafe {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();
        let N:usize = input.trim().parse().unwrap();
        
        for i in 0..N {
            
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            
            let inputs = input.trim().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            
            let input_t = inputs[0];
            let input_p = inputs[1];
    
            T[i] = input_t;
            P[i] = input_p;
        }

        dynamic(N);
        
    }
}


fn dynamic(N: usize) {
    
    unsafe {
        
        for i in 0..N+1 {
        
            dp[i] = std::cmp::max(max_cnt, dp[i]);
            
            if T[i] + i  <= N {
                dp[T[i] + i] = std::cmp::max(dp[T[i] + i], dp[i] + P[i]);
            }
            
            max_cnt = std::cmp::max(max_cnt, dp[i]);
        }

        print!("{}", max_cnt);
    }
}
