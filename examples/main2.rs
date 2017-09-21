
extern crate csv;
extern crate timely;
extern crate differential_dataflow;

//use timely::dataflow::*;
//use differential_dataflow::*;
use differential_dataflow::input::Input;
use differential_dataflow::operators::Join;

use csv::ReaderBuilder;
use csv::Writer;


fn main() {

    //define the dataflow
    timely::execute_from_args(std::env::args().skip(2), move |worker| {

        let (mut input1, probe) = worker.dataflow::<_, _, _>(|scope| {
            let (input1, data1) = scope.new_collection();

            let probe = data1.inspect(|x| println!("{:?}", x)).probe();

            (input1, probe)
        });


        //insert genemap
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
          //  .delimiter(b'\t')
            .from_path("C:/Users/apple/Desktop/db/genemap2-170101-03_d-plus.csv")
            .unwrap();

        for result in rdr.records() {
            let mut vec: Vec<String> = Vec::new();
            let record = result.expect("a CSV record");
            for field in record.iter() {
                vec.push(String::from(field));
            }
            input1.insert(vec);
        }




    })
            .unwrap();
}
