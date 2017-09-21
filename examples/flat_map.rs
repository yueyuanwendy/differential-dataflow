extern crate timely;

use timely::dataflow::operators::{ToStream, Map, Inspect};

fn main() {
    timely::example(|scope| {
                        (0..5)
                            .to_stream(scope)
                            .flat_map(|x| (0..x))
                            .inspect(|x| println!("seen: {:?}", x));
                    });
}
