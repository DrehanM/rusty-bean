
enum ThreadStatus {
    Running,
    Ready,
    Blocked,
    Dying,
}

pub struct Thread {
    id: i32,
    status: ThreadStatus,
    //name: str[16],

    //stack: &'a u8,

    //stack_pointer: &'a u8,
    priority: u32,
}

// impl Thread {
//     pub fn new(name: ) -> Thread {

//     }
// }