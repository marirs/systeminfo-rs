use systeminfo;

fn main() {
    let sysinfo = systeminfo::from_system_hardware();
    println!("{:#?}", sysinfo);

    let osinfo = systeminfo::from_system_os();
    println!("{:#?}", osinfo);
}
