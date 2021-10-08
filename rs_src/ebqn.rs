use crate::schema::{Env,V,Vu,Vs,Vr,Vn,Vh,Block,BlockInst,Code,Calleable,Body,A,Ar,set,new_scalar,none_or_clone,ok};
use rustler::{Atom,NifResult};
use rustler::resource::ResourceArc;
use cc_mt::{Cc, Trace, Tracer, collect_cycles};
use log::{debug, trace, error, log_enabled, info, Level};
use std::panic;
use crate::test::{bytecode};

fn ge(env: Env,i: usize) -> Env {
    match i {
        0 => env,
        _ => panic!("ge not implemented for i > 0")
    }
}

fn call(arity: usize,a: Vn,x: Vn, w: Vn) -> Vs {
    match a {
        Some(v) => v.call(arity,x,w),
        _ => panic!("unimplemented call"),
    }
}
fn call1(m: V,f: V) -> Vs {
    match &*m {
        Vu::BlockInst(bl) => {
            assert_eq!(1,bl.typ);
            bl.call_block(vec![Vh::V(m.clone()),none_or_clone(&Some(f))])
        },
        _ => panic!("call1 with invalid type"),
    }
}

fn derv(env: Env,code: &Cc<Code>,block: &Cc<Block>) -> Vs {
    match (block.typ,block.imm) {
        (0,true) => panic!("imm block"),
        (typ,_) => {
            let block_inst = BlockInst::new(env.clone(),code.clone(),typ,(*block).clone(),None);
            let r = Vs::V(Cc::new(Vu::BlockInst(block_inst)));
            r
        },
    }
}

fn list(l: Vec<Vs>) -> Vs {
    let shape = vec![Cc::new(Vu::Scalar(l.len() as f64))];
    let ravel = l.into_iter().map(|e|
        match e {
            Vs::V(v) => v,
            _ => panic!("illegal slot passed to list"),
        }
    ).collect::<Vec<V>>();
    Vs::V(Cc::new(Vu::A(A::new(ravel,shape))))
}
fn listr(l: Vec<Vs>) -> Vs {
    let ravel = l.into_iter().map(|e|
        match e {
            Vs::Slot(env,slot) => Vr::Slot(env,slot),
            _ => panic!("illegal non-slot passed to list"),
        }
    ).collect::<Vec<Vr>>();
    Vs::Ar(Ar::new(ravel))
}
pub fn vm(env: &Env,code: &Cc<Code>,block: &Cc<Block>,mut pos: usize,mut stack: Vec<Vs>) -> Vs {
    debug!("block (typ,imm,body) : ({},{},{:?})",block.typ,block.imm,block.body);
    loop {
        let op = code.bc[pos];pos+=1;
        debug!("dbging (op,pos) : {},{}",op,pos);
        match op {
            0 => {
                let x = code.bc[pos];pos+=1;
                let r = code.objs[x].clone();
                stack.push(Vs::V(r))
            },
            1 => {
                let x = code.bc[pos];pos+=1;
                let r = derv(env.clone(),&code,&code.blocks[x]);
                stack.push(r);
            },
            6 => {
                let _ = stack.pop();
            },
            7 => {
                break match stack.len() {
                    1 => {
                        stack.pop().unwrap()
                    },
                    _ => {
                        panic!("stack overflow")
                    }
                };
            },
            11 => {
                let x = code.bc[pos];pos+=1;
                let hd = stack.len() - x;
                let tl = stack.split_off(hd);
                stack.push(list(tl));
            },
            12 => {
                let x = code.bc[pos];pos+=1;
                let hd = stack.len() - x;
                let tl = stack.split_off(hd);
                stack.push(listr(tl));
            },
            16 => {
                let f = stack.pop().unwrap();
                let x = stack.pop().unwrap();
                let r = call(1,Some(f.to_ref().clone()),Some(x.to_ref().clone()),None);
                stack.push(r);
            },
            17 => {
                let w = stack.pop().unwrap();
                let f = stack.pop().unwrap();
                let x = stack.pop().unwrap();
                let r = call(2,Some(f.to_ref().clone()),Some(x.to_ref().clone()),Some(w.to_ref().clone()));
                stack.push(r);
            },
            26 => {
                let f = stack.pop().unwrap();
                let m = stack.pop().unwrap();
                let r = call1(m.to_ref().clone(),f.to_ref().clone());
                stack.push(r);
            },
            33 => {
                let x = code.bc[pos];pos+=1;
                let w = code.bc[pos];pos+=1;
                debug!("opcode 33 (x,w):({},{})",x,w);
                let t = ge(env.clone(),x);
                stack.push(Vs::Slot(t,w))
            },
            34 => {
                let x = code.bc[pos];pos+=1;
                let w = code.bc[pos];pos+=1;
                debug!("opcode 34 (x,w):({},{})",x,w);
                let t = ge(env.clone(),x);
                stack.push(Vs::V(t.get(w)))
            },
            // combine 48 & 49 for now
            48|49 => {
                let i = stack.pop().unwrap();
                let v = stack.pop().unwrap();
                let r = set(true,i,v); // rtns a reference to v
                stack.push(Vs::V(r));
            },
            _ => {
                panic!("unreachable op: {}",op);
            }
        }
    }
}

#[rustler::nif]
fn tests() -> NifResult<Atom> {
    bytecode();
    Ok(ok())
}

pub fn run(code: Cc<Code>) -> f64 {
    let root = Env::new(None,&code.blocks[0],0,None);
    let (pos,_locals) =
        match code.blocks[0].body {
            Body::Imm(b) => code.bodies[b],
            Body::Defer(_,_) => panic!("cant run deferred block"),
        };
    let rtn = vm(&root,&code,&code.blocks[0],pos,Vec::new());
    match &**rtn.to_ref() {
        Vu::Scalar(n) => *n,
        Vu::A(a) => panic!("got array w/ shape {:?}",a.sh),
        _ => panic!("run failed"),
    }
}

#[rustler::nif]
fn init_st() -> NifResult<(Atom,ResourceArc<Env>,Vs)> {
    //let code = Code::new(vec![0,0,7],vec![new_scalar(5.0)],vec![(0,true,new_body(Body::Imm(0)))],vec![(0,0)]);
    //let root = Env::new(None,&code.blocks[0],None);
    panic!("cant init anything");
    //let rtn = vm(&root,&code,&code.blocks[0],code.blocks[0].pos,Vec::new());
    //Ok((ok(),ResourceArc::new(root),rtn))
}
