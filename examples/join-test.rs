
extern crate timely;
extern crate differential_dataflow;

use differential_dataflow::input::Input;
use differential_dataflow::operators::Join;

fn main() {
    ::timely::example(|scope| {

        let x = scope.new_collection_from(vec![(0, 1), (0, 5)]).1;
        let y = scope.new_collection_from(vec![(0, 3), (0, 6)]).1;


        let a = x.join(&y).inspect(|x| println!("observed data: {:?}", x));

    });
}
