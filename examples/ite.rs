extern crate timely;
extern crate differential_dataflow;

use differential_dataflow::input::Input;
use differential_dataflow::operators::Iterate;
use differential_dataflow::operators::Consolidate;

fn main() {
    ::timely::example(|scope| {

        scope
            .new_collection_from(1..10u32)
            .1
            //.inspect(|x| println!("{:?}", x))
            .iterate(|values| {
                         values
                             .map(|x| if x % 2 == 0 { x / 2 } else { x })
                             //.consolidate()
                             //.inspect(|x| println!("{:?}", x))
                     })
            .inspect(|x| println!("{:?}", x));
    });
}
