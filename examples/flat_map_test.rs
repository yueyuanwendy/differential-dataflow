
extern crate timely;
extern crate differential_dataflow;

use differential_dataflow::input::Input;
use differential_dataflow::operators::Iterate;
use differential_dataflow::operators::Consolidate;
use timely::dataflow::operators::Map;

fn main() {
    ::timely::example(|scope| {

                          scope
                              .new_collection_from(1..5u32)
                              .1
                              .flat_map(|x| (0..x))
                              .inspect(|x| println!("{:?}", x));
                      });
}
