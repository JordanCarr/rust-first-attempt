fn main() {
    let verbose = true;

    // Fibonacci sequence
    let fib_num = 64usize;
    println!("Print first {} Fibonacci numbers", fib_num);
    fib_runner(fib_num, verbose);
}

fn fib_runner(number: usize, verbose: bool) {
    let mut computed = Vec::<usize>::new();
    for n in 0..=number {
        let result = fib(n, &mut computed);
        computed.push(result);
    }

    if verbose == true {
        for n in 0..computed.len() {
            println!("{}: {}", n, computed[n]);
        }
    } else if let Some(last_value) = computed.last() {
        println!("{}: {}", computed.len() - 1, last_value);
    }
}

fn fib(number: usize, computed: &mut Vec<usize>) -> usize {
    if computed.get(number).is_none() {
        return match &number {
            0 => 0,
            1 => 1,
            _ => fib(number - 1, computed) + fib(number - 2, computed),
        };
    }
    return computed[number];
}
