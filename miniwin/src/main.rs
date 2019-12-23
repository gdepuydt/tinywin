#![no_main]
#![no_std]

#![windows_subsystem = "windows"]
// #[cfg(windows)] extern crate winapi;

use core::mem::MaybeUninit;
use core::panic::PanicInfo;
use core::ffi::c_void;

const WM_PAINT: u32 = 0x000F;
const WM_DESTROY: u32 = 0x0002;
const DT_SINGLELINE: u64 = 0x00000020;
const DT_CENTER: u64 = 0x00000001;
const DT_VCENTER: u64 = 0x00000004;
const CS_OWNDC: u64 = 0x0020;
const CS_HREDRAW: u64 = 0x0002;
const CS_VREDRAW: u64 = 0x0001;
const WS_VISIBLE: u64= 0x10000000;
const CW_USEDEFAULT: i32 = 2147483648u32 as i32;
const WS_OVERLAPPED: u64 = 0x00000000; 
const WS_CAPTION: u64 = 0x00C00000;
const WS_SYSMENU: u64 = 0x00080000;
const WS_THICKFRAME: u64 = 0x00040000;
const WS_MINIMIZEBOX: u64 = 0x00020000;
const WS_MAXIMIZEBOX: u64 = 0x00010000;
const WS_OVERLAPPEDWINDOW: u64 = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;

pub enum HWND__ {}
type HWND = *mut HWND__;

pub enum HMENU__ {}
type HMENU = *mut HMENU__;

pub enum HBRUSH__ {}
type HBRUSH = *mut HBRUSH__;

pub enum HCURSOR__ {}
type HCURSOR = *mut HCURSOR__;

pub enum HICON__ {}
type HICON = *mut HICON__;

pub enum HINSTANCE__ {}
type HINSTANCE = *mut HINSTANCE__;

pub enum HDC__ {}
type HDC = *mut HDC__;

type HMODULE = HINSTANCE;

type WNDPROC = unsafe extern "system" fn(_: HWND, _: u32, _: u64, _: i32) -> i32;

#[repr(C)]
pub struct WNDCLASSA {
    pub style: u64,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: *const i8,
    pub lpszClassName: *const i8,
}

#[repr(C)]
pub struct POINT {
    x: i32,
    y: i32,
}

#[repr(C)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: u32,
    pub wParam: usize,
    pub lParam: isize,
    pub time: u64,
    pub pt: POINT,
}

#[link(name="Kernel32")]
extern "system" {
    fn GetModuleHandleA(lpModuleName: *const i8) -> HMODULE;
}

#[link(name="User32")]
extern "system" {
    fn RegisterClassA(lpWndClass: *const WNDCLASSA,) -> u16;
    fn CreateWindowExA(
        dwExStyle: u64,
        lpClassName: *const i8,
        lpWindowName: *const i8,
        dwStyle: u64,
        x: i32,
        y: i32,
        nWidth: i32,
        nHeight: i32,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: *mut c_void,
    ) -> HWND;
    fn DrawTextA(
        hdc: HDC,
        lpchText: *const i8,
        cchText: i32,
        lprc: *mut c_void,
        format: u64) -> i32;
    fn BeginPaint(hWnd: HWND, lpPaint: *mut c_void) -> HDC;
    fn EndPaint(hWnd: HWND, lpPaint: *mut c_void) -> i32;
    fn GetClientRect(hWnd: HWND, lpRect: *mut c_void) -> i32;
    fn DefWindowProcA(hWnd: HWND, Msg: u32, wParam: u64, lParam: i32,) -> i32;
    fn TranslateMessage(lpmsg: *const c_void) -> i32;
    fn DispatchMessageA(lpmsg: *const c_void) -> i32;
    fn GetMessageA(lpMsg: *mut c_void, hWnd: HWND, wMsgFilterMin: u32, wMsgFilterMax: u32) -> i32;
    fn PostQuitMessage(nExitCode: i32); 
}

pub unsafe extern "system" fn window_proc(hwnd: HWND,
    msg: u32, wparam: u64, lparam: i32) -> i32 {

    match msg {
        WM_PAINT => {
            let mut paint_struct = MaybeUninit::uninit();
            let mut rect = MaybeUninit::uninit();
            let hdc = BeginPaint(hwnd, paint_struct.as_mut_ptr());
            GetClientRect(hwnd, rect.as_mut_ptr());
            DrawTextA(hdc, "Hello world\0".as_ptr() as *const i8, -1, rect.as_mut_ptr(), DT_SINGLELINE | DT_CENTER | DT_VCENTER);
            EndPaint(hwnd, paint_struct.as_mut_ptr());
        }
        WM_DESTROY => {
            PostQuitMessage(0);
        }
        _ => { return DefWindowProcA(hwnd, msg, wparam, lparam); }
    }
    return 0;
}

fn create_window( ) -> HWND {
    unsafe {
        let hinstance = GetModuleHandleA( 0 as *const i8 );
        let wnd_class = WNDCLASSA {
            style : CS_OWNDC | CS_HREDRAW | CS_VREDRAW,     
            lpfnWndProc : window_proc,
            hInstance : hinstance,
            lpszClassName : "MyClass\0".as_ptr() as *const i8,
            cbClsExtra : 0,									
            cbWndExtra : 0,
            hIcon: 0 as HICON,
            hCursor: 0 as HCURSOR,
            hbrBackground: 0 as HBRUSH,
            lpszMenuName: 0 as *const i8,
        };
        RegisterClassA( &wnd_class );

        CreateWindowExA(
            0,									// dwExStyle 
            "MyClass\0".as_ptr() as *const i8,		                // class we registered.
            "MiniWIN\0".as_ptr() as *const i8,						// title
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,	// dwStyle
            CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT,	// size and position
            0 as HWND,               	// hWndParent
            0 as HMENU,					// hMenu
            hinstance,                  // hInstance
            0 as *mut c_void )				// lpParam
    }
}

// More info: https://msdn.microsoft.com/en-us/library/windows/desktop/ms644927(v=vs.85).aspx
fn handle_message( window : HWND ) -> bool {
    unsafe {
        let mut msg = MaybeUninit::uninit();
        if GetMessageA( msg.as_mut_ptr(), window, 0, 0 ) > 0 {
                TranslateMessage( msg.as_ptr() );
                DispatchMessageA( msg.as_ptr() );
            true
        } else {
            false
        }
    }
}

#[panic_handler]
#[no_mangle]
pub extern fn panic( _info: &PanicInfo ) -> ! { loop {} }

#[no_mangle]
pub extern "system" fn mainCRTStartup() {
    let window = create_window(  );
    loop {
        if !handle_message( window ) {
            break;
        }
    }
}