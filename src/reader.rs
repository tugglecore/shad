use windows::Win32::NetworkManagement::IpHelper::{
    GetExtendedUdpTable, GetTcpTable2, MIB_TCPTABLE2, MIB_UDPTABLE_OWNER_PID, UDP_TABLE_CLASS,
};

#[derive(Debug)]
pub struct Socket {
    pub process_id: String,
    pub protocol: String,
    pub local_address: String,
    pub local_port: String,
    pub remote_address: String,
    pub remote_port: String,
}

pub fn read_sockets() -> Vec<Socket> {
    let mut sockets = vec![];
    let mut size = u32::default();
    let return_value = unsafe { GetTcpTable2(Some(std::ptr::null_mut()), &mut size, false) };

    if return_value != 122 {
        panic!("Failed to read sockets")
    }

    let mut buffer = Vec::<MIB_TCPTABLE2>::with_capacity(size as usize);
    let return_value = unsafe { GetTcpTable2(Some(buffer.as_mut_ptr()), &mut size, true) };

    if return_value != 0 {
        panic!("Failed to read sockets")
    }

    let entries = unsafe { (*buffer.as_mut_ptr().cast::<MIB_TCPTABLE2>()).dwNumEntries };

    for i in 0..entries {
        let entry = unsafe { *(*buffer.as_ptr()).table.as_ptr().add(i as usize) };
        let socket = Socket {
            process_id: format!("{}", entry.dwOwningPid),
            protocol: String::from("TCP"),
            local_address: format!("{}", entry.dwLocalAddr),
            local_port: format!("{}", entry.dwLocalPort),
            remote_address: format!("{}", entry.dwRemoteAddr),
            remote_port: format!("{}", entry.dwRemotePort),
        };
        sockets.push(socket);
    }

    let mut size = u32::default();
    let return_value = unsafe {
        GetExtendedUdpTable(
            Some(std::ptr::null_mut()),
            &mut size,
            false,
            2u32,
            UDP_TABLE_CLASS(2),
            0,
        )
    };

    if return_value != 122 {
        panic!("Failed to read UDP sockets")
    }

    let mut buffer = Vec::<MIB_UDPTABLE_OWNER_PID>::with_capacity(size as usize);
    let return_value = unsafe {
        GetExtendedUdpTable(
            Some(buffer.as_mut_ptr() as *mut core::ffi::c_void),
            &mut size,
            false,
            2u32,
            UDP_TABLE_CLASS(2),
            0,
        )
    };

    if return_value != 0 {
        panic!("Failed to read UDP sockets")
    }

    let entries = unsafe { (*buffer.as_mut_ptr().cast::<MIB_TCPTABLE2>()).dwNumEntries };

    for i in 0..entries {
        let entry = unsafe { *(*buffer.as_ptr()).table.as_ptr().add(i as usize) };
        let socket = Socket {
            process_id: format!("{}", entry.dwOwningPid),
            protocol: String::from("UDP"),
            local_address: format!("{}", entry.dwLocalAddr),
            local_port: format!("{}", entry.dwLocalPort),
            remote_address: String::from("*"),
            remote_port: String::from("*"),
        };
        sockets.push(socket);
    }

    sockets
}
