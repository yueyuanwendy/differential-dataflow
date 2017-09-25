SVI Implementation
=====
*If you want to run the code, run command "cargo run --example "file name""
-----
This project is based on differential dataflow , because many operators and the defined dataflow are related to the environment of the differential dataflow. The version of differential dataflow used is 0.1.1. The commit number is afb4e21a8ce7598808b62d07e0044b0e931a92ab. The version of Rust is 1.17.0., and the version of cargo is 0.18.0.


one SVI system code contains three parts. The first is the framework of differential dataflow:
```Rust
timely::execute_from_args(std::env::args().skip(2), move |worker| {
let (mut input1, mut input2, mut input3, probe) = worker.dataflow::<_, _, _>(|scope| {
let (input1, data1) = scope.new_collection();
let (input2, data2) = scope.new_collection();
let (input3, data3) = scope.new_collection();
…..dataflow…..
let probe = classify .probe();
(input1, input2, input3, probe)
});
```
Those codes define a differential dataflow. In the SVI, there are three kinds of the data in SVI including gene maps, variant patient and variant summary. Therefore there are three collections in the system. If the data is inserted and the input1 is used, it means these data is inserted into Data1. 
Then, the dataflow in the code shows how to analyze data by using operators and this dataflow is also the second important part of system.
![](C:\Users\apple\Desktop\proposal\disserentation\Dataflow.PNG)

Firstly, there are three kinds of data in the whole system. Data1 is the gene map, Data2 is the patient variant and Data3 is the patient summary. For the data1, the filter will be used like:
```Rust
filter(|x: &Vec<String>| {
x[7].to_lowercase().contains("alzheimer") ||
x[12].to_lowercase().contains("alzheimer")
})
```
This means that the filter will select the record which contains “alzheimer”. Then, use inspect operator to save the result. For example, the inspector related to the filter is:
```Rust
inspect(move |x| gis_writer.write_record(&x.0).expect("Cannot write to file"))
```
After obtaining the data, the inspect operator will use the CSV export function to save the result of filter. Before using the inspector, the save path needs to be defined like:
```Rust
let mut gis_writer = Writer::from_path("C:/Users/apple/Desktop/output/genes-in-scope.csv").expect("Cannot create file");
```
In all the code, the functions of inspectors are all the same. The difference is the path of save results are differnet They are all used to save the results. Then, Data 1 will use the flat_map like:
```Rust
flat_map(|row| {
let mut out = Vec::new();
let genes = row[6].split(',').map(|x| x.trim()).collect::<Vec<_>>();
for gene in genes {
out.push((gene.to_string(), row.to_vec()));
}
out.into_iter()
})
```
This part of the code can be divided into two parts. For every record, the sixth column will be divided by “,” and then become a vector. On the other hand, every element in the vector will be added into the record. 

Next, we use the join for the previous results basing on the key. It means that if the Data1 and Data2 have the same key, the record will be combined. Finally, we will get the variant-in-scope. For the next progress, the data need to be added a key by combing some elements, and this action should be finished by using the map. For instance, data3 needs to use map:
```Rust
map(|x:Vec<String>| (format!("chr{}_{}",x[13].to_owned(), x[14].to_owned()), x))
```
After that, Data3 and the previous results will use the join operator. Finally, we use the map operator to add different colors for annotation according the requirement. If the phenotype is pathogenicity, the color will be red. If it is benign, the color will be green. If the variant is unknown or uncertain, the color will be amber.

The third main part is the insertion of data which is also the main part of framework. In this system, the records are inserted into data as the vector like:
```Rust
let mut rdr = ReaderBuilder::new()
.has_headers(false)
.flexible(true)
.delimiter(b'\t')
.from_path("C:/Users/apple/Desktop/db/genemap2-160428-esc.txt").unwrap();

for result in rdr.records() {
let mut vec: Vec<String> = Vec::new();
let record = result.expect("a CSV record");
for field in record.iter() {
vec.push(String::from(field));
}
input1.insert(vec);
}
```
This is a typical function about inserting the data. The first is to import the CSV and insert one record as a vector. It should be noticed that “input1.insert()” represents that this data will be inserted into the Data1. There is a function like “input1.remove(data)”, and this function means that the data will be removed from the data1. For the differential dataflow, usually, there is at least one data insertion, and the data is the original data. 

In this system, there are several rounds about inserting the data. The first round is to insert the earliest version. After inserting the data, there are some codes like:
```Rust
input1.advance_to(1);
input1.flush();
input2.advance_to(1);
input2.flush();
input3.advance_to(1);
input3.flush();
worker.step_while(|| probe.less_than(input3.time()));
```
This means that this round is finished.
In the continue rounds, the system will insert or remove some difference data. For example, the code shows like:
```Rust
let mut rdr = ReaderBuilder::new()
.has_headers(false)
.flexible(true)
.delimiter(b'\t')
.from_path("C:/Users/apple/Desktop/db/gm-160428-160601_delta-minus.tsv").unwrap();

for result in rdr.records() {
let mut vec: Vec<String> = Vec::new();
let record = result.expect("a CSV record");
for field in record.iter() {
vec.push(String::from(field));
}
input1.remove(vec);
}

input1.advance_to(2);
input1.flush();
input2.advance_to(2);
input2.flush();
input3.advance_to(2);
input3.flush();
worker.step_while(|| probe.less_than(input3.time()));
```
In these rounds, the code read the data from path firstly. Then, “input1.remove(Vec)” means removing the data from Data1. Then, 
```Rust
input1.advance_to(2);
………
worker.step_while(|| probe.less_than(input3.time()));
```
These codes mean this round has been finished, and the dataflow can continue to next round.
