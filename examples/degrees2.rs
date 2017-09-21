extern crate rand;
extern crate timely;
extern crate timely_sort;
extern crate differential_dataflow;
// extern crate vec_map;

use timely::dataflow::*;
use timely::dataflow::operators::*;

use rand::{Rng, SeedableRng, StdRng};
use std::io;

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
    let round: usize = std::env::args().nth(3).unwrap().parse().unwrap();

    //let kc1 = std::env::args().find(|x| x == "kcore1").is_some();
    //let kc2 = std::env::args().find(|x| x == "kcore2").is_some();

    // define a new computational scope, in which to run BFS
    timely::execute_from_args(std::env::args().skip(4), move |worker| {

        let index = worker.index();
        let peers = worker.peers();

        // create a a degree counting differential dataflow
        let (mut input, probe) = worker.dataflow(|scope| {

            // create edge input, count a few ways.
            let (input, edges) = scope.new_collection();

            // if kc1 { edges = kcore1(&edges, std::env::args().nth(4).unwrap().parse().unwrap()); }
            // if kc2 { edges = kcore2(&edges, std::env::args().nth(4).unwrap().parse().unwrap()); }

            let degrs = edges.map(|(src, _dst)| src)
                             .count();

            // pull of count, and count.
//            let distr = degrs.map(|(_src, cnt)| cnt as usize)
//                             .count();

            // show us something about the collection, notice when done.
            let probe = degrs.inspect(|x| println!("observed: {:?}", x))
                             .probe();

            (input, probe)
        });

        let timer = ::std::time::Instant::now();

        let seed: &[_] = &[1, 2, 3, index];
        let mut rng1: StdRng = SeedableRng::from_seed(seed); // rng for edge additions
        let mut rng2: StdRng = SeedableRng::from_seed(seed); // rng for edge additions

        // load up graph dataz
        for edge in 0..edges {
            //            println!("peer:{}, index:{}", peers, index);
            if edge % peers == index {
                let a = rng1.gen_range(0, nodes);
                let b = rng1.gen_range(0, nodes);
                input.insert((a, b));
                //                println!("insert number: a:{}, b:{}", a, b);
            }
            //            println!("Hi");
        }

        input.advance_to(1);
        input.flush();
        worker.step_while(|| probe.less_than(input.time()));

        if index == 0 {
            // let timer = timer.elapsed();
            // let nanos = timer.as_secs() * 1000000000 + timer.subsec_nanos() as u64;
            println!("Loading finished after {:?}", timer.elapsed());
        }

        if round > 0 {

            for round2 in 1..round {

                // only do the work of this worker.
                if round % peers == index {
                    input.advance_to(round2);
                    println!("plesae input add number:");
                    let mut input1 = String::new();
                    io::stdin()
                        .read_line(&mut input1)
                        .expect("Failed to read line");
                    let a = input1.trim().parse::<u32>().unwrap();
                    //                    println!("plesae input add node-b:");
                    //                    let mut input2 = String::new();
                    //                    io::stdin()
                    //                        .read_line(&mut input2)
                    //                        .expect("Failed to read line");
                    //                    let b = input2.trim().parse::<u32>().unwrap();
                    input.insert((a, 0));

                    println!("plesae input remove number:");
                    let mut input3 = String::new();
                    io::stdin()
                        .read_line(&mut input3)
                        .expect("Failed to read line");
                    let c = input3.trim().parse::<u32>().unwrap();
                    //                    println!("plesae input remove node-d:");
                    //                    let mut input4 = String::new();
                    //                    io::stdin()
                    //                        .read_line(&mut input4)
                    //                        .expect("Failed to read line");
                    //                    let d = input4.trim().parse::<u32>().unwrap();
                    input.remove((c, 0));
                    //                    println!("add node: a:{}, b:{}", a, b);
                    //                    println!("remove node: c:{}, d:{}", c, d);
                }

                if round % 1 == 0 {
                    // all workers indicate they have finished with `round`.
                    input.advance_to(round2 + 1);
                    input.flush();

                    let timer = ::std::time::Instant::now();
                    worker.step_while(|| probe.less_than(input.time()));
                    println!("worker {}, round {} finished after {:?}",
                             index,
                             round2,
                             timer.elapsed());
                }
            }
        }
    })
            .unwrap();
}

// fn kcore1<G: Scope>(edges: &Collection<G, (u32, u32)>, k: isize) -> Collection<G, (u32, u32)>
// where G::Timestamp: Lattice+Ord {

//     edges.iterate(|inner| {
//         // determine active vertices
//         let active = inner.flat_map(|(src,dst)| Some((src,())).into_iter().chain(Some((dst,())).into_iter()))
//                           .group(move |_k, s, t| { if s[0].1 > k { t.push(((),1)) } })
//                           .map(|(k,_)| k);
//                           // .threshold_u(move |_,cnt| if cnt >= k { 1 } else { 0 });

//         // restrict edges active vertices, return result
//         edges.enter(&inner.scope())
//              .semijoin(&active)
//              .map(|(src,dst)| (dst,src))
//              .semijoin(&active)
//              .map(|(dst,src)| (src,dst))
//     })
// }

// fn kcore2<G: Scope>(edges: &Collection<G, (u32, u32)>, k: isize) -> Collection<G, (u32, u32)>
// where G::Timestamp: Lattice+::std::hash::Hash+Ord {

//     edges.iterate(move |inner| {
//         // determine active vertices
//         let active = inner.flat_map(|(src,dst)| Some(src).into_iter().chain(Some(dst).into_iter()))
//                           .arrange_by_self()
//                           .group_arranged(move |_k, s, t| { if s[0].1 > k { t.push(((),1)) } }, Spine::new());

//         // restrict edges active vertices, return result
//         edges.enter(&inner.scope())
//              .arrange_by_key_hashed()
//              .join_core(&active, |k,v,_| Some((k.item.clone(), v.clone())))
//              .map(|(src,dst)| (dst,src))
//              .arrange_by_key_hashed()
//              .join_core(&active, |k,v,_| Some((k.item.clone(), v.clone())))
//              .map(|(dst,src)| (src,dst))
//     })
// }
