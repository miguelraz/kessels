use std::io;
use std::sync::Arc;
use std::thread;

fn work(_n: i64) -> i64 {
    (1..=10).sum()
}

fn kessels() -> i64 {
    println!("Please give me an int for the number of threads");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nThreads: usize = input.trim().parse().unwrap();

    println!("nThreads is {}", nThreads);

    if nThreads < 1 {
        panic!("nThreads must be greater than 1");
    }
    // Initialize variables
    let mut totalsum: i64 = 0;
    let n: usize = nThreads;
    let mut threads = Vec::new();
    let v: usize = n - 1;

    let mut edge = vec![0; v]; // Should be a 1 x v size vec
    let mut competing = vec![vec![0; v]; 2]; // should be a 2 x v size array
    let mut turn = vec![vec![0; v]; 2]; // should be a 2 x v size array :

    // Spawn nThreads
    for i in 0..nThreads {
        threads.push(thread::spawn(|| {
            // Begin downpass through the tournament tree
            // Initialize variables local to each threads
            let idThread = i;
            let mut node = idThread + n;

            // Begin local tournaments in each node
            // When thread reaches node == 1, may enter critical section
            while node > 1 {
                // Identify if thread came from left parent node (1) or right parent node (0)
                // Get node number from id
                let id = node % 2;
                node = node / 2;

                // Announce you are competing
                competing[node][id] = 1;

                // Local variable of if another thread is in same node
                let local = (turn[node][1 - id] + id) % 2;

                turn[node][id] = local;
                // Cond1 : You are the only thread, and no others are competing in same node
                // Cond2 : The other thread is competing, but is outside the critical section
                // while not true, wait until you can enter the critical section
                while !((competing[node][1 - id] == 0) || (local != (turn[node][1 - id] + id) % 2))
                {
                }

                edge[node] = id;
            }
            // Critical section
            // update global sum
            totalsum += work(4);
            // update node so as to stay out of next iteration of big while loop
            node = 1;

            // Begin UP pass through the tournament tree
            // set flags of other nodes you competed with to false (0) so they may compete as well
            // propagate values through the arithmetic to climb up the tree
            while node < n {
                competing[node][edge[node]] = 0;
                node = 2 * node + edge[node];
            }
        }));
    }
    for handle in threads {
        handle.join().expect("oops");
    }
    totalsum
}

#[test]
fn it_works() {
    let totalsum = kessels();
    let nThreads = 4;
    assert_eq!(nThreads * 55, totalsum);
    println!("Sanity Check");
    println!("55*{} == {}", nThreads, totalsum);
    println!("{}", 55 * nThreads == totalsum);
}
fn main() {
    let x = kessels();
    println!("{}", x);
}
