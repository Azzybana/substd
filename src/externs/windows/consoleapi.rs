#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

extern crate winapi;

use winapi::shared::minwindef::{BOOL, DWORD, LPDWORD, UINT, WORD};
use winapi::shared::ntdef::{HRESULT, LPCSTR, LPSTR};
use winapi::um::minwinbase::SECURITY_ATTRIBUTES;
use winapi::um::wincon::INPUT_RECORD;
use winapi::um::wincontypes::{
    CHAR_INFO,
    CONSOLE_CURSOR_INFO,
    CONSOLE_FONT_INFO,
    CONSOLE_FONT_INFOEX,
    CONSOLE_HISTORY_INFO,
    CONSOLE_READCONSOLE_CONTROL, // Added exposure
    CONSOLE_SCREEN_BUFFER_INFO,
    CONSOLE_SCREEN_BUFFER_INFOEX,
    CONSOLE_SELECTION_INFO,
    COORD,
    FOCUS_EVENT_RECORD, // Added exposure
    KEY_EVENT_RECORD,   // Added exposure
    MENU_EVENT_RECORD,  // Added exposure
    MOUSE_EVENT_RECORD, // Added exposure
    SMALL_RECT,
    WINDOW_BUFFER_SIZE_RECORD, // Added exposure
};
use winapi::um::winnt::HANDLE;

pub type HPCON = HANDLE;
pub type PHANDLER_ROUTINE = Option<unsafe extern "system" fn(dwCtrlType: DWORD) -> BOOL>;

pub const EVENT_CONSOLE_CARET: DWORD = 0x4001;
pub const EVENT_CONSOLE_END_APPLICATION: DWORD = 0x4007;
pub const EVENT_CONSOLE_LAYOUT: DWORD = 0x4005;
pub const EVENT_CONSOLE_START_APPLICATION: DWORD = 0x4006;
pub const EVENT_CONSOLE_UPDATE_REGION: DWORD = 0x4002;
pub const EVENT_CONSOLE_UPDATE_SCROLL: DWORD = 0x4004;
pub const EVENT_CONSOLE_UPDATE_SIMPLE: DWORD = 0x4003;

