use std::{cell::RefCell, fmt::Display};

struct Context {
    stack: Vec<(*const u8, Action)>,
    payload: Vec<u8>,
    base_ptr: *const u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Action {
    Advance,
    AdvanceSlice,
    AdvanceU32,
    AdvanceU64,
    AdvanceS32,
    AdvanceS64,
    AdvanceF32,
    AdvanceF64,
    AdvanceVector,
    AdvanceName,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Action::Advance => "const",
            Action::AdvanceSlice => "slice",
            Action::AdvanceU32 => "u32",
            Action::AdvanceU64 => "u64",
            Action::AdvanceS32 => "s32",
            Action::AdvanceS64 => "s64",
            Action::AdvanceF32 => "f32",
            Action::AdvanceF64 => "f64",
            Action::AdvanceVector => "vec",
            Action::AdvanceName => "name",
        };
        write!(f, "{s}")
    }
}

thread_local! {
    static CONTEXT: RefCell<Option<Context>> = RefCell::new(None);
}

pub fn initialize(bytes: &[u8]) {
    eprint!("payload:");
    for b in bytes {
        eprint!(" {b:02X}");
    }
    eprintln!();

    let mut payload = Vec::new();
    payload.extend_from_slice(bytes);

    CONTEXT.with(|ctx| {
        *ctx.borrow_mut() = Some(Context {
            stack: Vec::new(),
            payload,
            base_ptr: bytes.as_ptr(),
        })
    })
}

pub fn trace_start(action: Action, bytes: &[u8]) {
    CONTEXT.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        let ctx = ctx.as_mut().expect("context not initialized");
        ctx.stack.push((bytes.as_ptr(), action));
    })
}

pub fn trace_end(action: Action, bytes: &[u8]) {
    CONTEXT.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        let ctx = ctx.as_mut().expect("context not initialized");
        let (start_ptr, want_action) = ctx.stack.pop().expect("empty actions stack");
        assert_eq!(
            want_action, action,
            "popped action does not equal to the expected action"
        );

        for (_, action) in &ctx.stack {
            eprint!("{action} > ");
        }
        eprint!("{action} >");
        let start = start_ptr as usize - ctx.base_ptr as usize;
        let end = bytes.as_ptr() as usize - ctx.base_ptr as usize;
        for b in &ctx.payload[start..end] {
            eprint!(" {b:02X}");
        }

        eprintln!();
    })
}
