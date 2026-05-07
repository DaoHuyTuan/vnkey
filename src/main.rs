mod engine;

#[cfg(windows)]
mod windows_app {
    use super::engine::{transform_word, InputMethod};
    use once_cell::sync::Lazy;
    use std::ffi::OsStr;
    use std::mem::size_of;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use std::sync::Mutex;
    use windows::core::PCWSTR;
    use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, POINT, WPARAM};
    use windows::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        GetKeyboardLayout, GetKeyboardState, MapVirtualKeyW, SendInput, ToUnicodeEx, INPUT,
        INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_UNICODE,
        MAP_VIRTUAL_KEY_TYPE, VIRTUAL_KEY, VK_BACK, VK_CONTROL, VK_MENU, VK_SHIFT,
    };
    use windows::Win32::UI::Shell::{
        Shell_NotifyIconW, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_DELETE, NOTIFYICONDATAW,
    };
    use windows::Win32::UI::WindowsAndMessaging::{
        AppendMenuW, CallNextHookEx, CreatePopupMenu, CreateWindowExW, DefWindowProcW,
        DestroyMenu, DestroyWindow, DispatchMessageW, GetCursorPos, GetForegroundWindow,
        GetMessageW, GetWindowThreadProcessId, LoadIconW, PostQuitMessage, RegisterClassW,
        SetForegroundWindow, SetWindowsHookExW, TrackPopupMenu, TranslateMessage, UnhookWindowsHookEx,
        HC_ACTION, HHOOK, HWND_MESSAGE, IDI_APPLICATION, KBDLLHOOKSTRUCT, MF_STRING, MSG,
        TPM_RIGHTBUTTON, WH_KEYBOARD_LL, WINDOW_EX_STYLE, WINDOW_STYLE, WM_APP, WM_COMMAND,
        WM_CONTEXTMENU, WM_DESTROY, WM_KEYDOWN, WM_RBUTTONUP, WM_SYSKEYDOWN, WNDCLASSW,
    };

    static STATE: Lazy<Mutex<AppState>> = Lazy::new(|| Mutex::new(AppState::default()));
    const WM_TRAYICON: u32 = WM_APP + 1;
    const TRAY_ICON_ID: u32 = 1;
    const MENU_EXIT_ID: usize = 1001;

    #[derive(Debug)]
    struct AppState {
        active_method: InputMethod,
        raw_word: String,
        rendered_word: String,
    }

    impl Default for AppState {
        fn default() -> Self {
            Self {
                active_method: InputMethod::Telex,
                raw_word: String::new(),
                rendered_word: String::new(),
            }
        }
    }

    pub fn run() {
        println!("vnkey chạy nền. Phím tắt đổi kiểu gõ: Ctrl+Shift+V");

        unsafe {
            let module = GetModuleHandleW(None).unwrap_or_default();
            let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_proc), HINSTANCE(module.0), 0);
            let Ok(hook) = hook else {
                eprintln!("Không cài được keyboard hook.");
                return;
            };

            let message_window = create_message_window(HINSTANCE(module.0));
            if message_window.0.is_null() {
                eprintln!("Không tạo được message window cho tray icon.");
                let _ = UnhookWindowsHookEx(hook);
                return;
            }

            if !add_tray_icon(message_window) {
                eprintln!("Không tạo được tray icon.");
                let _ = DestroyWindow(message_window);
                let _ = UnhookWindowsHookEx(hook);
                return;
            }

            let mut msg = MSG::default();
            while GetMessageW(&mut msg, None, 0, 0).into() {
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            remove_tray_icon(message_window);
            let _ = UnhookWindowsHookEx(hook);
        }
    }

    unsafe fn create_message_window(instance: HINSTANCE) -> HWND {
        let class_name = to_wide("vnkey-tray-window");
        let wnd_class = WNDCLASSW {
            lpfnWndProc: Some(window_proc),
            hInstance: instance,
            lpszClassName: PCWSTR(class_name.as_ptr()),
            ..Default::default()
        };
        let _ = RegisterClassW(&wnd_class);
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            PCWSTR(class_name.as_ptr()),
            PCWSTR(class_name.as_ptr()),
            WINDOW_STYLE::default(),
            0,
            0,
            0,
            0,
            HWND_MESSAGE,
            None,
            instance,
            None,
        )
        .unwrap_or_default()
    }

    unsafe extern "system" fn window_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match msg {
            WM_TRAYICON => {
                let event = lparam.0 as u32;
                if event == WM_RBUTTONUP || event == WM_CONTEXTMENU {
                    show_tray_menu(hwnd);
                }
                LRESULT(0)
            }
            WM_COMMAND => {
                if (wparam.0 & 0xffff) as usize == MENU_EXIT_ID {
                    let _ = DestroyWindow(hwnd);
                }
                LRESULT(0)
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }

    unsafe fn show_tray_menu(hwnd: HWND) {
        let menu = match CreatePopupMenu() {
            Ok(m) => m,
            Err(_) => return,
        };
        let exit_text = to_wide("Exit");
        let _ = AppendMenuW(menu, MF_STRING, MENU_EXIT_ID, PCWSTR(exit_text.as_ptr()));
        let mut cursor = POINT::default();
        if GetCursorPos(&mut cursor).is_ok() {
            let _ = SetForegroundWindow(hwnd);
            let _ = TrackPopupMenu(menu, TPM_RIGHTBUTTON, cursor.x, cursor.y, 0, hwnd, None);
        }
        let _ = DestroyMenu(menu);
    }

    unsafe fn add_tray_icon(hwnd: HWND) -> bool {
        let mut nid = NOTIFYICONDATAW::default();
        nid.cbSize = size_of::<NOTIFYICONDATAW>() as u32;
        nid.hWnd = hwnd;
        nid.uID = TRAY_ICON_ID;
        nid.uFlags = NIF_MESSAGE | NIF_TIP | NIF_ICON;
        nid.uCallbackMessage = WM_TRAYICON;
        nid.hIcon = LoadIconW(None, IDI_APPLICATION).unwrap_or_default();

        let tip = to_wide("vnkey - Right click to Exit");
        let max = nid.szTip.len().min(tip.len());
        nid.szTip[..max].copy_from_slice(&tip[..max]);

        Shell_NotifyIconW(NIM_ADD, &mut nid).as_bool()
    }

    unsafe fn remove_tray_icon(hwnd: HWND) {
        let mut nid = NOTIFYICONDATAW::default();
        nid.cbSize = size_of::<NOTIFYICONDATAW>() as u32;
        nid.hWnd = hwnd;
        nid.uID = TRAY_ICON_ID;
        let _ = Shell_NotifyIconW(NIM_DELETE, &mut nid);
    }

    unsafe extern "system" fn keyboard_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        if code == HC_ACTION as i32 {
            let kb = &*(lparam.0 as *const KBDLLHOOKSTRUCT);
            let is_keydown = wparam.0 as u32 == WM_KEYDOWN || wparam.0 as u32 == WM_SYSKEYDOWN;
            let injected = (kb.flags.0 & 0x10) != 0;

            if is_keydown && !injected {
                if is_toggle_hotkey(kb.vkCode as u16) {
                    toggle_method();
                    return LRESULT(1);
                }

                if kb.vkCode == VK_BACK.0 as u32 {
                    if handle_backspace() {
                        return LRESULT(1);
                    }
                } else if let Some(ch) = vk_to_char(kb.vkCode as u16, kb.scanCode) {
                    if handle_char(ch) {
                        return LRESULT(1);
                    }
                } else {
                    flush_word_if_boundary();
                }
            }
        }
        CallNextHookEx(HHOOK::default(), code, wparam, lparam)
    }

    fn toggle_method() {
        let mut state = STATE.lock().expect("state poisoned");
        state.active_method = match state.active_method {
            InputMethod::Telex => InputMethod::Vni,
            InputMethod::Vni => InputMethod::Telex,
        };
        let name = match state.active_method {
            InputMethod::Telex => "Telex",
            InputMethod::Vni => "VNI",
        };
        println!("Đổi kiểu gõ: {name}");
    }

    fn handle_char(ch: char) -> bool {
        if is_word_char(ch) {
            let mut state = STATE.lock().expect("state poisoned");
            let visible_len_before_key = state.rendered_word.chars().count();
            state.raw_word.push(ch);
            let converted = transform_word(&state.raw_word, state.active_method);
            let mut appended = state.rendered_word.clone();
            appended.push(ch);

            if converted == appended {
                // Plain character path: keep native app insertion behavior for
                // better compatibility (notably browser text fields).
                state.rendered_word = converted;
                return false;
            }

            if converted != state.rendered_word {
                unsafe {
                    // The current key is intercepted before Windows inserts it, so only
                    // remove characters that were already visible.
                    send_backspaces(visible_len_before_key);
                    send_unicode_text(&converted);
                }
                state.rendered_word = converted;
                return true;
            }
            // The key is consumed as a transform marker (e.g. tone/shape key that doesn't
            // produce a visible change), so block default insertion.
            return true;
        }

        let mut state = STATE.lock().expect("state poisoned");
        state.raw_word.clear();
        state.rendered_word.clear();
        false
    }

    fn handle_backspace() -> bool {
        let mut state = STATE.lock().expect("state poisoned");
        let old_len = state.rendered_word.chars().count();
        if old_len == 0 {
            return false;
        }

        // Delete one visible character, then trim raw keys until the recomposed
        // word matches that visual target length.
        let target_len = old_len.saturating_sub(1);
        loop {
            if state.raw_word.is_empty() {
                state.rendered_word.clear();
                break;
            }
            state.raw_word.pop();
            let recomposed = transform_word(&state.raw_word, state.active_method);
            if recomposed.chars().count() <= target_len {
                state.rendered_word = recomposed;
                break;
            }
        }

        unsafe {
            send_backspaces(old_len);
            if !state.rendered_word.is_empty() {
                send_unicode_text(&state.rendered_word);
            }
        }
        true
    }

    fn flush_word_if_boundary() {
        let mut state = STATE.lock().expect("state poisoned");
        state.raw_word.clear();
        state.rendered_word.clear();
    }

    unsafe fn send_backspaces(count: usize) {
        for _ in 0..count {
            let down = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_BACK,
                        wScan: 0,
                        dwFlags: Default::default(),
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };
            let up = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_BACK,
                        wScan: 0,
                        dwFlags: KEYEVENTF_KEYUP,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };
            let inputs = [down, up];
            let _ = SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
        }
    }

    unsafe fn send_unicode_text(text: &str) {
        for ch in text.encode_utf16() {
            let down = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0),
                        wScan: ch,
                        dwFlags: KEYEVENTF_UNICODE,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };
            let up = INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0),
                        wScan: ch,
                        dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            };
            let inputs = [down, up];
            let _ = SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
        }
    }

    fn is_word_char(ch: char) -> bool {
        ch.is_ascii_alphanumeric()
    }

    fn is_toggle_hotkey(vk: u16) -> bool {
        if vk != b'V' as u16 {
            return false;
        }
        unsafe {
            let mut ks = [0u8; 256];
            if GetKeyboardState(&mut ks).is_err() {
                return false;
            }
            let ctrl = ks[VK_CONTROL.0 as usize] & 0x80 != 0;
            let shift = ks[VK_SHIFT.0 as usize] & 0x80 != 0;
            let alt = ks[VK_MENU.0 as usize] & 0x80 != 0;
            ctrl && shift && !alt
        }
    }

    fn vk_to_char(vk: u16, scan: u32) -> Option<char> {
        unsafe {
            let mut key_state = [0u8; 256];
            if GetKeyboardState(&mut key_state).is_err() {
                return None;
            }

            let hwnd = GetForegroundWindow();
            let thread = GetWindowThreadProcessId(hwnd, Some(null_mut()));
            let layout = GetKeyboardLayout(thread);

            let mut buff = [0u16; 8];
            let result = ToUnicodeEx(
                vk as u32,
                if scan == 0 {
                    MapVirtualKeyW(vk as u32, MAP_VIRTUAL_KEY_TYPE(0))
                } else {
                    scan
                },
                &key_state,
                &mut buff,
                0,
                layout,
            );
            if result == 1 {
                char::from_u32(buff[0] as u32)
            } else {
                None
            }
        }
    }

    fn to_wide(value: &str) -> Vec<u16> {
        OsStr::new(value)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect()
    }
}

#[cfg(windows)]
fn main() {
    windows_app::run();
}

#[cfg(not(windows))]
fn main() {
    println!("vnkey hiện chỉ hỗ trợ Windows trong bản MVP này.");
}
