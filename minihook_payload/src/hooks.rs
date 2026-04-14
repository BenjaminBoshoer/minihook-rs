use windows::Win32::Foundation;
use windows::Win32::UI::WindowsAndMessaging::{MESSAGEBOX_STYLE, MessageBoxExA, MB_OKCANCEL};
use windows::core::{s, PCSTR};

pub fn MyMessageBoxExA(
    /*hWnd: Option<Foundation::HWND>,
    lpText: PCSTR,
    lpCaption: PCSTR,
    utype: MESSAGEBOX_STYLE,
    wlanguageid: u16,*/
) -> i8 {
    let hooked_text = s!("Hooked");
    let hooked_caption = s!("Hooked");
    let result = unsafe { MessageBoxExA(None, hooked_caption, hooked_text, MB_OKCANCEL, 0) };
    0
}
