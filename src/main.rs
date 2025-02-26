pub mod app;
pub mod sockets;

pub use app::App;
use windows::Win32::NetworkManagement::IpHelper::{GetTcpTable2, MIB_TCPROW2, MIB_TCPTABLE2};

fn main() -> color_eyre::Result<()> {
    // color_eyre::install()?;
    // let terminal = ratatui::init();
    // let result = App::new().run(terminal);
    // ratatui::restore();
    // result
    print_sockets();
    Ok(())
}

fn print_sockets() {
    let mut size = u32::default();
    unsafe {
        let b = GetTcpTable2(Some(std::ptr::null_mut()), &mut size, false);
        println!("return status of first call: {}", b);
    }

    let mut buffer = Vec::<MIB_TCPTABLE2>::with_capacity(size as usize);
    unsafe {
        let a = GetTcpTable2(
            Some(buffer.as_mut_ptr()), 
            &mut size, true);
        println!("return status of second call: {}", a);
    }
    println!("Buffer: {buffer:#?}");
    println!("Sizepoint after second call: {size:#?}");
    unsafe {
        let num_of_entries = (*buffer.as_mut_ptr().cast::<MIB_TCPTABLE2>()).dwNumEntries;
        println!("What am I doning: {}", num_of_entries);
        let a = *(*buffer.as_mut_ptr()).table.as_ptr();
        println!("What am I doning: {:#?}", a);
        let a = *(*buffer.as_mut_ptr()).table.as_ptr().add(0);
        println!("What am I doning: {:#?}", a);
        let a = *(*buffer.as_mut_ptr()).table.as_ptr().add(1);
        println!("What am I doning: {:#?}", a);
        let a = *(*buffer.as_mut_ptr()).table.as_ptr().add(2);
        println!("What am I doning: {:#?}", a);

        let a = *(*buffer.as_mut_ptr()).table.as_ptr().add((num_of_entries - 1) as usize);
        println!("What am I doning: {:#?}", a);

        let a = *(*buffer.as_mut_ptr()).table.as_ptr().add((num_of_entries) as usize);
        println!("What am I doning: {:#?}", a);

        let a = *(*buffer.as_mut_ptr()).table.as_ptr().add((num_of_entries as usize) + 1);
        println!("What am I doning: {:#?}", a);
    };
}
