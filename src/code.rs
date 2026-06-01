use std::os::raw::c_uchar;

use global_hotkey::hotkey::Code as GHCode;
use global_hotkey::hotkey::Modifiers as GHModifiers;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum HotKeyState {
    Pressed = 0,
    Released = 1,
    Cancelled = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MyCode {
    /// <code class="keycap">`~</code> on a US keyboard. This is the <code class="keycap">半角/全角/漢字</code> (<span class="unicode">hankaku/zenkaku/kanji</span>) key on Japanese keyboards
    Backquote = 0,
    /// Used for both the US <code class="keycap">\|</code> (on the 101-key layout) and also for the key
    /// located between the <code class="keycap">"</code> and <code class="keycap">Enter</code> keys on row C of the 102-,
    /// 104- and 106-key layouts.
    /// Labelled <code class="keycap">#~</code> on a UK (102) keyboard.
    Backslash,
    /// <code class="keycap">[{</code> on a US keyboard.
    BracketLeft,
    /// <code class="keycap">]}</code> on a US keyboard.
    BracketRight,
    /// <code class="keycap">,&lt;</code> on a US keyboard.
    Comma,
    /// <code class="keycap">0)</code> on a US keyboard.
    Digit0,
    /// <code class="keycap">1!</code> on a US keyboard.
    Digit1,
    /// <code class="keycap">2@</code> on a US keyboard.
    Digit2,
    /// <code class="keycap">3#</code> on a US keyboard.
    Digit3,
    /// <code class="keycap">4$</code> on a US keyboard.
    Digit4,
    /// <code class="keycap">5%</code> on a US keyboard.
    Digit5,
    /// <code class="keycap">6^</code> on a US keyboard.
    Digit6,
    /// <code class="keycap">7&amp;</code> on a US keyboard.
    Digit7,
    /// <code class="keycap">8*</code> on a US keyboard.
    Digit8,
    /// <code class="keycap">9(</code> on a US keyboard.
    Digit9,
    /// <code class="keycap">=+</code> on a US keyboard.
    Equal,
    /// Located between the left <code class="keycap">Shift</code> and <code class="keycap">Z</code> keys.
    /// Labelled <code class="keycap">\|</code> on a UK keyboard.
    IntlBackslash,
    /// Located between the <code class="keycap">/</code> and right <code class="keycap">Shift</code> keys.
    /// Labelled <code class="keycap">\ろ</code> (<span class="unicode">ro</span>) on a Japanese keyboard.
    IntlRo,
    /// Located between the <code class="keycap">=</code> and <code class="keycap">Backspace</code> keys.
    /// Labelled <code class="keycap">¥</code> (<span class="unicode">yen</span>) on a Japanese keyboard. <code class="keycap">\/</code> on a
    /// Russian keyboard.
    IntlYen,
    /// <code class="keycap">a</code> on a US keyboard.
    /// Labelled <code class="keycap">q</code> on an AZERTY (e.g., French) keyboard.
    KeyA,
    /// <code class="keycap">b</code> on a US keyboard.
    KeyB,
    /// <code class="keycap">c</code> on a US keyboard.
    KeyC,
    /// <code class="keycap">d</code> on a US keyboard.
    KeyD,
    /// <code class="keycap">e</code> on a US keyboard.
    KeyE,
    /// <code class="keycap">f</code> on a US keyboard.
    KeyF,
    /// <code class="keycap">g</code> on a US keyboard.
    KeyG,
    /// <code class="keycap">h</code> on a US keyboard.
    KeyH,
    /// <code class="keycap">i</code> on a US keyboard.
    KeyI,
    /// <code class="keycap">j</code> on a US keyboard.
    KeyJ,
    /// <code class="keycap">k</code> on a US keyboard.
    KeyK,
    /// <code class="keycap">l</code> on a US keyboard.
    KeyL,
    /// <code class="keycap">m</code> on a US keyboard.
    KeyM,
    /// <code class="keycap">n</code> on a US keyboard.
    KeyN,
    /// <code class="keycap">o</code> on a US keyboard.
    KeyO,
    /// <code class="keycap">p</code> on a US keyboard.
    KeyP,
    /// <code class="keycap">q</code> on a US keyboard.
    /// Labelled <code class="keycap">a</code> on an AZERTY (e.g., French) keyboard.
    KeyQ,
    /// <code class="keycap">r</code> on a US keyboard.
    KeyR,
    /// <code class="keycap">s</code> on a US keyboard.
    KeyS,
    /// <code class="keycap">t</code> on a US keyboard.
    KeyT,
    /// <code class="keycap">u</code> on a US keyboard.
    KeyU,
    /// <code class="keycap">v</code> on a US keyboard.
    KeyV,
    /// <code class="keycap">w</code> on a US keyboard.
    /// Labelled <code class="keycap">z</code> on an AZERTY (e.g., French) keyboard.
    KeyW,
    /// <code class="keycap">x</code> on a US keyboard.
    KeyX,
    /// <code class="keycap">y</code> on a US keyboard.
    /// Labelled <code class="keycap">z</code> on a QWERTZ (e.g., German) keyboard.
    KeyY,
    /// <code class="keycap">z</code> on a US keyboard.
    /// Labelled <code class="keycap">w</code> on an AZERTY (e.g., French) keyboard, and <code class="keycap">y</code> on a
    /// QWERTZ (e.g., German) keyboard.
    KeyZ,
    /// <code class="keycap">-_</code> on a US keyboard.
    Minus,
    /// <code class="keycap">.></code> on a US keyboard.
    Period,
    /// <code class="keycap">'"</code> on a US keyboard.
    Quote,
    /// <code class="keycap">;:</code> on a US keyboard.
    Semicolon,
    /// <code class="keycap">/?</code> on a US keyboard.
    Slash,
    /// <code class="keycap">Alt</code>, <code class="keycap">Option</code> or <code class="keycap">⌥</code>.
    AltLeft,
    /// <code class="keycap">Alt</code>, <code class="keycap">Option</code> or <code class="keycap">⌥</code>.
    /// This is labelled <code class="keycap">AltGr</code> key on many keyboard layouts.
    AltRight,
    /// <code class="keycap">Backspace</code> or <code class="keycap">⌫</code>.
    /// Labelled <code class="keycap">Delete</code> on Apple keyboards.
    Backspace,
    /// <code class="keycap">CapsLock</code> or <code class="keycap">⇪</code>
    CapsLock,
    /// The application context menu key, which is typically found between the right <code class="keycap">Meta</code> key and the right <code class="keycap">Control</code> key.
    ContextMenu,
    /// <code class="keycap">Control</code> or <code class="keycap">⌃</code>
    ControlLeft,
    /// <code class="keycap">Control</code> or <code class="keycap">⌃</code>
    ControlRight,
    /// <code class="keycap">Enter</code> or <code class="keycap">↵</code>. Labelled <code class="keycap">Return</code> on Apple keyboards.
    Enter,
    /// The Windows, <code class="keycap">⌘</code>, <code class="keycap">Command</code> or other OS symbol key.
    MetaLeft,
    /// The Windows, <code class="keycap">⌘</code>, <code class="keycap">Command</code> or other OS symbol key.
    MetaRight,
    /// <code class="keycap">Shift</code> or <code class="keycap">⇧</code>
    ShiftLeft,
    /// <code class="keycap">Shift</code> or <code class="keycap">⇧</code>
    ShiftRight,
    /// <code class="keycap"> </code> (space)
    Space,
    /// <code class="keycap">Tab</code> or <code class="keycap">⇥</code>
    Tab,
    /// Japanese: <code class="keycap">変換</code> (<span class="unicode">henkan</span>)
    Convert,
    /// Japanese: <code class="keycap">カタカナ/ひらがな/ローマ字</code> (<span class="unicode">katakana/hiragana/romaji</span>)
    KanaMode,
    /// Korean: HangulMode <code class="keycap">한/영</code> (<span class="unicode">han/yeong</span>)<br>Japanese (Mac keyboard): <code class="keycap">かな</code> (<span class="unicode">kana</span>)
    Lang1,
    /// Korean: Hanja <code class="keycap">한자</code> (<span class="unicode">hanja</span>)<br>Japanese (Mac keyboard): <code class="keycap">英数</code> (<span class="unicode">eisu</span>)
    Lang2,
    /// Japanese (word-processing keyboard): Katakana
    Lang3,
    /// Japanese (word-processing keyboard): Hiragana
    Lang4,
    /// Japanese (word-processing keyboard): Zenkaku/Hankaku
    Lang5,
    /// Japanese: <code class="keycap">無変換</code> (<span class="unicode">muhenkan</span>)
    NonConvert,
    /// <code class="keycap">⌦</code>. The forward delete key.
    /// Note that on Apple keyboards, the key labelled <code class="keycap">Delete</code> on the main part of
    /// the keyboard should be encoded as <code class="code">"Backspace"</code>.
    Delete,
    /// <code class="keycap">End</code> or <code class="keycap">↘</code>
    End,
    /// <code class="keycap">Help</code>. Not present on standard PC keyboards.
    Help,
    /// <code class="keycap">Home</code> or <code class="keycap">↖</code>
    Home,
    /// <code class="keycap">Insert</code> or <code class="keycap">Ins</code>. Not present on Apple keyboards.
    Insert,
    /// <code class="keycap">Page Down</code>, <code class="keycap">PgDn</code> or <code class="keycap">⇟</code>
    PageDown,
    /// <code class="keycap">Page Up</code>, <code class="keycap">PgUp</code> or <code class="keycap">⇞</code>
    PageUp,
    /// <code class="keycap">↓</code>
    ArrowDown,
    /// <code class="keycap">←</code>
    ArrowLeft,
    /// <code class="keycap">→</code>
    ArrowRight,
    /// <code class="keycap">↑</code>
    ArrowUp,
    /// On the Mac, the <code class="code">"NumLock"</code> code should be used for the numpad <code class="keycap">Clear</code> key.
    NumLock,
    /// <code class="keycap">0 Ins</code> on a keyboard<br><code class="keycap">0</code> on a phone or remote control
    Numpad0,
    /// <code class="keycap">1 End</code> on a keyboard<br><code class="keycap">1</code> or <code class="keycap">1 QZ</code> on a phone or
    /// remote control
    Numpad1,
    /// <code class="keycap">2 ↓</code> on a keyboard<br><code class="keycap">2 ABC</code> on a phone or remote control
    Numpad2,
    /// <code class="keycap">3 PgDn</code> on a keyboard<br><code class="keycap">3 DEF</code> on a phone or remote control
    Numpad3,
    /// <code class="keycap">4 ←</code> on a keyboard<br><code class="keycap">4 GHI</code> on a phone or remote control
    Numpad4,
    /// <code class="keycap">5</code> on a keyboard<br><code class="keycap">5 JKL</code> on a phone or remote control
    Numpad5,
    /// <code class="keycap">6 →</code> on a keyboard<br><code class="keycap">6 MNO</code> on a phone or remote control
    Numpad6,
    /// <code class="keycap">7 Home</code> on a keyboard<br><code class="keycap">7 PQRS</code> or <code class="keycap">7 PRS</code> on a phone
    /// or remote control
    Numpad7,
    /// <code class="keycap">8 ↑</code> on a keyboard<br><code class="keycap">8 TUV</code> on a phone or remote control
    Numpad8,
    /// <code class="keycap">9 PgUp</code> on a keyboard<br><code class="keycap">9 WXYZ</code> or <code class="keycap">9 WXY</code> on a phone
    /// or remote control
    Numpad9,
    /// <code class="keycap">+</code>
    NumpadAdd,
    /// Found on the Microsoft Natural Keyboard.
    NumpadBackspace,
    /// <code class="keycap">C</code> or <code class="keycap">AC</code> (All Clear). Also for use with numpads that have a <code class="keycap">Clear</code> key that is separate from the <code class="keycap">NumLock</code> key. On the Mac, the numpad <code class="keycap">Clear</code> key should always
    /// be encoded as <code class="code">"NumLock"</code>.
    NumpadClear,
    /// <code class="keycap">CE</code> (Clear Entry)
    NumpadClearEntry,
    /// <code class="keycap">,</code> (thousands separator). For locales where the thousands separator
    /// is a "." (e.g., Brazil), this key may generate a <code class="keycap">.</code>.
    NumpadComma,
    /// <code class="keycap">. Del</code>. For locales where the decimal separator is "," (e.g.,
    /// Brazil), this key may generate a <code class="keycap">,</code>.
    NumpadDecimal,
    /// <code class="keycap">/</code>
    NumpadDivide,
    NumpadEnter,
    /// <code class="keycap">=</code>
    NumpadEqual,
    /// <code class="keycap">#</code> on a phone or remote control device. This key is typically found
    /// below the <code class="keycap">9</code> key and to the right of the <code class="keycap">0</code> key.
    NumpadHash,
    /// <code class="keycap">M+</code> Add current entry to the value stored in memory.
    NumpadMemoryAdd,
    /// <code class="keycap">MC</code> Clear the value stored in memory.
    NumpadMemoryClear,
    /// <code class="keycap">MR</code> Replace the current entry with the value stored in memory.
    NumpadMemoryRecall,
    /// <code class="keycap">MS</code> Replace the value stored in memory with the current entry.
    NumpadMemoryStore,
    /// <code class="keycap">M-</code> Subtract current entry from the value stored in memory.
    NumpadMemorySubtract,
    /// <code class="keycap">*</code> on a keyboard. For use with numpads that provide mathematical
    /// operations (<code class="keycap">+</code>, <code class="keycap">-</code>, <code class="keycap">*</code> and <code class="keycap">/</code>).<br>Use <code class="code">"NumpadStar"</code> for the <code class="keycap">*</code> key on phones and remote controls.
    NumpadMultiply,
    /// <code class="keycap">(</code> Found on the Microsoft Natural Keyboard.
    NumpadParenLeft,
    /// <code class="keycap">)</code> Found on the Microsoft Natural Keyboard.
    NumpadParenRight,
    /// <code class="keycap">*</code> on a phone or remote control device.
    /// This key is typically found below the <code class="keycap">7</code> key and to the left of
    /// the <code class="keycap">0</code> key.<br>Use <code class="code">"NumpadMultiply"</code> for the <code class="keycap">*</code> key on
    /// numeric keypads.
    NumpadStar,
    /// <code class="keycap">-</code>
    NumpadSubtract,
    /// <code class="keycap">Esc</code> or <code class="keycap">⎋</code>
    Escape,
    /// <code class="keycap">Fn</code> This is typically a hardware key that does not generate a separate
    /// code. Most keyboards do not place this key in the function section, but it is
    /// included here to keep it with related keys.
    Fn,
    /// <code class="keycap">FLock</code> or <code class="keycap">FnLock</code>. Function Lock key. Found on the Microsoft
    /// Natural Keyboard.
    FnLock,
    /// <code class="keycap">PrtScr SysRq</code> or <code class="keycap">Print Screen</code>
    PrintScreen,
    /// <code class="keycap">Scroll Lock</code>
    ScrollLock,
    /// <code class="keycap">Pause Break</code>
    Pause,
    /// Some laptops place this key to the left of the <code class="keycap">↑</code> key.
    BrowserBack,
    BrowserFavorites,
    /// Some laptops place this key to the right of the <code class="keycap">↑</code> key.
    BrowserForward,
    BrowserHome,
    BrowserRefresh,
    BrowserSearch,
    BrowserStop,
    /// <code class="keycap">Eject</code> or <code class="keycap">⏏</code>. This key is placed in the <a data-link-type="dfn" href="#function-section" id="ref-for-function-section①①">function
    /// section</a> on some Apple keyboards.
    Eject,
    /// Sometimes labelled <code class="keycap">My Computer</code> on the keyboard
    LaunchApp1,
    /// Sometimes labelled <code class="keycap">Calculator</code> on the keyboard
    LaunchApp2,
    LaunchMail,
    MediaPlayPause,
    MediaSelect,
    MediaStop,
    MediaTrackNext,
    MediaTrackPrevious,
    /// This key is placed in the function section on some Apple keyboards,
    /// replacing the <code class="keycap">Eject</code> key.
    Power,
    Sleep,
    AudioVolumeDown,
    AudioVolumeMute,
    AudioVolumeUp,
    WakeUp,
    Hyper,
    Super,
    Turbo,
    Abort,
    Resume,
    Suspend,
    /// Found on Sun’s USB keyboard.
    Again,
    /// Found on Sun’s USB keyboard.
    Copy,
    /// Found on Sun’s USB keyboard.
    Cut,
    /// Found on Sun’s USB keyboard.
    Find,
    /// Found on Sun’s USB keyboard.
    Open,
    /// Found on Sun’s USB keyboard.
    Paste,
    /// Found on Sun’s USB keyboard.
    Props,
    /// Found on Sun’s USB keyboard.
    Select,
    /// Found on Sun’s USB keyboard.
    Undo,
    /// Use for dedicated <code class="keycap">ひらがな</code> key found on some Japanese word processing keyboards.
    Hiragana,
    /// Use for dedicated <code class="keycap">カタカナ</code> key found on some Japanese word processing keyboards.
    Katakana,
    /// This value code should be used when no other
    /// value given in this specification is appropriate.
    Unidentified,
    /// <code class="keycap">F1</code>
    F1,
    /// <code class="keycap">F2</code>
    F2,
    /// <code class="keycap">F3</code>
    F3,
    /// <code class="keycap">F4</code>
    F4,
    /// <code class="keycap">F5</code>
    F5,
    /// <code class="keycap">F6</code>
    F6,
    /// <code class="keycap">F7</code>
    F7,
    /// <code class="keycap">F8</code>
    F8,
    /// <code class="keycap">F9</code>
    F9,
    /// <code class="keycap">F10</code>
    F10,
    /// <code class="keycap">F11</code>
    F11,
    /// <code class="keycap">F12</code>
    F12,
    /// <code class="keycap">F13</code>
    F13,
    /// <code class="keycap">F14</code>
    F14,
    /// <code class="keycap">F15</code>
    F15,
    /// <code class="keycap">F16</code>
    F16,
    /// <code class="keycap">F17</code>
    F17,
    /// <code class="keycap">F18</code>
    F18,
    /// <code class="keycap">F19</code>
    F19,
    /// <code class="keycap">F20</code>
    F20,
    /// <code class="keycap">F21</code>
    F21,
    /// <code class="keycap">F22</code>
    F22,
    /// <code class="keycap">F23</code>
    F23,
    /// <code class="keycap">F24</code>
    F24,
    /// <code class="keycap">F25</code>
    F25,
    /// <code class="keycap">F26</code>
    F26,
    /// <code class="keycap">F27</code>
    F27,
    /// <code class="keycap">F28</code>
    F28,
    /// <code class="keycap">F29</code>
    F29,
    /// <code class="keycap">F30</code>
    F30,
    /// <code class="keycap">F31</code>
    F31,
    /// <code class="keycap">F32</code>
    F32,
    /// <code class="keycap">F33</code>
    F33,
    /// <code class="keycap">F34</code>
    F34,
    /// <code class="keycap">F35</code>
    F35,
    /// Non-standard code value supported by Chromium.
    BrightnessDown,
    /// Non-standard code value supported by Chromium.
    BrightnessUp,
    /// Non-standard code value supported by Chromium.
    DisplayToggleIntExt,
    /// Non-standard code value supported by Chromium.
    KeyboardLayoutSelect,
    /// Non-standard code value supported by Chromium.
    LaunchAssistant,
    /// Non-standard code value supported by Chromium.
    LaunchControlPanel,
    /// Non-standard code value supported by Chromium.
    LaunchScreenSaver,
    /// Non-standard code value supported by Chromium.
    MailForward,
    /// Non-standard code value supported by Chromium.
    MailReply,
    /// Non-standard code value supported by Chromium.
    MailSend,
    /// Non-standard code value supported by Chromium.
    MediaFastForward,
    /// Non-standard code value supported by Chromium.
    MediaPause,
    /// Non-standard code value supported by Chromium.
    MediaPlay,
    /// Non-standard code value supported by Chromium.
    MediaRecord,
    /// Non-standard code value supported by Chromium.
    MediaRewind,
    /// Non-standard code value supported by Chromium.
    MicrophoneMuteToggle,
    /// Non-standard code value supported by Chromium.
    PrivacyScreenToggle,
    /// Non-standard code value supported by Chromium.
    SelectTask,
    /// Non-standard code value supported by Chromium.
    ShowAllWindows,
    /// Non-standard code value supported by Chromium.
    ZoomToggle,
}
pub fn digit_to_mycode(arr: c_uchar) -> Option<MyCode> {
    Some(match arr {
        0 => MyCode::Backquote,
        1 => MyCode::Backslash,
        2 => MyCode::BracketLeft,
        3 => MyCode::BracketRight,
        4 => MyCode::Comma,
        5 => MyCode::Digit0,
        6 => MyCode::Digit1,
        7 => MyCode::Digit2,
        8 => MyCode::Digit3,
        9 => MyCode::Digit4,
        10 => MyCode::Digit5,
        11 => MyCode::Digit6,
        12 => MyCode::Digit7,
        13 => MyCode::Digit8,
        14 => MyCode::Digit9,
        15 => MyCode::Equal,
        16 => MyCode::IntlBackslash,
        17 => MyCode::IntlRo,
        18 => MyCode::IntlYen,
        19 => MyCode::KeyA,
        20 => MyCode::KeyB,
        21 => MyCode::KeyC,
        22 => MyCode::KeyD,
        23 => MyCode::KeyE,
        24 => MyCode::KeyF,
        25 => MyCode::KeyG,
        26 => MyCode::KeyH,
        27 => MyCode::KeyI,
        28 => MyCode::KeyJ,
        29 => MyCode::KeyK,
        30 => MyCode::KeyL,
        31 => MyCode::KeyM,
        32 => MyCode::KeyN,
        33 => MyCode::KeyO,
        34 => MyCode::KeyP,
        35 => MyCode::KeyQ,
        36 => MyCode::KeyR,
        37 => MyCode::KeyS,
        38 => MyCode::KeyT,
        39 => MyCode::KeyU,
        40 => MyCode::KeyV,
        41 => MyCode::KeyW,
        42 => MyCode::KeyX,
        43 => MyCode::KeyY,
        44 => MyCode::KeyZ,
        45 => MyCode::Minus,
        46 => MyCode::Period,
        47 => MyCode::Quote,
        48 => MyCode::Semicolon,
        49 => MyCode::Slash,
        50 => MyCode::AltLeft,
        51 => MyCode::AltRight,
        52 => MyCode::Backspace,
        53 => MyCode::CapsLock,
        54 => MyCode::ContextMenu,
        55 => MyCode::ControlLeft,
        56 => MyCode::ControlRight,
        57 => MyCode::Enter,
        58 => MyCode::MetaLeft,
        59 => MyCode::MetaRight,
        60 => MyCode::ShiftLeft,
        61 => MyCode::ShiftRight,
        62 => MyCode::Space,
        63 => MyCode::Tab,
        64 => MyCode::Convert,
        65 => MyCode::KanaMode,
        66 => MyCode::Lang1,
        67 => MyCode::Lang2,
        68 => MyCode::Lang3,
        69 => MyCode::Lang4,
        70 => MyCode::Lang5,
        71 => MyCode::NonConvert,
        72 => MyCode::Delete,
        73 => MyCode::End,
        74 => MyCode::Help,
        75 => MyCode::Home,
        76 => MyCode::Insert,
        77 => MyCode::PageDown,
        78 => MyCode::PageUp,
        79 => MyCode::ArrowDown,
        80 => MyCode::ArrowLeft,
        81 => MyCode::ArrowRight,
        82 => MyCode::ArrowUp,
        83 => MyCode::NumLock,
        84 => MyCode::Numpad0,
        85 => MyCode::Numpad1,
        86 => MyCode::Numpad2,
        87 => MyCode::Numpad3,
        88 => MyCode::Numpad4,
        89 => MyCode::Numpad5,
        90 => MyCode::Numpad6,
        91 => MyCode::Numpad7,
        92 => MyCode::Numpad8,
        93 => MyCode::Numpad9,
        94 => MyCode::NumpadAdd,
        95 => MyCode::NumpadBackspace,
        96 => MyCode::NumpadClear,
        97 => MyCode::NumpadClearEntry,
        98 => MyCode::NumpadComma,
        99 => MyCode::NumpadDecimal,
        100 => MyCode::NumpadDivide,
        101 => MyCode::NumpadEnter,
        102 => MyCode::NumpadEqual,
        103 => MyCode::NumpadHash,
        104 => MyCode::NumpadMemoryAdd,
        105 => MyCode::NumpadMemoryClear,
        106 => MyCode::NumpadMemoryRecall,
        107 => MyCode::NumpadMemoryStore,
        108 => MyCode::NumpadMemorySubtract,
        109 => MyCode::NumpadMultiply,
        110 => MyCode::NumpadParenLeft,
        111 => MyCode::NumpadParenRight,
        112 => MyCode::NumpadStar,
        113 => MyCode::NumpadSubtract,
        114 => MyCode::Escape,
        115 => MyCode::Fn,
        116 => MyCode::FnLock,
        117 => MyCode::PrintScreen,
        118 => MyCode::ScrollLock,
        119 => MyCode::Pause,
        120 => MyCode::BrowserBack,
        121 => MyCode::BrowserFavorites,
        122 => MyCode::BrowserForward,
        123 => MyCode::BrowserHome,
        124 => MyCode::BrowserRefresh,
        125 => MyCode::BrowserSearch,
        126 => MyCode::BrowserStop,
        127 => MyCode::Eject,
        128 => MyCode::LaunchApp1,
        129 => MyCode::LaunchApp2,
        130 => MyCode::LaunchMail,
        131 => MyCode::MediaPlayPause,
        132 => MyCode::MediaSelect,
        133 => MyCode::MediaStop,
        134 => MyCode::MediaTrackNext,
        135 => MyCode::MediaTrackPrevious,
        136 => MyCode::Power,
        137 => MyCode::Sleep,
        138 => MyCode::AudioVolumeDown,
        139 => MyCode::AudioVolumeMute,
        140 => MyCode::AudioVolumeUp,
        141 => MyCode::WakeUp,
        142 => MyCode::Hyper,
        143 => MyCode::Super,
        144 => MyCode::Turbo,
        145 => MyCode::Abort,
        146 => MyCode::Resume,
        147 => MyCode::Suspend,
        148 => MyCode::Again,
        149 => MyCode::Copy,
        150 => MyCode::Cut,
        151 => MyCode::Find,
        152 => MyCode::Open,
        153 => MyCode::Paste,
        154 => MyCode::Props,
        155 => MyCode::Select,
        156 => MyCode::Undo,
        157 => MyCode::Hiragana,
        158 => MyCode::Katakana,
        159 => MyCode::Unidentified,
        160 => MyCode::F1,
        161 => MyCode::F2,
        162 => MyCode::F3,
        163 => MyCode::F4,
        164 => MyCode::F5,
        165 => MyCode::F6,
        166 => MyCode::F7,
        167 => MyCode::F8,
        168 => MyCode::F9,
        169 => MyCode::F10,
        170 => MyCode::F11,
        171 => MyCode::F12,
        172 => MyCode::F13,
        173 => MyCode::F14,
        174 => MyCode::F15,
        175 => MyCode::F16,
        176 => MyCode::F17,
        177 => MyCode::F18,
        178 => MyCode::F19,
        179 => MyCode::F20,
        180 => MyCode::F21,
        181 => MyCode::F22,
        182 => MyCode::F23,
        183 => MyCode::F24,
        184 => MyCode::F25,
        185 => MyCode::F26,
        186 => MyCode::F27,
        187 => MyCode::F28,
        188 => MyCode::F29,
        189 => MyCode::F30,
        190 => MyCode::F31,
        191 => MyCode::F32,
        192 => MyCode::F33,
        193 => MyCode::F34,
        194 => MyCode::F35,
        195 => MyCode::BrightnessDown,
        196 => MyCode::BrightnessUp,
        197 => MyCode::DisplayToggleIntExt,
        198 => MyCode::KeyboardLayoutSelect,
        199 => MyCode::LaunchAssistant,
        200 => MyCode::LaunchControlPanel,
        201 => MyCode::LaunchScreenSaver,
        202 => MyCode::MailForward,
        203 => MyCode::MailReply,
        204 => MyCode::MailSend,
        205 => MyCode::MediaFastForward,
        206 => MyCode::MediaPause,
        207 => MyCode::MediaPlay,
        208 => MyCode::MediaRecord,
        209 => MyCode::MediaRewind,
        210 => MyCode::MicrophoneMuteToggle,
        211 => MyCode::PrivacyScreenToggle,
        212 => MyCode::SelectTask,
        213 => MyCode::ShowAllWindows,
        214 => MyCode::ZoomToggle,
        _ => return None,
    })
}
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub  enum  MyModifiers {
    ALT = 0,
    AltGraph = 1,
    CapsLock = 2,
    Control = 3,
    Fn = 4,
    FnLock = 5,
    Meta = 6,
    NumLock = 7,
    ScrollLock = 8,
    Shift = 9,
    Symbol = 10,
    SymboLock = 11,
    Hyper = 12,
    Super = 13,
}

/// 将 u8 数字安全地转换为 Modifiers 枚举
pub fn digit_to_modifier(value: c_uchar) -> Option<MyModifiers> {
    Some(match value {
        0 => MyModifiers::ALT,
        1 => MyModifiers::AltGraph,
        2 => MyModifiers::CapsLock,
        3 => MyModifiers::Control,
        4 => MyModifiers::Fn,
        5 => MyModifiers::FnLock,
        6 => MyModifiers::Meta,
        7 => MyModifiers::NumLock,
        8 => MyModifiers::ScrollLock,
        9 => MyModifiers::Shift,
        10 => MyModifiers::Symbol,
        11 => MyModifiers::SymboLock,
        12 => MyModifiers::Hyper,
        13 => MyModifiers::Super,
        _ => return None,
    })
}
///    pub struct Modifiers: u32 {
///     const ALT = 0x01;
///     const ALT_GRAPH = 0x2;
///     const CAPS_LOCK = 0x4;
///     const CONTROL = 0x8;
///     const FN = 0x10;
///     const FN_LOCK = 0x20;
///     const META = 0x40;
///     const NUM_LOCK = 0x80;
///     const SCROLL_LOCK = 0x100;
///     const SHIFT = 0x200;
///     const SYMBOL = 0x400;
///     const SYMBOL_LOCK = 0x800;
///     const HYPER = 0x1000;
///     const SUPER = 0x2000;
/// }
/// 将单个自定义 Modifiers 枚举值转换为 global_hotkey 的 Modifiers 位标志
pub fn enum_modefiers_to_tauri_modefiers(modifier: MyModifiers) -> GHModifiers {
    match modifier {
        MyModifiers::ALT => GHModifiers::ALT,
        MyModifiers::AltGraph => GHModifiers::ALT_GRAPH,
        MyModifiers::CapsLock => GHModifiers::CAPS_LOCK,
        MyModifiers::Control => GHModifiers::CONTROL,
        MyModifiers::Fn => GHModifiers::FN,
        MyModifiers::FnLock => GHModifiers::FN_LOCK,
        MyModifiers::Meta => GHModifiers::META,
        MyModifiers::NumLock => GHModifiers::NUM_LOCK,
        MyModifiers::ScrollLock => GHModifiers::SCROLL_LOCK,
        MyModifiers::Shift => GHModifiers::SHIFT,
        MyModifiers::Symbol => GHModifiers::SYMBOL,
        MyModifiers::SymboLock => GHModifiers::SYMBOL_LOCK,
        MyModifiers::Hyper => GHModifiers::HYPER,
        MyModifiers::Super => GHModifiers::SUPER,
    }
}

/// 将单个 MyCode 枚举值转换为 global_hotkey 的 Code
pub fn mycode_to_global_code(code: MyCode) -> GHCode {
    match code {
        MyCode::Backquote => GHCode::Backquote,
        MyCode::Backslash => GHCode::Backslash,
        MyCode::BracketLeft => GHCode::BracketLeft,
        MyCode::BracketRight => GHCode::BracketRight,
        MyCode::Comma => GHCode::Comma,
        MyCode::Digit0 => GHCode::Digit0,
        MyCode::Digit1 => GHCode::Digit1,
        MyCode::Digit2 => GHCode::Digit2,
        MyCode::Digit3 => GHCode::Digit3,
        MyCode::Digit4 => GHCode::Digit4,
        MyCode::Digit5 => GHCode::Digit5,
        MyCode::Digit6 => GHCode::Digit6,
        MyCode::Digit7 => GHCode::Digit7,
        MyCode::Digit8 => GHCode::Digit8,
        MyCode::Digit9 => GHCode::Digit9,
        MyCode::Equal => GHCode::Equal,
        MyCode::IntlBackslash => GHCode::IntlBackslash,
        MyCode::IntlRo => GHCode::IntlRo,
        MyCode::IntlYen => GHCode::IntlYen,
        MyCode::KeyA => GHCode::KeyA,
        MyCode::KeyB => GHCode::KeyB,
        MyCode::KeyC => GHCode::KeyC,
        MyCode::KeyD => GHCode::KeyD,
        MyCode::KeyE => GHCode::KeyE,
        MyCode::KeyF => GHCode::KeyF,
        MyCode::KeyG => GHCode::KeyG,
        MyCode::KeyH => GHCode::KeyH,
        MyCode::KeyI => GHCode::KeyI,
        MyCode::KeyJ => GHCode::KeyJ,
        MyCode::KeyK => GHCode::KeyK,
        MyCode::KeyL => GHCode::KeyL,
        MyCode::KeyM => GHCode::KeyM,
        MyCode::KeyN => GHCode::KeyN,
        MyCode::KeyO => GHCode::KeyO,
        MyCode::KeyP => GHCode::KeyP,
        MyCode::KeyQ => GHCode::KeyQ,
        MyCode::KeyR => GHCode::KeyR,
        MyCode::KeyS => GHCode::KeyS,
        MyCode::KeyT => GHCode::KeyT,
        MyCode::KeyU => GHCode::KeyU,
        MyCode::KeyV => GHCode::KeyV,
        MyCode::KeyW => GHCode::KeyW,
        MyCode::KeyX => GHCode::KeyX,
        MyCode::KeyY => GHCode::KeyY,
        MyCode::KeyZ => GHCode::KeyZ,
        MyCode::Minus => GHCode::Minus,
        MyCode::Period => GHCode::Period,
        MyCode::Quote => GHCode::Quote,
        MyCode::Semicolon => GHCode::Semicolon,
        MyCode::Slash => GHCode::Slash,
        MyCode::AltLeft => GHCode::AltLeft,
        MyCode::AltRight => GHCode::AltRight,
        MyCode::Backspace => GHCode::Backspace,
        MyCode::CapsLock => GHCode::CapsLock,
        MyCode::ContextMenu => GHCode::ContextMenu,
        MyCode::ControlLeft => GHCode::ControlLeft,
        MyCode::ControlRight => GHCode::ControlRight,
        MyCode::Enter => GHCode::Enter,
        MyCode::MetaLeft => GHCode::MetaLeft,
        MyCode::MetaRight => GHCode::MetaRight,
        MyCode::ShiftLeft => GHCode::ShiftLeft,
        MyCode::ShiftRight => GHCode::ShiftRight,
        MyCode::Space => GHCode::Space,
        MyCode::Tab => GHCode::Tab,
        MyCode::Convert => GHCode::Convert,
        MyCode::KanaMode => GHCode::KanaMode,
        MyCode::Lang1 => GHCode::Lang1,
        MyCode::Lang2 => GHCode::Lang2,
        MyCode::Lang3 => GHCode::Lang3,
        MyCode::Lang4 => GHCode::Lang4,
        MyCode::Lang5 => GHCode::Lang5,
        MyCode::NonConvert => GHCode::NonConvert,
        MyCode::Delete => GHCode::Delete,
        MyCode::End => GHCode::End,
        MyCode::Help => GHCode::Help,
        MyCode::Home => GHCode::Home,
        MyCode::Insert => GHCode::Insert,
        MyCode::PageDown => GHCode::PageDown,
        MyCode::PageUp => GHCode::PageUp,
        MyCode::ArrowDown => GHCode::ArrowDown,
        MyCode::ArrowLeft => GHCode::ArrowLeft,
        MyCode::ArrowRight => GHCode::ArrowRight,
        MyCode::ArrowUp => GHCode::ArrowUp,
        MyCode::NumLock => GHCode::NumLock,
        MyCode::Numpad0 => GHCode::Numpad0,
        MyCode::Numpad1 => GHCode::Numpad1,
        MyCode::Numpad2 => GHCode::Numpad2,
        MyCode::Numpad3 => GHCode::Numpad3,
        MyCode::Numpad4 => GHCode::Numpad4,
        MyCode::Numpad5 => GHCode::Numpad5,
        MyCode::Numpad6 => GHCode::Numpad6,
        MyCode::Numpad7 => GHCode::Numpad7,
        MyCode::Numpad8 => GHCode::Numpad8,
        MyCode::Numpad9 => GHCode::Numpad9,
        MyCode::NumpadAdd => GHCode::NumpadAdd,
        MyCode::NumpadBackspace => GHCode::NumpadBackspace,
        MyCode::NumpadClear => GHCode::NumpadClear,
        MyCode::NumpadClearEntry => GHCode::NumpadClearEntry,
        MyCode::NumpadComma => GHCode::NumpadComma,
        MyCode::NumpadDecimal => GHCode::NumpadDecimal,
        MyCode::NumpadDivide => GHCode::NumpadDivide,
        MyCode::NumpadEnter => GHCode::NumpadEnter,
        MyCode::NumpadEqual => GHCode::NumpadEqual,
        MyCode::NumpadHash => GHCode::NumpadHash,
        MyCode::NumpadMemoryAdd => GHCode::NumpadMemoryAdd,
        MyCode::NumpadMemoryClear => GHCode::NumpadMemoryClear,
        MyCode::NumpadMemoryRecall => GHCode::NumpadMemoryRecall,
        MyCode::NumpadMemoryStore => GHCode::NumpadMemoryStore,
        MyCode::NumpadMemorySubtract => GHCode::NumpadMemorySubtract,
        MyCode::NumpadMultiply => GHCode::NumpadMultiply,
        MyCode::NumpadParenLeft => GHCode::NumpadParenLeft,
        MyCode::NumpadParenRight => GHCode::NumpadParenRight,
        MyCode::NumpadStar => GHCode::NumpadStar,
        MyCode::NumpadSubtract => GHCode::NumpadSubtract,
        MyCode::Escape => GHCode::Escape,
        MyCode::Fn => GHCode::Fn,
        MyCode::FnLock => GHCode::FnLock,
        MyCode::PrintScreen => GHCode::PrintScreen,
        MyCode::ScrollLock => GHCode::ScrollLock,
        MyCode::Pause => GHCode::Pause,
        MyCode::BrowserBack => GHCode::BrowserBack,
        MyCode::BrowserFavorites => GHCode::BrowserFavorites,
        MyCode::BrowserForward => GHCode::BrowserForward,
        MyCode::BrowserHome => GHCode::BrowserHome,
        MyCode::BrowserRefresh => GHCode::BrowserRefresh,
        MyCode::BrowserSearch => GHCode::BrowserSearch,
        MyCode::BrowserStop => GHCode::BrowserStop,
        MyCode::Eject => GHCode::Eject,
        MyCode::LaunchApp1 => GHCode::LaunchApp1,
        MyCode::LaunchApp2 => GHCode::LaunchApp2,
        MyCode::LaunchMail => GHCode::LaunchMail,
        MyCode::MediaPlayPause => GHCode::MediaPlayPause,
        MyCode::MediaSelect => GHCode::MediaSelect,
        MyCode::MediaStop => GHCode::MediaStop,
        MyCode::MediaTrackNext => GHCode::MediaTrackNext,
        MyCode::MediaTrackPrevious => GHCode::MediaTrackPrevious,
        MyCode::Power => GHCode::Power,
        MyCode::Sleep => GHCode::Sleep,
        MyCode::AudioVolumeDown => GHCode::AudioVolumeDown,
        MyCode::AudioVolumeMute => GHCode::AudioVolumeMute,
        MyCode::AudioVolumeUp => GHCode::AudioVolumeUp,
        MyCode::WakeUp => GHCode::WakeUp,
        MyCode::Hyper => GHCode::Hyper,
        MyCode::Super => GHCode::Super,
        MyCode::Turbo => GHCode::Turbo,
        MyCode::Abort => GHCode::Abort,
        MyCode::Resume => GHCode::Resume,
        MyCode::Suspend => GHCode::Suspend,
        MyCode::Again => GHCode::Again,
        MyCode::Copy => GHCode::Copy,
        MyCode::Cut => GHCode::Cut,
        MyCode::Find => GHCode::Find,
        MyCode::Open => GHCode::Open,
        MyCode::Paste => GHCode::Paste,
        MyCode::Props => GHCode::Props,
        MyCode::Select => GHCode::Select,
        MyCode::Undo => GHCode::Undo,
        MyCode::Hiragana => GHCode::Hiragana,
        MyCode::Katakana => GHCode::Katakana,
        MyCode::Unidentified => GHCode::Unidentified,
        MyCode::F1 => GHCode::F1,
        MyCode::F2 => GHCode::F2,
        MyCode::F3 => GHCode::F3,
        MyCode::F4 => GHCode::F4,
        MyCode::F5 => GHCode::F5,
        MyCode::F6 => GHCode::F6,
        MyCode::F7 => GHCode::F7,
        MyCode::F8 => GHCode::F8,
        MyCode::F9 => GHCode::F9,
        MyCode::F10 => GHCode::F10,
        MyCode::F11 => GHCode::F11,
        MyCode::F12 => GHCode::F12,
        MyCode::F13 => GHCode::F13,
        MyCode::F14 => GHCode::F14,
        MyCode::F15 => GHCode::F15,
        MyCode::F16 => GHCode::F16,
        MyCode::F17 => GHCode::F17,
        MyCode::F18 => GHCode::F18,
        MyCode::F19 => GHCode::F19,
        MyCode::F20 => GHCode::F20,
        MyCode::F21 => GHCode::F21,
        MyCode::F22 => GHCode::F22,
        MyCode::F23 => GHCode::F23,
        MyCode::F24 => GHCode::F24,
        MyCode::F25 => GHCode::F25,
        MyCode::F26 => GHCode::F26,
        MyCode::F27 => GHCode::F27,
        MyCode::F28 => GHCode::F28,
        MyCode::F29 => GHCode::F29,
        MyCode::F30 => GHCode::F30,
        MyCode::F31 => GHCode::F31,
        MyCode::F32 => GHCode::F32,
        MyCode::F33 => GHCode::F33,
        MyCode::F34 => GHCode::F34,
        MyCode::F35 => GHCode::F35,
        MyCode::BrightnessDown => GHCode::BrightnessDown,
        MyCode::BrightnessUp => GHCode::BrightnessUp,
        MyCode::DisplayToggleIntExt => GHCode::DisplayToggleIntExt,
        MyCode::KeyboardLayoutSelect => GHCode::KeyboardLayoutSelect,
        MyCode::LaunchAssistant => GHCode::LaunchAssistant,
        MyCode::LaunchControlPanel => GHCode::LaunchControlPanel,
        MyCode::LaunchScreenSaver => GHCode::LaunchScreenSaver,
        MyCode::MailForward => GHCode::MailForward,
        MyCode::MailReply => GHCode::MailReply,
        MyCode::MailSend => GHCode::MailSend,
        MyCode::MediaFastForward => GHCode::MediaFastForward,
        MyCode::MediaPause => GHCode::MediaPause,
        MyCode::MediaPlay => GHCode::MediaPlay,
        MyCode::MediaRecord => GHCode::MediaRecord,
        MyCode::MediaRewind => GHCode::MediaRewind,
        MyCode::MicrophoneMuteToggle => GHCode::MicrophoneMuteToggle,
        MyCode::PrivacyScreenToggle => GHCode::PrivacyScreenToggle,
        MyCode::SelectTask => GHCode::SelectTask,
        MyCode::ShowAllWindows => GHCode::ShowAllWindows,
        MyCode::ZoomToggle => GHCode::ZoomToggle,
    }
}

#[cfg(test)]
mod test {
    use crate::code::*;

    #[test]
    fn reflect() {
        assert_eq!(digit_to_mycode(0), Some(MyCode::Backquote));
        assert_eq!(digit_to_mycode(214), Some(MyCode::ZoomToggle));
    }
}
