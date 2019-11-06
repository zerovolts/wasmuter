#[derive(Clone, Copy)]
pub struct TypeIndex(pub u32);

#[derive(Clone, Copy)]
pub struct FunctionIndex(pub u32);

#[derive(Clone, Copy)]
pub struct TableIndex(pub u32);

#[derive(Clone, Copy)]
pub struct MemoryIndex(pub u32);

#[derive(Clone, Copy)]
pub struct GlobalIndex(pub u32);

#[derive(Clone, Copy)]
pub struct LocalIndex(pub u32);

#[derive(Clone, Copy)]
pub struct LabelIndex(pub u32);
