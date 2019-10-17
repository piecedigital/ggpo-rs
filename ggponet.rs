/* automatically generated by rust-bindgen */

pub const __GNUC_VA_LIST: u32 = 1;
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const __bool_true_false_are_defined: u32 = 1;
pub const GGPO_MAX_PLAYERS: u32 = 4;
pub const GGPO_MAX_PREDICTION_FRAMES: u32 = 8;
pub const GGPO_MAX_SPECTATORS: u32 = 32;
pub const GGPO_SPECTATOR_INPUT_INTERVAL: u32 = 4;
pub const GGPO_INVALID_HANDLE: i32 = -1;
pub type va_list = __builtin_va_list;
pub type __gnuc_va_list = __builtin_va_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GGPOSession {
    _unused: [u8; 0],
}
pub type GGPOPlayerHandle = ::std::os::raw::c_int;
pub const GGPOPlayerType_GGPO_PLAYERTYPE_LOCAL: GGPOPlayerType = 0;
pub const GGPOPlayerType_GGPO_PLAYERTYPE_REMOTE: GGPOPlayerType = 1;
pub const GGPOPlayerType_GGPO_PLAYERTYPE_SPECTATOR: GGPOPlayerType = 2;
pub type GGPOPlayerType = i32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GGPOPlayer {
    pub size: ::std::os::raw::c_int,
    pub type_: GGPOPlayerType,
    pub player_num: ::std::os::raw::c_int,
    pub u: GGPOPlayer__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union GGPOPlayer__bindgen_ty_1 {
    pub local: GGPOPlayer__bindgen_ty_1__bindgen_ty_1,
    pub remote: GGPOPlayer__bindgen_ty_1__bindgen_ty_2,
    _bindgen_union_align: [u16; 17usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GGPOPlayer__bindgen_ty_1__bindgen_ty_1 {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_GGPOPlayer__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<GGPOPlayer__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Size of: ",
            stringify!(GGPOPlayer__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<GGPOPlayer__bindgen_ty_1__bindgen_ty_1>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(GGPOPlayer__bindgen_ty_1__bindgen_ty_1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GGPOPlayer__bindgen_ty_1__bindgen_ty_2 {
    pub ip_address: [::std::os::raw::c_char; 32usize],
    pub port: ::std::os::raw::c_short,
}
#[test]
fn bindgen_test_layout_GGPOPlayer__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<GGPOPlayer__bindgen_ty_1__bindgen_ty_2>(),
        34usize,
        concat!(
            "Size of: ",
            stringify!(GGPOPlayer__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<GGPOPlayer__bindgen_ty_1__bindgen_ty_2>(),
        2usize,
        concat!(
            "Alignment of ",
            stringify!(GGPOPlayer__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPOPlayer__bindgen_ty_1__bindgen_ty_2>())).ip_address
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOPlayer__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(ip_address)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPOPlayer__bindgen_ty_1__bindgen_ty_2>())).port as *const _
                as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOPlayer__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(port)
        )
    );
}
#[test]
fn bindgen_test_layout_GGPOPlayer__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<GGPOPlayer__bindgen_ty_1>(),
        34usize,
        concat!("Size of: ", stringify!(GGPOPlayer__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<GGPOPlayer__bindgen_ty_1>(),
        2usize,
        concat!("Alignment of ", stringify!(GGPOPlayer__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GGPOPlayer__bindgen_ty_1>())).local as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOPlayer__bindgen_ty_1),
            "::",
            stringify!(local)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GGPOPlayer__bindgen_ty_1>())).remote as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOPlayer__bindgen_ty_1),
            "::",
            stringify!(remote)
        )
    );
}
#[test]
fn bindgen_test_layout_GGPOPlayer() {
    assert_eq!(
        ::std::mem::size_of::<GGPOPlayer>(),
        48usize,
        concat!("Size of: ", stringify!(GGPOPlayer))
    );
    assert_eq!(
        ::std::mem::align_of::<GGPOPlayer>(),
        4usize,
        concat!("Alignment of ", stringify!(GGPOPlayer))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GGPOPlayer>())).size as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOPlayer),
            "::",
            stringify!(size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GGPOPlayer>())).type_ as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOPlayer),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GGPOPlayer>())).player_num as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOPlayer),
            "::",
            stringify!(player_num)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GGPOPlayer>())).u as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOPlayer),
            "::",
            stringify!(u)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GGPOLocalEndpoint {
    pub player_num: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_GGPOLocalEndpoint() {
    assert_eq!(
        ::std::mem::size_of::<GGPOLocalEndpoint>(),
        4usize,
        concat!("Size of: ", stringify!(GGPOLocalEndpoint))
    );
    assert_eq!(
        ::std::mem::align_of::<GGPOLocalEndpoint>(),
        4usize,
        concat!("Alignment of ", stringify!(GGPOLocalEndpoint))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GGPOLocalEndpoint>())).player_num as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOLocalEndpoint),
            "::",
            stringify!(player_num)
        )
    );
}
pub const GGPOErrorCode_GGPO_OK: GGPOErrorCode = 0;
pub const GGPOErrorCode_GGPO_ERRORCODE_SUCCESS: GGPOErrorCode = 0;
pub const GGPOErrorCode_GGPO_ERRORCODE_GENERAL_FAILURE: GGPOErrorCode = -1;
pub const GGPOErrorCode_GGPO_ERRORCODE_INVALID_SESSION: GGPOErrorCode = 1;
pub const GGPOErrorCode_GGPO_ERRORCODE_INVALID_PLAYER_HANDLE: GGPOErrorCode = 2;
pub const GGPOErrorCode_GGPO_ERRORCODE_PLAYER_OUT_OF_RANGE: GGPOErrorCode = 3;
pub const GGPOErrorCode_GGPO_ERRORCODE_PREDICTION_THRESHOLD: GGPOErrorCode = 4;
pub const GGPOErrorCode_GGPO_ERRORCODE_UNSUPPORTED: GGPOErrorCode = 5;
pub const GGPOErrorCode_GGPO_ERRORCODE_NOT_SYNCHRONIZED: GGPOErrorCode = 6;
pub const GGPOErrorCode_GGPO_ERRORCODE_IN_ROLLBACK: GGPOErrorCode = 7;
pub const GGPOErrorCode_GGPO_ERRORCODE_INPUT_DROPPED: GGPOErrorCode = 8;
pub const GGPOErrorCode_GGPO_ERRORCODE_PLAYER_DISCONNECTED: GGPOErrorCode = 9;
pub const GGPOErrorCode_GGPO_ERRORCODE_TOO_MANY_SPECTATORS: GGPOErrorCode = 10;
pub const GGPOErrorCode_GGPO_ERRORCODE_INVALID_REQUEST: GGPOErrorCode = 11;
pub type GGPOErrorCode = i32;
pub const GGPOEventCode_GGPO_EVENTCODE_CONNECTED_TO_PEER: GGPOEventCode = 1000;
pub const GGPOEventCode_GGPO_EVENTCODE_SYNCHRONIZING_WITH_PEER: GGPOEventCode = 1001;
pub const GGPOEventCode_GGPO_EVENTCODE_SYNCHRONIZED_WITH_PEER: GGPOEventCode = 1002;
pub const GGPOEventCode_GGPO_EVENTCODE_RUNNING: GGPOEventCode = 1003;
pub const GGPOEventCode_GGPO_EVENTCODE_DISCONNECTED_FROM_PEER: GGPOEventCode = 1004;
pub const GGPOEventCode_GGPO_EVENTCODE_TIMESYNC: GGPOEventCode = 1005;
pub const GGPOEventCode_GGPO_EVENTCODE_CONNECTION_INTERRUPTED: GGPOEventCode = 1006;
pub const GGPOEventCode_GGPO_EVENTCODE_CONNECTION_RESUMED: GGPOEventCode = 1007;
pub type GGPOEventCode = i32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GGPOEvent {
    pub code: GGPOEventCode,
    pub u: GGPOEvent__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union GGPOEvent__bindgen_ty_1 {
    pub connected: GGPOEvent__bindgen_ty_1__bindgen_ty_1,
    pub synchronizing: GGPOEvent__bindgen_ty_1__bindgen_ty_2,
    pub synchronized: GGPOEvent__bindgen_ty_1__bindgen_ty_3,
    pub disconnected: GGPOEvent__bindgen_ty_1__bindgen_ty_4,
    pub timesync: GGPOEvent__bindgen_ty_1__bindgen_ty_5,
    pub connection_interrupted: GGPOEvent__bindgen_ty_1__bindgen_ty_6,
    pub connection_resumed: GGPOEvent__bindgen_ty_1__bindgen_ty_7,
    _bindgen_union_align: [u32; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GGPOEvent__bindgen_ty_1__bindgen_ty_1 {
    pub player: GGPOPlayerHandle,
}
#[test]
fn bindgen_test_layout_GGPOEvent__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<GGPOEvent__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Size of: ",
            stringify!(GGPOEvent__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<GGPOEvent__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(GGPOEvent__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPOEvent__bindgen_ty_1__bindgen_ty_1>())).player as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOEvent__bindgen_ty_1__bindgen_ty_1),
            "::",
            stringify!(player)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GGPOEvent__bindgen_ty_1__bindgen_ty_2 {
    pub player: GGPOPlayerHandle,
    pub count: ::std::os::raw::c_int,
    pub total: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_GGPOEvent__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<GGPOEvent__bindgen_ty_1__bindgen_ty_2>(),
        12usize,
        concat!(
            "Size of: ",
            stringify!(GGPOEvent__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<GGPOEvent__bindgen_ty_1__bindgen_ty_2>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(GGPOEvent__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPOEvent__bindgen_ty_1__bindgen_ty_2>())).player as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOEvent__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(player)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPOEvent__bindgen_ty_1__bindgen_ty_2>())).count as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOEvent__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(count)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPOEvent__bindgen_ty_1__bindgen_ty_2>())).total as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOEvent__bindgen_ty_1__bindgen_ty_2),
            "::",
            stringify!(total)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GGPOEvent__bindgen_ty_1__bindgen_ty_3 {
    pub player: GGPOPlayerHandle,
}
#[test]
fn bindgen_test_layout_GGPOEvent__bindgen_ty_1__bindgen_ty_3() {
    assert_eq!(
        ::std::mem::size_of::<GGPOEvent__bindgen_ty_1__bindgen_ty_3>(),
        4usize,
        concat!(
            "Size of: ",
            stringify!(GGPOEvent__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<GGPOEvent__bindgen_ty_1__bindgen_ty_3>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(GGPOEvent__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPOEvent__bindgen_ty_1__bindgen_ty_3>())).player as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOEvent__bindgen_ty_1__bindgen_ty_3),
            "::",
            stringify!(player)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GGPOEvent__bindgen_ty_1__bindgen_ty_4 {
    pub player: GGPOPlayerHandle,
}
#[test]
fn bindgen_test_layout_GGPOEvent__bindgen_ty_1__bindgen_ty_4() {
    assert_eq!(
        ::std::mem::size_of::<GGPOEvent__bindgen_ty_1__bindgen_ty_4>(),
        4usize,
        concat!(
            "Size of: ",
            stringify!(GGPOEvent__bindgen_ty_1__bindgen_ty_4)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<GGPOEvent__bindgen_ty_1__bindgen_ty_4>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(GGPOEvent__bindgen_ty_1__bindgen_ty_4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPOEvent__bindgen_ty_1__bindgen_ty_4>())).player as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOEvent__bindgen_ty_1__bindgen_ty_4),
            "::",
            stringify!(player)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GGPOEvent__bindgen_ty_1__bindgen_ty_5 {
    pub frames_ahead: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_GGPOEvent__bindgen_ty_1__bindgen_ty_5() {
    assert_eq!(
        ::std::mem::size_of::<GGPOEvent__bindgen_ty_1__bindgen_ty_5>(),
        4usize,
        concat!(
            "Size of: ",
            stringify!(GGPOEvent__bindgen_ty_1__bindgen_ty_5)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<GGPOEvent__bindgen_ty_1__bindgen_ty_5>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(GGPOEvent__bindgen_ty_1__bindgen_ty_5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPOEvent__bindgen_ty_1__bindgen_ty_5>())).frames_ahead
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOEvent__bindgen_ty_1__bindgen_ty_5),
            "::",
            stringify!(frames_ahead)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GGPOEvent__bindgen_ty_1__bindgen_ty_6 {
    pub player: GGPOPlayerHandle,
    pub disconnect_timeout: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_GGPOEvent__bindgen_ty_1__bindgen_ty_6() {
    assert_eq!(
        ::std::mem::size_of::<GGPOEvent__bindgen_ty_1__bindgen_ty_6>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(GGPOEvent__bindgen_ty_1__bindgen_ty_6)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<GGPOEvent__bindgen_ty_1__bindgen_ty_6>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(GGPOEvent__bindgen_ty_1__bindgen_ty_6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPOEvent__bindgen_ty_1__bindgen_ty_6>())).player as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOEvent__bindgen_ty_1__bindgen_ty_6),
            "::",
            stringify!(player)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPOEvent__bindgen_ty_1__bindgen_ty_6>())).disconnect_timeout
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOEvent__bindgen_ty_1__bindgen_ty_6),
            "::",
            stringify!(disconnect_timeout)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GGPOEvent__bindgen_ty_1__bindgen_ty_7 {
    pub player: GGPOPlayerHandle,
}
#[test]
fn bindgen_test_layout_GGPOEvent__bindgen_ty_1__bindgen_ty_7() {
    assert_eq!(
        ::std::mem::size_of::<GGPOEvent__bindgen_ty_1__bindgen_ty_7>(),
        4usize,
        concat!(
            "Size of: ",
            stringify!(GGPOEvent__bindgen_ty_1__bindgen_ty_7)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<GGPOEvent__bindgen_ty_1__bindgen_ty_7>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(GGPOEvent__bindgen_ty_1__bindgen_ty_7)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPOEvent__bindgen_ty_1__bindgen_ty_7>())).player as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOEvent__bindgen_ty_1__bindgen_ty_7),
            "::",
            stringify!(player)
        )
    );
}
#[test]
fn bindgen_test_layout_GGPOEvent__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<GGPOEvent__bindgen_ty_1>(),
        12usize,
        concat!("Size of: ", stringify!(GGPOEvent__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<GGPOEvent__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(GGPOEvent__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPOEvent__bindgen_ty_1>())).connected as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOEvent__bindgen_ty_1),
            "::",
            stringify!(connected)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPOEvent__bindgen_ty_1>())).synchronizing as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOEvent__bindgen_ty_1),
            "::",
            stringify!(synchronizing)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPOEvent__bindgen_ty_1>())).synchronized as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOEvent__bindgen_ty_1),
            "::",
            stringify!(synchronized)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPOEvent__bindgen_ty_1>())).disconnected as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOEvent__bindgen_ty_1),
            "::",
            stringify!(disconnected)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPOEvent__bindgen_ty_1>())).timesync as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOEvent__bindgen_ty_1),
            "::",
            stringify!(timesync)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPOEvent__bindgen_ty_1>())).connection_interrupted as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOEvent__bindgen_ty_1),
            "::",
            stringify!(connection_interrupted)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPOEvent__bindgen_ty_1>())).connection_resumed as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOEvent__bindgen_ty_1),
            "::",
            stringify!(connection_resumed)
        )
    );
}
#[test]
fn bindgen_test_layout_GGPOEvent() {
    assert_eq!(
        ::std::mem::size_of::<GGPOEvent>(),
        16usize,
        concat!("Size of: ", stringify!(GGPOEvent))
    );
    assert_eq!(
        ::std::mem::align_of::<GGPOEvent>(),
        4usize,
        concat!("Alignment of ", stringify!(GGPOEvent))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GGPOEvent>())).code as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOEvent),
            "::",
            stringify!(code)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GGPOEvent>())).u as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOEvent),
            "::",
            stringify!(u)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GGPOSessionCallbacks {
    pub begin_game:
        ::std::option::Option<unsafe extern "C" fn(game: *const ::std::os::raw::c_char) -> bool>,
    pub save_game_state: ::std::option::Option<
        unsafe extern "C" fn(
            buffer: *mut *mut ::std::os::raw::c_uchar,
            len: *mut ::std::os::raw::c_int,
            checksum: *mut ::std::os::raw::c_int,
            frame: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub load_game_state: ::std::option::Option<
        unsafe extern "C" fn(
            buffer: *mut ::std::os::raw::c_uchar,
            len: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub log_game_state: ::std::option::Option<
        unsafe extern "C" fn(
            filename: *mut ::std::os::raw::c_char,
            buffer: *mut ::std::os::raw::c_uchar,
            len: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub free_buffer:
        ::std::option::Option<unsafe extern "C" fn(buffer: *mut ::std::os::raw::c_void)>,
    pub advance_frame:
        ::std::option::Option<unsafe extern "C" fn(flags: ::std::os::raw::c_int) -> bool>,
    pub on_event: ::std::option::Option<unsafe extern "C" fn(info: *mut GGPOEvent) -> bool>,
}
#[test]
fn bindgen_test_layout_GGPOSessionCallbacks() {
    assert_eq!(
        ::std::mem::size_of::<GGPOSessionCallbacks>(),
        56usize,
        concat!("Size of: ", stringify!(GGPOSessionCallbacks))
    );
    assert_eq!(
        ::std::mem::align_of::<GGPOSessionCallbacks>(),
        8usize,
        concat!("Alignment of ", stringify!(GGPOSessionCallbacks))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GGPOSessionCallbacks>())).begin_game as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOSessionCallbacks),
            "::",
            stringify!(begin_game)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPOSessionCallbacks>())).save_game_state as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOSessionCallbacks),
            "::",
            stringify!(save_game_state)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPOSessionCallbacks>())).load_game_state as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOSessionCallbacks),
            "::",
            stringify!(load_game_state)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPOSessionCallbacks>())).log_game_state as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOSessionCallbacks),
            "::",
            stringify!(log_game_state)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPOSessionCallbacks>())).free_buffer as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOSessionCallbacks),
            "::",
            stringify!(free_buffer)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPOSessionCallbacks>())).advance_frame as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOSessionCallbacks),
            "::",
            stringify!(advance_frame)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GGPOSessionCallbacks>())).on_event as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPOSessionCallbacks),
            "::",
            stringify!(on_event)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GGPONetworkStats {
    pub network: GGPONetworkStats__bindgen_ty_1,
    pub timesync: GGPONetworkStats__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GGPONetworkStats__bindgen_ty_1 {
    pub send_queue_len: ::std::os::raw::c_int,
    pub recv_queue_len: ::std::os::raw::c_int,
    pub ping: ::std::os::raw::c_int,
    pub kbps_sent: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_GGPONetworkStats__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<GGPONetworkStats__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(GGPONetworkStats__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<GGPONetworkStats__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(GGPONetworkStats__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPONetworkStats__bindgen_ty_1>())).send_queue_len as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPONetworkStats__bindgen_ty_1),
            "::",
            stringify!(send_queue_len)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPONetworkStats__bindgen_ty_1>())).recv_queue_len as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPONetworkStats__bindgen_ty_1),
            "::",
            stringify!(recv_queue_len)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPONetworkStats__bindgen_ty_1>())).ping as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPONetworkStats__bindgen_ty_1),
            "::",
            stringify!(ping)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPONetworkStats__bindgen_ty_1>())).kbps_sent as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPONetworkStats__bindgen_ty_1),
            "::",
            stringify!(kbps_sent)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GGPONetworkStats__bindgen_ty_2 {
    pub local_frames_behind: ::std::os::raw::c_int,
    pub remote_frames_behind: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_GGPONetworkStats__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<GGPONetworkStats__bindgen_ty_2>(),
        8usize,
        concat!("Size of: ", stringify!(GGPONetworkStats__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<GGPONetworkStats__bindgen_ty_2>(),
        4usize,
        concat!("Alignment of ", stringify!(GGPONetworkStats__bindgen_ty_2))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPONetworkStats__bindgen_ty_2>())).local_frames_behind
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPONetworkStats__bindgen_ty_2),
            "::",
            stringify!(local_frames_behind)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<GGPONetworkStats__bindgen_ty_2>())).remote_frames_behind
                as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPONetworkStats__bindgen_ty_2),
            "::",
            stringify!(remote_frames_behind)
        )
    );
}
#[test]
fn bindgen_test_layout_GGPONetworkStats() {
    assert_eq!(
        ::std::mem::size_of::<GGPONetworkStats>(),
        24usize,
        concat!("Size of: ", stringify!(GGPONetworkStats))
    );
    assert_eq!(
        ::std::mem::align_of::<GGPONetworkStats>(),
        4usize,
        concat!("Alignment of ", stringify!(GGPONetworkStats))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GGPONetworkStats>())).network as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPONetworkStats),
            "::",
            stringify!(network)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<GGPONetworkStats>())).timesync as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(GGPONetworkStats),
            "::",
            stringify!(timesync)
        )
    );
}
extern "C" {
    pub fn ggpo_start_session(
        session: *mut *mut GGPOSession,
        cb: *mut GGPOSessionCallbacks,
        game: *const ::std::os::raw::c_char,
        num_players: ::std::os::raw::c_int,
        input_size: ::std::os::raw::c_int,
        localport: ::std::os::raw::c_int,
    ) -> GGPOErrorCode;
}
extern "C" {
    pub fn ggpo_add_player(
        session: *mut GGPOSession,
        player: *mut GGPOPlayer,
        handle: *mut GGPOPlayerHandle,
    ) -> GGPOErrorCode;
}
extern "C" {
    pub fn ggpo_start_synctest(
        session: *mut *mut GGPOSession,
        cb: *mut GGPOSessionCallbacks,
        game: *mut ::std::os::raw::c_char,
        num_players: ::std::os::raw::c_int,
        input_size: ::std::os::raw::c_int,
        frames: ::std::os::raw::c_int,
    ) -> GGPOErrorCode;
}
extern "C" {
    pub fn ggpo_start_spectating(
        session: *mut *mut GGPOSession,
        cb: *mut GGPOSessionCallbacks,
        game: *const ::std::os::raw::c_char,
        num_players: ::std::os::raw::c_int,
        input_size: ::std::os::raw::c_int,
        local_port: ::std::os::raw::c_int,
        host_ip: *mut ::std::os::raw::c_char,
        host_port: ::std::os::raw::c_int,
    ) -> GGPOErrorCode;
}
extern "C" {
    pub fn ggpo_close_session(arg1: *mut GGPOSession) -> GGPOErrorCode;
}
extern "C" {
    pub fn ggpo_set_frame_delay(
        arg1: *mut GGPOSession,
        player: GGPOPlayerHandle,
        frame_delay: ::std::os::raw::c_int,
    ) -> GGPOErrorCode;
}
extern "C" {
    pub fn ggpo_idle(arg1: *mut GGPOSession, timeout: ::std::os::raw::c_int) -> GGPOErrorCode;
}
extern "C" {
    pub fn ggpo_add_local_input(
        arg1: *mut GGPOSession,
        player: GGPOPlayerHandle,
        values: *mut ::std::os::raw::c_void,
        size: ::std::os::raw::c_int,
    ) -> GGPOErrorCode;
}
extern "C" {
    pub fn ggpo_synchronize_input(
        arg1: *mut GGPOSession,
        values: *mut ::std::os::raw::c_void,
        size: ::std::os::raw::c_int,
        disconnect_flags: *mut ::std::os::raw::c_int,
    ) -> GGPOErrorCode;
}
extern "C" {
    pub fn ggpo_disconnect_player(
        arg1: *mut GGPOSession,
        player: GGPOPlayerHandle,
    ) -> GGPOErrorCode;
}
extern "C" {
    pub fn ggpo_advance_frame(arg1: *mut GGPOSession) -> GGPOErrorCode;
}
extern "C" {
    pub fn ggpo_get_network_stats(
        arg1: *mut GGPOSession,
        player: GGPOPlayerHandle,
        stats: *mut GGPONetworkStats,
    ) -> GGPOErrorCode;
}
extern "C" {
    pub fn ggpo_set_disconnect_timeout(
        arg1: *mut GGPOSession,
        timeout: ::std::os::raw::c_int,
    ) -> GGPOErrorCode;
}
extern "C" {
    pub fn ggpo_set_disconnect_notify_start(
        arg1: *mut GGPOSession,
        timeout: ::std::os::raw::c_int,
    ) -> GGPOErrorCode;
}
extern "C" {
    pub fn ggpo_log(arg1: *mut GGPOSession, fmt: *const ::std::os::raw::c_char, ...);
}
extern "C" {
    pub fn ggpo_logv(arg1: *mut GGPOSession, fmt: *const ::std::os::raw::c_char, args: va_list);
}
pub type __builtin_va_list = *mut ::std::os::raw::c_char;
