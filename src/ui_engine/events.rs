#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UIEvent{
    None,
    Init,

    Repaint,
    PointerDown{x: u32, y: u32},
    PointerMove{x: u32, y: u32},
    PointerUp{x: u32, y: u32},
    AsyncTaskFinished{task_id: u32, task_sub_id: u32},


}
