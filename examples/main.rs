extern crate rand;
extern crate timely;
extern crate timely_sort;
extern crate differential_dataflow;
// extern crate vec_map;

use timely::dataflow::*;
use timely::dataflow::operators::*;

use rand::{Rng, SeedableRng, StdRng};

use differential_dataflow::input::Input;
use differential_dataflow::trace::Trace;
use differential_dataflow::{Collection, AsCollection};
use differential_dataflow::operators::*;
// use differential_dataflow::operators::join::JoinArranged;
use differential_dataflow::operators::group::GroupArranged;
use differential_dataflow::operators::count::CountTotal;
use differential_dataflow::operators::arrange::{ArrangeByKey, ArrangeBySelf};
use differential_dataflow::lattice::Lattice;
use differential_dataflow::trace::implementations::hash::HashValSpine as Spine;

fn main() {
    let nodes: u32 = std::env::args().nth(1).unwrap().parse().unwrap();
    let edges: usize = std::env::args().nth(2).unwrap().parse().unwrap();

    // create a a degree counting differential dataflow
    let (mut input, probe) = worker.dataflow(|scope| {

        // create edge input, count a few ways.
        let (input, edges) = scope.new_collection();

        //pull off source and count them
        let degrs = edges.map(|(src, _dst)| src).count();

        // pull of count, and count.
        let distr = degrs.map(|(_src, cnt)| cnt as usize).count();

        // show us something about the collection, notice when done.
        let probe = distr.inspect(|x| println!("observed: {:?}", x)).probe();

        (input, probe)
    });
}
