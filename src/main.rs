use std::{thread::sleep, time::Duration};
use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::GetModuleHandleA, Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<()> {
    unsafe {
        let instance = GetModuleHandleA(None)?;
        debug_assert!(instance.0 != std::ptr::null_mut());

        let window_class = s!("Sample Window Class");

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: std::mem::transmute(instance),
            lpszClassName: window_class,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);

        let _hwnd = CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            window_class,
            s!("Slow Window"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            400,
            400,
            None,
            None,
            Some(std::mem::transmute(instance)),
            None,
        );

        let mut message = MSG::default();
        while GetMessageA(&mut message, Some(HWND(std::ptr::null_mut())), 0, 0).into() {
            let _ = TranslateMessage(&message);
            DispatchMessageA(&message);
        }

        Ok(())
    }
}

extern "system" fn wndproc(hwnd: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            WM_WINDOWPOSCHANGING | WM_MOVING | WM_SIZING => {
                println!("Window is being moved/resized - sleeping for 1 second");
                sleep(Duration::from_secs(1));
                DefWindowProcA(hwnd, message, wparam, lparam)
            }
            WM_PAINT => {
                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(hwnd, &mut ps);
                let _ = FillRect(hdc, &ps.rcPaint, HBRUSH((COLOR_WINDOW.0 + 1) as *mut _));
                let _ = EndPaint(hwnd, &ps);
                LRESULT(0)
            }
            _ => DefWindowProcA(hwnd, message, wparam, lparam),
        }
    }
}
