use std::convert::TryInto;
use std::io;
use std::thread;

pub mod kessels {
    pub fn work1<T: From<i8> + std::ops::AddAssign<T>>(_i: T) -> T {
        let mut _sum = T::from(0);
        for iter in 1..11 {
            _sum += T::from(iter);
        }
        _sum
    }
}

fn main() {
    println!("Please give me an int for the number of threads");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nThreads: usize = input.trim().parse().unwrap();

    println!("nThreads is {}", nThreads);

    if nThreads < 1 {
        panic!("nThreads must be greater than 1");
    }
    // Initialize variables
    let mut sum: i64 = 0;
    let n: usize = nThreads;
    let v: usize = n - 1;
    let once: usize = v;
    let mut edge = vec![0 as usize; once]; // Should be a 1 x v size vec
    let mut competing = vec![vec![0 as i64; once]; 2]; // should be a 2 x v size array
    let mut turn = vec![vec![0 as i64; once]; 2]; // should be a 2 x v size array :

    let mut idThread = 0;
    let mut node: usize = 0;
    let mut id = 0;
    let mut local = 0;

    // Spawn nThreads
    for i in 0..nThreads {
        // Spin up another thread
        thread::spawn(move || {
            // Begin downpass through the tournament tree
            // Initialize variables local to each threads
            idThread = i;
            node = idThread + n;
            id = node % 2;
            local = 0;

            // Begin local tournaments in each node
            // When thread reaches node == 1, may enter critical section
            while node > 1 {
                // Identify if thread came from left parent node (1) or right parent node (0)
                id = node % 2;
                // Get node number from id
                node = node / 2;

                // Anounce you are competing
                unsafe {competing[node][id] = 1;}

                // Local variable of if another thread is in same node
                let temp: i64 = turn[node][1 - id] as i64;
                let local: i64 = temp % 2;

                unsafe {turn[node][id] = local;}
                // Cond1 : You are the only thread, and no others are competing in same node
                // Cond2 : The other thread is competing, but is outside the critical section
                // while not true, wait until you can enter the critical section
                while !((competing[node][1 - id] == 0) || (local as usize != (turn[node][1 - id] as usize + id as usize) % 2))
                {
                }

                edge[node] = id;
            }
            // Critical section
            // update global sum
            sum += kessels::work1(1);
            // update node so as to stay out of next iteration of big while loop
            node = 1;

            // Begin UP pass through the tournament tree
            // set flags of other nodes you competed with to false (0) so they may compete as well
            // propagate values through the arithmetic to climb up the tree
            while node < n {
                competing[node as usize][edge[node as usize] as usize] = 0;
                node = 2 * node + edge[node];
            }
        });
    }

    #[test]
    fn it_works() {
        assert_eq!(nThreads * 55, sum)
    }
    println!("Sanity Check");
    println!("55*{} == {}", nThreads, sum);
    println!("{}", 55 * nThreads == sum.try_into().unwrap());
}
