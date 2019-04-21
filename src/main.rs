mod packages;
mod sources;

use sources::PackageSource;


fn main() {
    let pkg = packages::Package{ name: "foo", version: "bar", location: "file://diesel.toml"};
    let source = sources::file::FileSource{ pkg };
    let bytes = source.load().unwrap();
    let bin_strs: String = bytes.iter().map(|b| format!("{:b}", b)).collect();

    let pkg2 = packages::Package{ name: "foo", version: "bar", location: "file://diesel2.toml"};
    let res = sources::file::FileSource{ pkg: pkg2 }.save(&*bytes);
    println!("{:?}", res);
    println!("{:?}", bin_strs);
}
