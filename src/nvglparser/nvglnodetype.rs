use serde::Serialize;

#[derive(Debug,Serialize,Clone)]
pub enum Node {
    // literal type
    Number(NumberNode),
    Bool(BoolNode),
    String(StringNode),
    Array(ArrayNode),
    Object(ObjectNode),
    Function(FunctionNode),
    // identifier
    Id(IdNode),
    Key1(KeyNode),
    Key2(KeyNode),
    // operation
    Opr(OprNode),
    UOpr(UOprNode),
    // FuncCall
    FuncCall(FuncCallNode),
    NewFuncCall(NewFuncCallNode),
    // statement
    Stat(StatNode),
    ReturnStat(ReturnStatNode),
    AStat(AStatNode),
    MLTAStat(MLTAStatNode),
    PMLTAStat(PMLTAStatNode),
    Return(),
    Break(),
    Continue(),
    // rootobj
    // Includes(IncludesNode),
    Includes(IncludesNode),
    IncludesBlock(IncludesBlockNode),
    Imports(ImportsNode),
    ImportsBlock(ImportsBlockNode),
    // Init(InitNode),
    // Item(ItemNode),
    Obj(ObjNode),
    TLObj(TLObjNode),
    Init(InitNode),
    Item(ItemNode),
    TLObjStat(TLObjStatNode),
    Block(BlockNode),
    ObjFunc(ObjFuncNode),
    ObjConf(ObjConfNode),
    ObjFrame(ObjFrameNode),
    ObjConfGElm(ObjConfGElmNode),
    ObjConfRElm(ObjConfRElmNode),
    // TimeLine(TimeLineNode),
    // structure
    If(IfNode),
    While(WhileNode),
    Times(TimesNode),
    TimesAs(TimesAsNode),
    //
    Scope(),
}

#[derive(Debug,Serialize,Clone)]
pub struct NumberNode {
    pub val: f64,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct BoolNode {
    pub val: bool,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct StringNode {
    pub val: String,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct ArrayNode {
    pub val: Vec<Node>,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct ObjectElmNode {
    pub key: Node,
    pub val: Node,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct ObjectNode {
    pub val: Vec<ObjectElmNode>,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct FunctionNode {
    pub args: Vec<Node>,
    pub val: Box<Node>,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct IdNode {
    pub val: String,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct KeyNode {
    pub l: Box<Node>,
    pub r: Box<Node>,
    pub pos: NodePos,
}

#[derive(Debug,Serialize,Clone)]
pub struct OprNode {
    pub opr: String,
    pub l: Box<Node>,
    pub r: Box<Node>,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct UOprNode {
    pub opr: String,
    pub r: Box<Node>,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct FuncCallNode {
    pub func: Box<Node>,
    pub args: Vec<Node>,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct NewFuncCallNode {
    pub func: Box<Node>,
    pub args: Vec<Node>,
    pub pos: NodePos,
}

#[derive(Debug,Serialize,Clone)]
pub struct ExprNode {
    pub expr: Box<Node>,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct StatNode {
    pub expr: Box<Node>,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct ReturnStatNode {
    pub expr: Box<Node>,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct AStatNode {
    pub loc: Box<Node>,
    pub expr: Box<Node>,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct MLTAStatNode {
    pub loc: Box<Node>,
    pub val: String,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct PMLTAStatNode {
    pub loc: Box<Node>,
    pub val: String,
    pub expr: Box<Node>,
    pub pos: NodePos,
}

#[derive(Debug,Serialize,Clone)]
pub struct ObjNode {
    pub name: Box<Node>,
    pub val: Vec<Node>,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct ObjFuncNode {
    pub name: String,
    pub val: Box<Node>,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct ObjConfNode {
    pub val: Vec<Node>,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct ObjFrameNode {
    pub arg: String,
    pub val: Box<Node>,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct ObjConfGElmNode {
    pub name: String,
    pub valtype: String,
    pub val: Box<Node>,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct ObjConfRElmNode {
    pub name: String,
    pub val: Vec<Node>,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct InitNode {
    pub val: Box<Node>,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct ItemNode {
    pub name: Box<Node>,
    pub val: Box<Node>,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct BlockNode {
    pub stats: Vec<Node>,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct TLObjNode {
    pub val: Vec<Node>,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct TLObjStatNode {
    pub objname: Box<Node>,
    pub args: Vec<ObjectElmNode>,
    pub pos: NodePos,
}

#[derive(Debug,Serialize,Clone)]
pub struct IncludesNode {
    pub val: Box<Node>,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct IncludesElmNode {
    pub module: Node,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct IncludesBlockNode {
    pub val: Vec<IncludesElmNode>,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct ImportsNode {
    pub val: Box<Node>,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct ImportsElmNode {
    pub module: Node,
    pub name: Box<Node>,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct ImportsBlockNode {
    pub val: Vec<ImportsElmNode>,
    pub pos: NodePos,
}


#[derive(Debug,Serialize,Clone)]
pub struct IfElmNode {
    pub cond: Node,
    pub block: Node,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct IfNode {
    pub val: Vec<IfElmNode>,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct WhileNode {
    pub cond: Box<Node>,
    pub block: Box<Node>,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct TimesNode {
    pub num: Box<Node>,
    pub block: Box<Node>,
    pub pos: NodePos,
}
#[derive(Debug,Serialize,Clone)]
pub struct TimesAsNode {
    pub loc: Box<Node>,
    pub num: Box<Node>,
    pub block: Box<Node>,
    pub pos: NodePos,
}





#[derive(Debug,Serialize,Clone)]
pub struct NodePos {
    pub start: usize,
    pub end: usize,
}