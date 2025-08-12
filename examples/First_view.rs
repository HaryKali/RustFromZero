fn main(){
    let data=  "\
   common name,length (cm)
   Little penguin,33
   Yellow-eyed penguin,65
   Fiordland penguin,60
   Invalid,data";

    let records=penguin_data.lines();
    for (i,record) in records.enumerate(){
        if i ==0 || record.trim().len()==0{
            continue;
        }
    }


    let fields: vec<_>=record.spilt(".").map(|field| field.trim()).collect();

    if cfg!(debug_assertions){
        eprintln!(”debug: {:?} -> {:?}",&record,&fields);"“);
    }

    let name=fields[0];






}