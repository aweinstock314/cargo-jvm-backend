use std::io;
use std::fs::File;

fn print_runtime_jar_classes() -> io::Result<()> {
    let path = "/usr/lib/jvm/java-8-openjdk-amd64/jre/lib/rt.jar"; // TODO: resolve at runtime via parsing the output of `java -verbose:class 2>&1 | grep Opened`
    let mut jar = zip::ZipArchive::new(io::BufReader::new(File::open(path)?))?;

    for i in 0..jar.len() {
        let mut file = jar.by_index(i).unwrap();
        if !file.name().ends_with(".class") { continue; }
        if let Ok(cls) = jreflection::Class::read(&mut file) {
            println!("{:?}", cls.path);
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    print_runtime_jar_classes()?;
}
