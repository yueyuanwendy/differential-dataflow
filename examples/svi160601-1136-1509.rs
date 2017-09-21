
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
    timely::execute_from_args(std::env::args().skip(2), move |worker| {
        let mut gis_writer = Writer::from_path("C:/Users/apple/Desktop/output/genes-in-scope.csv").expect("Cannot create file");
       // let mut gis_writer2 = Writer::from_path("C:/Users/apple/Desktop/output/genes-in-scope-exploded.csv").expect("Cannot create file");
        let mut fpv_writer = Writer::from_path("C:/Users/apple/Desktop/output/nonsynonymous-pv.csv").expect("Cannot create file");
        //let mut fpv_writer2 = Writer::from_path("C:/Users/apple/Desktop/output/nonsynonymous-pv-exploded.csv").expect("Cannot create file");
        let mut vis_wrt = Writer::from_path("C:/Users/apple/Desktop/output/variants-in-scope.csv").expect("Cannot create file");
        let mut join_wrt = Writer::from_path("C:/Users/apple/Desktop/output/debug-join.csv").expect("Cannot create file");
        let mut class_wrt = Writer::from_path("C:/Users/apple/Desktop/output/svi-classfication.csv").expect("Cannot create file");

        let (mut input1, mut input2, mut input3, probe) = worker.dataflow::<_, _, _>(|scope| {
            let (input1, data1) = scope.new_collection();
            let (input2, data2) = scope.new_collection();
            let (input3, data3) = scope.new_collection();
            
            let gis = data1
                .filter(|x: &Vec<String>| {
                    x[7].to_lowercase().contains("cadasil") ||
                    x[12].to_lowercase().contains("cadasil") ||
                     x[7].to_lowercase().contains("cerebral arteriopathy") ||
                    x[12].to_lowercase().contains("cerebral arteriopathy") ||
                     x[7].to_lowercase().contains("subcortical infarcts") ||
                    x[12].to_lowercase().contains("subcortical infarcts") ||
                     x[7].to_lowercase().contains("leukoencephalopathy") ||
                    x[12].to_lowercase().contains("leukoencephalopathy") 
                })
          .inspect(move |x| gis_writer.write_record(&x.0).expect("Cannot write to file"))
                .flat_map(|row| {
                    let mut out = Vec::new();
                    let genes = row[6].split(',').map(|x| x.trim()).collect::<Vec<_>>();
                    for gene in genes {
                        out.push((gene.to_string(), row.to_vec()));
                    }
                    out.into_iter()
                });
             //  .inspect(move |x| gis_writer2.write_record(&(x.0).1).expect("Cannot write to file"));
                

            let pv = data2
                .filter(|x: &Vec<String>| {
                    x[7].to_lowercase() != "synonymous snv" 
                })
        .inspect(move |x| fpv_writer.write_record(&x.0).expect("Cannot write to file"))
                .flat_map(|row| {
                    let mut out = Vec::new();
                    let genes = row[6].split(',').map(|x| x.trim()).collect::<Vec<_>>();
                    for gene in genes {
                        out.push((gene.to_string(), row.to_vec()));
                    }
                    out.into_iter()
                });
             //  .inspect(move |x| fpv_writer2.write_record(&(x.0).1).expect("Cannot write to file"));
                
                

            let join = gis.join(&pv);
            let join2=join.map(|x|
        
        {
               let mut join_out = Vec::new();
               for i in (x.1).to_vec() {
               	  join_out.push(i.clone());
               }
              for i in (x.2).to_vec() {
              	  join_out.push(i.clone());
             }
              join_out
        }
            	
            )
      .inspect(move |x| vis_wrt.write_record(&(x.0)).expect("Cannot write to file"));
          
           
        let variant=join2.map(|x| (format!("{}_{}",x[14].to_owned(), x[15].to_owned() ) , x));
         let annotat= data3.map(|x:Vec<String>| (format!("chr{}_{}",x[13].to_owned(), x[14].to_owned()), x)); 
         
             let join3 = variant.join(&annotat);
        
         
         let join4 = join3.map(|x|
         
           
        {
               let mut join_out2 = Vec::new();
               for i in (x.2).to_vec() {
               	  join_out2.push(i.clone());
               }
              for i in (x.1).to_vec() {
              	  join_out2.push(i.clone());
             }
              join_out2
        }
         
         )
         .inspect(move |x| join_wrt.write_record(&(x.0)).expect("Cannot write to file"));
//          
      let classify = join3.map(|x|
      	{
      		let mut join_out3 = Vec::new();
      		
            if (x.2)[5].to_lowercase()==("pathogenic")
            {
            	join_out3.push(String::from("red"));
            	
              for i in (x.2).to_vec() {
               	  join_out3.push(i.clone());
               }
              for i in (x.1).to_vec() {
              	  join_out3.push(i.clone());
             }
            	
            }
            
            else if (x.2)[5].to_lowercase()==("benign")
             {
             	join_out3.push(String::from("green"));
            	
              for i in (x.2).to_vec() {
               	  join_out3.push(i.clone());
               }
              for i in (x.1).to_vec() {
              	  join_out3.push(i.clone());
             }
             }       
             
             else 
             {
             	join_out3.push(String::from("amber"));
            	
              for i in (x.2).to_vec() {
               	  join_out3.push(i.clone());
               }
              for i in (x.1).to_vec() {
              	  join_out3.push(i.clone());
             }
             }
             join_out3
       })
 .inspect(move |x| class_wrt.write_record(&(x.0)).expect("Cannot write to file"));   
          
           
            let probe = classify
             // .inspect(|x| println!("observed data: {:?}", x))
                //.map(|x| ??? -- concat x.0.1 with x.0.2)
                .probe();

            (input1, input2, input3, probe)
        });
        
         let timer = ::std::time::Instant::now();

        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .delimiter(b'\t')
            .from_path("C:/Users/apple/Desktop/db/genemap2-160601-esc.txt").unwrap();

        for result in rdr.records() {
            let mut vec: Vec<String> = Vec::new();
            let record = result.expect("a CSV record");
            for field in record.iter() {
                vec.push(String::from(field));
            }
            input1.insert(vec);
        }

        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
          
            .from_path("C:/Users/apple/Desktop/db/Freebayes_BatchCalling_d_1136.entire.csv").unwrap();

        for result in rdr.records() {
            let mut vec: Vec<String> = Vec::new();
            let record = result.expect("a CSV record");
            for field in record.iter() {
                vec.push(String::from(field));
            }
            input2.insert(vec);
        }
        
        //insert clin_var
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .delimiter(b'\t')
            .from_path("C:/Users/apple/Desktop/db/variant_summary-1509.txt").unwrap();

        for result in rdr.records() {
            let mut vec: Vec<String> = Vec::new();
            let record = result.expect("a CSV record");
            for field in record.iter() {
                vec.push(String::from(field));
            }
            input3.insert(vec);
        }


      
        input1.advance_to(1);
        input1.flush();
        input2.advance_to(1);
        input2.flush();
        input3.advance_to(1);
        input3.flush();
        worker.step_while(|| probe.less_than(input3.time()));
        println!("Loading finished after {:?}", timer.elapsed());
     
     
    })
    .unwrap();
}