#[link(name = "kernel32")]
extern "system" {
    // AddConsoleAlias defines a console alias for the specified executable.
    pub fn AddConsoleAliasA(lpSource: LPCSTR, lpTarget: LPCSTR, lpExeName: LPCSTR) -> BOOL;

    // AllocConsole allocates a new console for the calling process.
    pub fn AllocConsole() -> BOOL;

    // AttachConsole attaches the calling process to the console of the specified process.
    pub fn AttachConsole(dwProcessId: DWORD) -> BOOL;

    // ClosePseudoConsole closes a pseudoconsole for the given handle.
    pub fn ClosePseudoConsole(hPC: HPCON) -> HRESULT;

    // CreatePseudoConsole allocates a new pseudoconsole for the calling process.
    pub fn CreatePseudoConsole(
        size: COORD,
        hInput: HANDLE,
        hOutput: HANDLE,
        dwFlags: DWORD,
        phPC: *mut HPCON,
    ) -> HRESULT;

    // CreateConsoleScreenBuffer creates a console screen buffer.
    pub fn CreateConsoleScreenBuffer(
        dwDesiredAccess: DWORD,
        dwShareMode: DWORD,
        lpSecurityAttributes: *const SECURITY_ATTRIBUTES,
        dwFlags: DWORD,
        lpScreenBufferData: *mut (),
    ) -> HANDLE;

    // FillConsoleOutputAttribute sets the text and background color attributes.
    pub fn FillConsoleOutputAttribute(
        hConsoleOutput: HANDLE,
        wAttribute: WORD,
        nLength: DWORD,
        dwWriteCoord: COORD,
        lpNumberOfAttrsWritten: LPDWORD,
    ) -> BOOL;

    // FillConsoleOutputCharacter writes a character to the console screen buffer.
    pub fn FillConsoleOutputCharacterA(
        hConsoleOutput: HANDLE,
        cCharacter: i8, // ANSI char
        nLength: DWORD,
        dwWriteCoord: COORD,
        lpNumberOfCharsWritten: LPDWORD,
    ) -> BOOL;

    // FlushConsoleInputBuffer flushes the console input buffer.
    pub fn FlushConsoleInputBuffer(hConsoleInput: HANDLE) -> BOOL;

    // FreeConsole detaches the calling process from its console.
    pub fn FreeConsole() -> BOOL;

    // GenerateConsoleCtrlEvent sends a specified signal to a console process group.
    pub fn GenerateConsoleCtrlEvent(dwCtrlEvent: DWORD, dwProcessGroupId: DWORD) -> BOOL;

    // GetConsoleAlias retrieves the specified alias for the specified executable.
    pub fn GetConsoleAliasA(
        lpSource: LPCSTR,
        lpTargetBuffer: LPSTR,
        TargetBufferLength: DWORD,
        lpExeName: LPCSTR,
    ) -> DWORD;

    // GetConsoleAliases retrieves all defined console aliases for the specified executable.
    pub fn GetConsoleAliasesA(
        lpAliasBuffer: LPSTR,
        AliasBufferLength: DWORD,
        lpExeName: LPCSTR,
    ) -> DWORD;

    // GetConsoleAliasesLength returns the size (in bytes) of the buffer.
    pub fn GetConsoleAliasesLengthA(lpExeName: LPCSTR) -> DWORD;

    // GetConsoleAliasExes retrieves the names of all executables with console aliases defined.
    pub fn GetConsoleAliasExesA(lpExeNameBuffer: LPSTR, ExeNameBufferLength: DWORD) -> DWORD;

    // GetConsoleAliasExesLength returns the size (in bytes) of the buffer needed.
    pub fn GetConsoleAliasExesLengthA() -> DWORD;

    // GetConsoleCP retrieves the input code page used by the console.
    pub fn GetConsoleCP() -> UINT;

    // GetConsoleCursorInfo retrieves information about the console cursor.
    pub fn GetConsoleCursorInfo(
        hConsoleOutput: HANDLE,
        lpConsoleCursorInfo: *mut CONSOLE_CURSOR_INFO,
    ) -> BOOL;

    // GetConsoleDisplayMode retrieves the display mode of the current console.
    pub fn GetConsoleDisplayMode(lpModeFlags: *mut DWORD) -> BOOL;

    // GetConsoleFontSize retrieves the size of the font used by the console.
    pub fn GetConsoleFontSize(hConsoleOutput: HANDLE, nFont: DWORD) -> COORD;

    // GetConsoleHistoryInfo retrieves the history settings for the console.
    pub fn GetConsoleHistoryInfo(lpConsoleHistoryInfo: *mut CONSOLE_HISTORY_INFO) -> BOOL;

    // GetConsoleMode retrieves the current input or output mode.
    pub fn GetConsoleMode(hConsoleHandle: HANDLE, lpMode: LPDWORD) -> BOOL;

    // GetConsoleOriginalTitle retrieves the original title for the current console window.
    pub fn GetConsoleOriginalTitleA(lpConsoleTitle: LPSTR, nSize: DWORD) -> DWORD;

    // GetConsoleOutputCP retrieves the output code page used by the console.
    pub fn GetConsoleOutputCP() -> UINT;

    // GetConsoleProcessList retrieves a list of processes attached to the console.
    pub fn GetConsoleProcessList(lpdwProcessList: *mut DWORD, dwProcessCount: DWORD) -> DWORD;

    // GetConsoleScreenBufferInfo retrieves information about the console screen buffer.
    pub fn GetConsoleScreenBufferInfo(
        hConsoleOutput: HANDLE,
        lpConsoleScreenBufferInfo: *mut CONSOLE_SCREEN_BUFFER_INFO,
    ) -> BOOL;

    // GetConsoleScreenBufferInfoEx retrieves extended information about the console screen buffer.
    pub fn GetConsoleScreenBufferInfoEx(
        hConsoleOutput: HANDLE,
        lpConsoleScreenBufferInfoEx: *mut CONSOLE_SCREEN_BUFFER_INFOEX,
    ) -> BOOL;

    // GetConsoleSelectionInfo retrieves information about the current console selection.
    pub fn GetConsoleSelectionInfo(lpConsoleSelectionInfo: *mut CONSOLE_SELECTION_INFO) -> BOOL;

    // GetConsoleTitle retrieves the title for the current console window.
    pub fn GetConsoleTitleA(lpConsoleTitle: LPSTR, nSize: DWORD) -> DWORD;

    // GetConsoleWindow retrieves the window handle of the console.
    pub fn GetConsoleWindow() -> HANDLE;

    // GetCurrentConsoleFont retrieves information about the current console font.
    pub fn GetCurrentConsoleFont(
        hConsoleOutput: HANDLE,
        bMaximumWindow: BOOL,
        lpConsoleCurrentFont: *mut CONSOLE_FONT_INFO,
    ) -> BOOL;

    // GetCurrentConsoleFontEx retrieves extended information about the current console font.
    pub fn GetCurrentConsoleFontEx(
        hConsoleOutput: HANDLE,
        bMaximumWindow: BOOL,
        lpConsoleCurrentFontEx: *mut CONSOLE_FONT_INFOEX,
    ) -> BOOL;

    // GetLargestConsoleWindowSize retrieves the size of the largest possible console window.
    pub fn GetLargestConsoleWindowSize(hConsoleOutput: HANDLE) -> COORD;

    // GetNumberOfConsoleInputEvents retrieves the number of unread input records.
    pub fn GetNumberOfConsoleInputEvents(hConsoleInput: HANDLE, lpcNumberOfEvents: LPDWORD)
        -> BOOL;

    // GetNumberOfConsoleMouseButtons retrieves the number of mouse buttons.
    pub fn GetNumberOfConsoleMouseButtons(lpNumberOfMouseButtons: LPDWORD) -> BOOL;

    // GetStdHandle retrieves a handle for the standard device.
    pub fn GetStdHandle(nStdHandle: DWORD) -> HANDLE;

    // HandlerRoutine: Application-defined function type (see PHANDLER_ROUTINE above).

    // PeekConsoleInput reads data from the console input buffer without removing it.
    pub fn PeekConsoleInputA(
        hConsoleInput: HANDLE,
        lpBuffer: *mut INPUT_RECORD,
        nLength: DWORD,
        lpNumberOfEventsRead: LPDWORD,
    ) -> BOOL;

    // ReadConsole reads character input from the console.
    pub fn ReadConsoleA(
        hConsoleInput: HANDLE,
        lpBuffer: *mut (),
        nNumberOfCharsToRead: DWORD,
        lpNumberOfCharsRead: LPDWORD,
        pInputControl: *mut (),
    ) -> BOOL;

    // ReadConsoleInput reads data from the console input buffer.
    pub fn ReadConsoleInputA(
        hConsoleInput: HANDLE,
        lpBuffer: *mut INPUT_RECORD,
        nLength: DWORD,
        lpNumberOfEventsRead: LPDWORD,
    ) -> BOOL;

    // ReadConsoleInputEx reads data from the console input buffer with configurable behavior.
    pub fn ReadConsoleInputExA(
        hConsoleInput: HANDLE,
        lpBuffer: *mut INPUT_RECORD,
        nLength: DWORD,
        lpNumberOfEventsRead: LPDWORD,
        dwFlags: DWORD,
    ) -> BOOL;

    // ReadConsoleOutput reads character and color attribute data from a block in a console screen buffer.
    pub fn ReadConsoleOutputA(
        hConsoleOutput: HANDLE,
        lpBuffer: *mut CHAR_INFO,
        dwBufferSize: COORD,
        dwBufferCoord: COORD,
        lpReadRegion: *mut SMALL_RECT,
    ) -> BOOL;

    // ReadConsoleOutputAttribute copies attributes from consecutive cells.
    pub fn ReadConsoleOutputAttribute(
        hConsoleOutput: HANDLE,
        lpAttribute: *mut WORD,
        nLength: DWORD,
        dwReadCoord: COORD,
        lpNumberOfAttrsRead: LPDWORD,
    ) -> BOOL;

    // ReadConsoleOutputCharacter copies characters from consecutive cells.
    pub fn ReadConsoleOutputCharacterA(
        hConsoleOutput: HANDLE,
        lpCharacter: LPSTR,
        nLength: DWORD,
        dwReadCoord: COORD,
        lpNumberOfCharsRead: LPDWORD,
    ) -> BOOL;

    // ResizePseudoConsole resizes the pseudoconsole internal buffers.
    pub fn ResizePseudoConsole(hPC: HPCON, size: COORD) -> HRESULT;

    // ScrollConsoleScreenBuffer moves a block of data in a screen buffer.
    pub fn ScrollConsoleScreenBufferA(
        hConsoleOutput: HANDLE,
        lpScrollRect: *const SMALL_RECT,
        lpClipRect: *const SMALL_RECT,
        dwDestinationOrigin: COORD,
        lpFill: *const CHAR_INFO,
    ) -> BOOL;

    // SetConsoleActiveScreenBuffer sets the active screen buffer.
    pub fn SetConsoleActiveScreenBuffer(hConsoleOutput: HANDLE) -> BOOL;

    // SetConsoleCP sets the input code page.
    pub fn SetConsoleCP(wCodePageID: UINT) -> BOOL;

    // SetConsoleCtrlHandler adds or removes an application-defined handler.
    pub fn SetConsoleCtrlHandler(HandlerRoutine: PHANDLER_ROUTINE, Add: BOOL) -> BOOL;

    // SetConsoleCursorInfo sets the cursor size and visibility.
    pub fn SetConsoleCursorInfo(
        hConsoleOutput: HANDLE,
        lpConsoleCursorInfo: *const CONSOLE_CURSOR_INFO,
    ) -> BOOL;

    // SetConsoleCursorPosition sets the cursor position.
    pub fn SetConsoleCursorPosition(hConsoleOutput: HANDLE, dwCursorPosition: COORD) -> BOOL;

    // SetConsoleDisplayMode sets the display mode of the console.
    pub fn SetConsoleDisplayMode(
        hConsoleOutput: HANDLE,
        dwFlags: DWORD,
        lpNewScreenBufferDimensions: *mut COORD,
    ) -> BOOL;

    // SetConsoleHistoryInfo sets the history settings.
    pub fn SetConsoleHistoryInfo(lpConsoleHistoryInfo: *const CONSOLE_HISTORY_INFO) -> BOOL;

    // SetConsoleMode sets the console input or output mode.
    pub fn SetConsoleMode(hConsoleHandle: HANDLE, dwMode: DWORD) -> BOOL;

    // SetConsoleOutputCP sets the output code page.
    pub fn SetConsoleOutputCP(wCodePageID: UINT) -> BOOL;

    // SetConsoleScreenBufferInfoEx sets extended info about the screen buffer.
    pub fn SetConsoleScreenBufferInfoEx(
        hConsoleOutput: HANDLE,
        lpConsoleScreenBufferInfoEx: *const CONSOLE_SCREEN_BUFFER_INFOEX,
    ) -> BOOL;

    // SetConsoleScreenBufferSize changes the size of the screen buffer.
    pub fn SetConsoleScreenBufferSize(hConsoleOutput: HANDLE, dwSize: COORD) -> BOOL;

    // SetConsoleTextAttribute sets text and background color attributes.
    pub fn SetConsoleTextAttribute(hConsoleOutput: HANDLE, wAttributes: WORD) -> BOOL;

    // SetConsoleTitle sets the title for the console window.
    pub fn SetConsoleTitleA(lpConsoleTitle: LPCSTR) -> BOOL;

    // SetConsoleWindowInfo sets the size and position of the console window.
    pub fn SetConsoleWindowInfo(
        hConsoleOutput: HANDLE,
        bAbsolute: BOOL,
        lpConsoleWindow: *const SMALL_RECT,
    ) -> BOOL;

    // SetCurrentConsoleFontEx sets extended current console font information.
    pub fn SetCurrentConsoleFontEx(
        hConsoleOutput: HANDLE,
        bMaximumWindow: BOOL,
        lpConsoleCurrentFontEx: *const CONSOLE_FONT_INFOEX,
    ) -> BOOL;

    // SetStdHandle sets the handle for a standard device.
    pub fn SetStdHandle(nStdHandle: DWORD, hHandle: HANDLE) -> BOOL;

    // WriteConsole writes a string to the console.
    pub fn WriteConsoleA(
        hConsoleOutput: HANDLE,
        lpBuffer: *const (),
        nNumberOfCharsToWrite: DWORD,
        lpNumberOfCharsWritten: LPDWORD,
        lpReserved: *mut (),
    ) -> BOOL;

    // WriteConsoleInput writes data directly to the console input buffer.
    pub fn WriteConsoleInputA(
        hConsoleInput: HANDLE,
        lpBuffer: *const INPUT_RECORD,
        nLength: DWORD,
        lpNumberOfEventsWritten: LPDWORD,
    ) -> BOOL;

    // WriteConsoleOutput writes character and attribute data to a console screen buffer.
    pub fn WriteConsoleOutputA(
        hConsoleOutput: HANDLE,
        lpBuffer: *const CHAR_INFO,
        dwBufferSize: COORD,
        dwBufferCoord: COORD,
        lpWriteRegion: *mut SMALL_RECT,
    ) -> BOOL;

    // WriteConsoleOutputAttribute writes attributes to consecutive cells.
    pub fn WriteConsoleOutputAttribute(
        hConsoleOutput: HANDLE,
        lpAttribute: *const WORD,
        nLength: DWORD,
        dwWriteCoord: COORD,
        lpNumberOfAttrsWritten: LPDWORD,
    ) -> BOOL;

    // WriteConsoleOutputCharacter writes characters to consecutive cells.
    pub fn WriteConsoleOutputCharacterA(
        hConsoleOutput: HANDLE,
        lpCharacter: LPCSTR,
        nLength: DWORD,
        dwWriteCoord: COORD,
        lpNumberOfCharsWritten: LPDWORD,
    ) -> BOOL;
}
