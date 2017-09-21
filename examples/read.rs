//#![feature(core_intrinsics)]

extern crate csv;
extern crate timely;
extern crate differential_dataflow;
// extern crate vec_map;

use std::io;

//use timely::dataflow::*;
//use differential_dataflow::*;
use differential_dataflow::input::Input;

//use timely::dataflow::operators::Filter;
use std::io::prelude::*;
use csv::ReaderBuilder;
use std::fs::File;
use std::path::Path;

use std::io::Write;



fn main() {

    timely::execute_from_args(std::env::args().skip(2), move |worker| {


        let (mut input, probe) = worker.dataflow::<_, _, _>(|scope| {
            let (input, data) = scope.new_collection();

            //            let a = data.filter(|x: &Vec<String>| {
            //                                    x[7].to_lowercase().contains("alzheimer") ||
            //                                    x[12].to_lowercase().contains("alzheimer")
            //                                });
            let probe = data.inspect(|x| println!("{:?}", (x.0)[0])).probe();


            (input, probe)
        });

        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .delimiter(b'\t')
            .from_path("C:/Users/apple/Desktop/genemap-print/genemap2-170101-esc.txt")
            .unwrap();

        for result in rdr.records() {
            let mut vec: Vec<String> = Vec::new();
            let record = result.expect("a CSV record");
            for field in record.iter() {
                vec.push(String::from(field));
            }
            input.insert(vec);
        }

        input.advance_to(1);
        input.flush();
        worker.step_while(|| probe.less_than(input.time()));
        //       wrt.flush();
    })
            .unwrap();
}
