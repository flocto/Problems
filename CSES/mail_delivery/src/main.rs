use std::io;
use std::collections::BTreeSet;
use std::fs;

/*
    Input
    The first input line has two integers n and m: the number of crossings and streets. The crossings are numbered 1,2,…,n, and the post office is located at crossing 1.
    After that, there are m lines describing the streets. Each line has two integers a and b: there is a street between crossings a and b. All streets are two-way streets.
    Every street is between two different crossings, and there is at most one street between two crossings.

    Output
    Print all the crossings on the route in the order you will visit them. You can print any valid solution.
    If there are no solutions, print "IMPOSSIBLE".
*/
fn main() {
    //let data = fs::read_to_string("mail.in").unwrap();
    //let mut lines = data.lines();
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");
    let line: Vec<&str> = line.trim().split_whitespace().collect();
    let n = line[0].parse::<usize>().unwrap();
    let m: usize = line[1].parse().expect("Please type a number");
    let mut graph: Vec<BTreeSet<usize>> = Vec::with_capacity(n+1);
    for _ in 0..n+1 {
        graph.push(BTreeSet::new());
    }
    for _ in 0..m {
        //let line = lines.next().unwrap().trim().to_string();
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        let line: Vec<&str> = line.trim().split_whitespace().collect();
        let a: usize = line[0].parse().expect("Please type a number");
        let b: usize = line[1].parse().expect("Please type a number");
        graph[a].insert(b);
        graph[b].insert(a);
    }


    //println!("verif start");
    if !verif(n, &graph) {
        println!("IMPOSSIBLE");
        return;
    }
    //println!("verif done");
    let ans = eulerian_cycle(&graph).iter().rev().map(|&x| x).collect::<Vec<usize>>();
    
    if ans.len() != m + 1 || ans[0] != 1 || ans[m] != 1{
        println!("IMPOSSIBLE");
    } else {
        //let mut ans_str = String::new();
        for i in ans {
            //ans_str.push_str(&i.to_string());
            //ans_str.push_str(" ");
            print!("{} ", i);
        }
        //fs::write("mail.out", ans_str).expect("Unable to write file");
    }
}
// find a eulerian cycle that starts and ends at node 1, using every edge in the graph
fn eulerian_cycle(s: &Vec<BTreeSet<usize>>) -> Vec<usize> {
    let mut graph = s.clone();
    let mut stack: Vec<usize> = Vec::new();
    let mut ans: Vec<usize> = Vec::new();
    stack.push(1);
    while !stack.is_empty() {
        let node = stack.pop().unwrap();
        let neighbors = graph[node].clone();
        if neighbors.len() == 0 {
            ans.push(node);
        } else {
            let neighbor = neighbors.iter().next().unwrap().clone();
            graph[node].remove(&neighbor);
            graph[neighbor].remove(&node);
            stack.push(node);
            stack.push(neighbor);
        }
    }
    ans
}

// check if the graph is eulerian cycle
fn verif(n: usize, graph: &Vec<BTreeSet<usize>>) -> bool {
    let mut degree: Vec<usize> = Vec::with_capacity(n+1);
    for _ in 0..n+1 {
        degree.push(0);
    }

    for i in 1..n+1 {
        let neighbors = graph[i].clone();
        degree[i] = neighbors.len();
        if neighbors.len() % 2 != 0 {
            return false;
        }
    }

    let visited: Vec<bool> = dfs(n, 1, &graph);
    //println!("{:?}", visited);
    //println!("{:?}", degree);
    for i in 1..n+1 {
        if !visited[i] && degree[i] != 0 {
            return false;
        }
    }
    true
}

fn dfs(n: usize, cur: usize, streets: &Vec<BTreeSet<usize>>) -> Vec<bool>{
    let mut visited: Vec<bool> = vec![false; n+1];
    let mut stack: Vec<usize> = Vec::new();
    stack.push(cur);
    while !stack.is_empty() {
        let node = stack.pop().unwrap();
        visited[node] = true;
        let neighbors = streets[node].clone();
        for neighbor in neighbors {
            if !visited[neighbor] {
                stack.push(neighbor);
            }
        }
    }
    visited
}