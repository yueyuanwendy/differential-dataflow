extern crate timely;
extern crate differential_dataflow;
extern crate rand;

use rand::{Rng};


use differential_dataflow::input::Input;
use differential_dataflow::operators::*;



fn main() {
//    ::timely::example(|scope| {
//        // report the number of occurrences of each key
//        scope.new_collection_from(1 .. 20).1
//             .map(|x| x / 3)
//             .count_total().inspect(|x| println!("{:?}", x));
//    });
    ::timely::execute_from_args(std::env::args(), move |worker| {
        let (mut input, probe) = worker.dataflow::<u64, _, _>(|scope| {
            let (input, data) = scope.new_collection();
            
            let probe = data.inspect(|x| println!("observed data: {:?}", x))
                .count_total()
                .inspect(|x| println!("observed counts: {:?}", x))
                .probe();

            (input, probe)
        });

        let mut rng = rand::thread_rng();

        for _ in 0 .. 3 {
            let time = *input.epoch();
            for round in time .. time + 10 {
                let val = rng.gen_range(0, 20);
                println!("round = {:?}, value = {:?}", round, val);
                input.insert(val);
                input.advance_to(round);
            }
            
            input.flush();
            while probe.less_than(input.time()) {
                worker.step();
            }
        }
    }).unwrap();
}
