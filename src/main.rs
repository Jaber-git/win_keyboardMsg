use winapi::{
    ctypes::c_void,
    um::{
        winuser::{
            CreateWindowExW, DefWindowProcW, RegisterDeviceNotificationW, 
            WNDCLASSW, WM_DEVICECHANGE, WS_OVERLAPPED,
            DEVICE_NOTIFY_WINDOW_HANDLE,
        },
        dbt::{
            DEV_BROADCAST_DEVICEINTERFACE_W, DEV_BROADCAST_HDR,
            DBT_DEVTYP_DEVICEINTERFACE, DBT_DEVICEARRIVAL, DBT_DEVICEREMOVECOMPLETE
        },
        libloaderapi::GetModuleHandleW,
    },
    shared::{
        windef::{HWND, POINT},
        minwindef::{WPARAM, LPARAM, LRESULT, UINT},
        guiddef::GUID,
    }
};
use widestring::U16CString;
use std::{mem, ptr};

extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        match msg {
            WM_DEVICECHANGE => {
                // Check for null pointer before dereferencing
                if lparam != 0 {
                    let header = &*(lparam as *const DEV_BROADCAST_HDR);
                    
                    if header.dbch_devicetype == DBT_DEVTYP_DEVICEINTERFACE {
                        match wparam {
                            DBT_DEVICEARRIVAL => {
                                println!("Keyboard connected!");
                            }
                            DBT_DEVICEREMOVECOMPLETE => {
                                println!("Keyboard disconnected!");
                            }
                            _ => (),
                        }
                    }
                }
                DefWindowProcW(hwnd, msg, wparam, lparam)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

fn main() {
    // Register window class
    let class_name = U16CString::from_str("KeyboardMonitorClass").unwrap();
    let wnd_class = WNDCLASSW {
        lpfnWndProc: Some(window_proc),
        lpszClassName: class_name.as_ptr(),
        hInstance: unsafe { GetModuleHandleW(ptr::null()) },
        ..unsafe { mem::zeroed() }
    };

    unsafe {
        if winapi::um::winuser::RegisterClassW(&wnd_class) == 0 {
            panic!("Failed to register window class");
        }

        // Create message-only window
        let hwnd = CreateWindowExW(
            0,
            class_name.as_ptr(),
            U16CString::from_str("KeyboardMonitor").unwrap().as_ptr(),
            WS_OVERLAPPED,
            0, 0, 0, 0,
            ptr::null_mut(),
            ptr::null_mut(),
            wnd_class.hInstance,
            ptr::null_mut(),
        );

        if hwnd.is_null() {
            panic!("Failed to create window");
        }

        // Register for device notifications
        let mut filter = DEV_BROADCAST_DEVICEINTERFACE_W {
            dbcc_size: mem::size_of::<DEV_BROADCAST_DEVICEINTERFACE_W>() as u32,
            dbcc_devicetype: DBT_DEVTYP_DEVICEINTERFACE,
            dbcc_reserved: 0,
            dbcc_classguid: GUID {
                Data1: 0x884b96c3,
                Data2: 0x56ef,
                Data3: 0x11d1,
                Data4: [0xbc, 0x8c, 0x00, 0xa0, 0xc9, 0x14, 0x05, 0xdd],
            }, // GUID_DEVINTERFACE_KEYBOARD
            dbcc_name: [0; 1],
        };

        let hdev_notify = RegisterDeviceNotificationW(
            hwnd as *mut c_void,
            &mut filter as *mut _ as *mut c_void,
            DEVICE_NOTIFY_WINDOW_HANDLE,
        );

        if hdev_notify.is_null() {
            panic!("Failed to register for device notifications");
        }

        println!("Monitoring for keyboard connections/disconnections...");

        // Message loop
        let mut msg = winapi::um::winuser::MSG {
            hwnd: ptr::null_mut(),
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: POINT { x: 0, y: 0 },
        };
        
        while winapi::um::winuser::GetMessageW(&mut msg, hwnd, 0, 0) > 0 {
            winapi::um::winuser::TranslateMessage(&msg);
            winapi::um::winuser::DispatchMessageW(&msg);
        }
    }
}