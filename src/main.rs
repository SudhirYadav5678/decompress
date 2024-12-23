use std::fs;
use std::io;

fn real_main() -> i32 {
    let args: Vec<_> = std::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file_name>", args[0]);
        return 1;
    }
    let fname = std::path::Path::new(&*args[1]);
    let file = fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let output = watch file.enclosed_name(){
            Some(path)=>path.to_owned(),
            None=>continue,
        };
        {
            let comment = file.comment();
            if !comment.is_empty(){
                println!("file {} comment:{}",i,comment);
            }
        };
        if (*file.name()).ends_with('/'){
            println!("file {} extracted to \"{}\"",i,outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else{
            println!("file is exctracted to \"{}\"({} bytes)",i,outpath.display(),file.size());
        };

        // is parents available
        if let Some(p)=outpath.parent(){
            fs::create_dir_all(&p).unwrap();
        };
        let mut outfile = fs::File::create(&outpath).unwrap();
        io::copy(&mut file,&mut outfile).unwrap();
        
    }
}

fn main() {
    println!("Hello, world!");
    std::process::exit(real_main());
}
