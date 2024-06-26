var mermaid;
var code = "";

async function loadmodule(url) {
    const module = await import(url);
    mermaid = module.default;
    mermaid.initialize({
        securityLevel: "loose",
    });
}


var dec_id = 0;
function ScopeMarker(start,end) {
    input.deltaDecorations(dec_id,[])
    const decorations = []
    console.log(getLineAndCol(start),getLineAndCol(end))
    decorations.push({range: new monaco.Range(...getLineAndCol(start),...getLineAndCol(end)),options:{inlineClassName: 'nvgl_scope'}})
    dec_id = input.deltaDecorations([],decorations)
}

function getLineAndCol(i) {
    let j = 0;
    let line = 1;
    let col = 0;
    while (j<=i) {
        if (code[j]=="\n") {
            line++;
            col = 0;
        }
        else {
            col++;
        }
        j++;
    }
    return [line,col];
}

function drawGraphView(obj,_code,graphview) {
    code = _code;
    graphview.removeAttribute('data-processed');
    graphview.innerHTML = "";
    const graphDefinition = {main:`%%{init:{'theme':'dark'}}%%\ngraph TD\n!(root)\nclick ! call ScopeMarker(0,${code.length})\n`};
    for (let j in obj) {
        addGraphObj(graphDefinition,obj[j],"!",`${j}_`);
    }
    graphview.innerHTML = graphDefinition.main;
    mermaid.init();
}
function addGraphObj(txt,obj,name,i) {
    if (obj==null) {return}
    const keys = Object.keys(obj);
    if (keys.length!=1) {console.warn("Invalid AST");}
    const key = keys[0];
    const val = obj[key];
    switch (key) {
        case "String":
        case "Number":
        case "Id":
            txt.main += `${name} --${key}--> ${name}${i}(${val.val})\n`;
            break;
        case "Scope":
            txt.main += `${name} --> ${name}${i}(Scope)\n`;
            break;
        case "Var":
            txt.main += `${name} --var--> ${name}${i}(${val})\n`;
            break;
        case "UOpr":
            txt.main += `${name} --> ${name}${i}["${val.opr}""]\n`;
            addGraphObj(txt,val.r,`${name}${i}`,"r");
            break;
        case "Key":
            txt.main += `${name} --> ${name}${i}["key"]\n`;
            addGraphObj(txt,val.l,`${name}${i}`,"l");
            addGraphObj(txt,val.r,`${name}${i}`,"r");
            break;
        case "Opr":
            txt.main += `${name} --> ${name}${i}["${val.opr}"]\n`;
            addGraphObj(txt,val.l,`${name}${i}`,"l");
            addGraphObj(txt,val.r,`${name}${i}`,"r");
            break;
        case "Stat":
            txt.main += `${name} --> ${name}${i}["Stat"]\n`;
            addGraphObj(txt,val.expr,`${name}${i}`,"e");
            break;
        case "AStat":
            txt.main += `${name} --> ${name}${i}["AStat"]\n`;
            addGraphObj(txt,val.loc,`${name}${i}`,"l");
            addGraphObj(txt,val.expr,`${name}${i}`,"e");
            break;
        case "ReturnStat":
            txt.main += `${name} --> ${name}${i}["ReturnStat"]\n`;
            addGraphObj(txt,val.expr,`${name}${i}`,"e");
            break;
        case "MLTAStat":
            txt.main += `${name} --> ${name}${i}["MLTAStat"]\n`;
            addGraphObj(txt,val.loc,`${name}${i}`,"l");
            txt.main += `${name}${i} --val--> ${name}${i}v(${val.val})\n`;
            break;
        case "Block":
            for (let j in val.stats) {
                addGraphObj(txt,val.stats[j],`${name}`,`${j}_`);
            }
            break;
        case "If":
            txt.main += `${name} --> ${name}${i}["If"]\n`;
            for (let j in val.val) {
                txt.main += `${name}${i} --> ${name}${i}${j}_["IfElm ${Number(j)+1}"]\n`;
                addGraphObj(txt,val.val[j].cond,`${name}${i}${j}_`,`l`);
                addGraphObj(txt,val.val[j].block,`${name}${i}${j}_`,`r`);
            }
            break;
        case "Includes":
            txt.main += `${name} --> ${name}${i}["Includes"]\n`;
            addGraphObj(txt,val.val,`${name}${i}`,"b");
            break;
        case "IncludesBlock":
            for (let j in val.val) {
                txt.main += `${name} --> ${name}${i}${j}_["Include ${Number(j)+1}"]\n`;
                addGraphObj(txt,val.val[j].module,`${name}${i}${j}_`,`${j}l_`);
                addGraphObj(txt,val.val[j].name,`${name}${i}${j}_`,`${j}r_`);
            }
            break;
        case "Imports":
            txt.main += `${name} --> ${name}${i}["Imports"]\n`;
            addGraphObj(txt,val.val,`${name}${i}`,"b");
            break;
        case "ImportsBlock":
            for (let j in val.val) {
                txt.main += `${name} --> ${name}${i}${j}_["Import ${Number(j)+1}"]\n`;
                addGraphObj(txt,val.val[j].module,`${name}${i}${j}_`,`${j}l_`);
                addGraphObj(txt,val.val[j].name,`${name}${i}${j}_`,`${j}r_`);
            }
            break;
        case "Init":
            txt.main += `${name} --> ${name}${i}["Init()"]\n`;
            addGraphObj(txt,val.val,`${name}${i}`,"i");
            break;
        case "Item":
            txt.main += `${name} --> ${name}${i}["Item()"]\n`;
            txt.main += `${name}${i} --> ${name}${i}l["name"]\n`;
            addGraphObj(txt,val.name,`${name}${i}l`,"i");
            txt.main += `${name}${i} --> ${name}${i}r["block"]\n`;
            addGraphObj(txt,val.val,`${name}${i}r`,"i");
            break;
        case "TLObj":
            txt.main += `${name} --> ${name}${i}["TimeLine"]\n`;
            for (let j in val.val) {
                addGraphObj(txt,val.val[j],`${name}${i}`,`${j}_`);
            }
            break;
        case "TLObjStat":
            txt.main += `${name} --> ${name}${i}[["Object()"]]\n`;
            addGraphObj(txt,val.objname,`${name}${i}`,"l");
            txt.main += `${name}${i} --> ${name}${i}a["Args"]\n`;
            for (let j in val.args) {
                txt.main += `${name}${i}a --> ${name}${i}a${j}_["arg ${(Number(j)+1)}"]\n`;
                addGraphObj(txt,val.args[j].key,`${name}${i}a${j}_`,`l`);
                addGraphObj(txt,val.args[j].val,`${name}${i}a${j}_`,`r`);
            }
            break;
        case "FuncCall":
            txt.main += `${name} --> ${name}${i}["${key}"]\n`;
            addGraphObj(txt,val.func,`${name}${i}`,"l");
            txt.main += `${name}${i} --> ${name}${i}a["Args"]\n`;
            for (let j in val.args) {
                addGraphObj(txt,val.args[j],`${name}${i}a`,`${j}_r`);
            }
            break;
        case "Function":
            txt.main += `${name} --> ${name}${i}["${key}"]\n`;
            txt.main += `${name}${i} --> ${name}${i}a["Args"]\n`;
            for (let j in val.args) {
                addGraphObj(txt,val.args[j],`${name}${i}a`,`${j}_r`);
            }
            addGraphObj(txt,val.val,`${name}${i}`,"l");
            break;
    }
    if (val.pos!=null) {
        txt.main += `click ${name}${i} call ScopeMarker(${val.pos.start},${val.pos.end})\n`;
    }
}


export {loadmodule as init, ScopeMarker};
export default drawGraphView;