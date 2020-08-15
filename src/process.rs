
type Pid = u32;

pub struct Process<'a> {
    process_id: Pid,
    name: &'a str,

    stack_pointer: &'a u8,
    //priority: u32,
}