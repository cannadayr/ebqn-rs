use crate::schema::{A,V,Vn,Vs,Decoder};
use crate::ebqn::{call};
use cc_mt::Cc;

/*
pub fn plus(arity:usize, x: Vn,w: Vn) -> Vs {
    match arity {
        1 => panic!("no monadic addition"),
        2 => Vs::V(V::Scalar(x.unwrap().to_f64() + w.unwrap().to_f64())),
        _ => panic!("illegal arity"),
    }
}

fn noop(_arity: usize, _x: Vn, _w: Vn) -> Vs {
    panic!("noop not implemented");
}
fn noop1(_arity: usize, _f: Vn, _x: Vn, _w: Vn) -> Vs {
    panic!("noop1 not implemented");
}
fn noop2(_arity: usize, _f: Vn, _g: Vn, _x: Vn, _w: Vn) -> Vs {
    panic!("noop2 not implemented");
}
*/

// Type
fn typ(arity: usize, x: Vn, _w: Vn) -> Vs {
    match arity {
        1 => match x.unwrap() {
            V::Scalar(n) => Vs::V(V::Scalar(1.0)),
            _ => panic!("no matching value for typ"),
        },
        _ => panic!("typ not implemented"),
    }
}
// Fill
fn fill(arity: usize, x: Vn, _w: Vn) -> Vs {
    match arity {
        1 => Vs::V(V::Scalar(0.0)),
        2 => Vs::V(x.unwrap()),
        _ => panic!("illegal fill arity"),
    }
}
// Log
fn log(_arity: usize, _x: Vn, _w: Vn) -> Vs {
    panic!("log not implemented");
}
// GroupLen
fn group_len(_arity: usize, _x: Vn, _w: Vn) -> Vs {
    panic!("group_len not implemented");
}
// GroupOrd
fn group_ord(_arity: usize, _x: Vn, _w: Vn) -> Vs {
    panic!("group_ord not implemented");
}
// !
fn assert_fn(_arity: usize, _x: Vn, _w: Vn) -> Vs {
    panic!("assert_fn not implemented");
}
// +
fn plus(_arity: usize, _x: Vn, _w: Vn) -> Vs {
    panic!("plus not implemented");
}
// -
fn minus(arity: usize, x: Vn, w: Vn) -> Vs {
    match arity {
        1 => Vs::V(V::Scalar(-1.0 * x.unwrap().to_f64())),
        2 => Vs::V(V::Scalar(w.unwrap().to_f64() - x.unwrap().to_f64())),
        _ => panic!("illegal minus arity"),
    }
}
// ×
fn times(arity: usize, x: Vn, w: Vn) -> Vs {
    match arity {
        1 => Vs::V(V::Scalar(x.unwrap().to_f64().signum())),
        2 => Vs::V(V::Scalar(w.unwrap().to_f64() * x.unwrap().to_f64())),
        _ => panic!("illegal times arity"),
    }
}
// ÷
fn divide(_arity: usize, _x: Vn, _w: Vn) -> Vs {
    panic!("divide not implemented");
}
// ⋆
fn power(_arity: usize, _x: Vn, _w: Vn) -> Vs {
    panic!("power not implemented");
}
// ⌊
fn floor(_arity: usize, _x: Vn, _w: Vn) -> Vs {
    panic!("floor not implemented");
}
// =
fn equals(_arity: usize, _x: Vn, _w: Vn) -> Vs {
    panic!("equals not implemented");
}
// ≤
fn lesseq(arity: usize, x: Vn, w: Vn) -> Vs {
    match arity {
        2 => {
            let t = typ(1,x.clone(),None).to_ref().to_f64();
            let s = typ(1,w.clone(),None).to_ref().to_f64();
            match t != s {
                true  => Vs::V(V::Scalar((s <= t) as i64 as f64)),
                false => Vs::V(V::Scalar((w.unwrap().to_f64() <= x.unwrap().to_f64()) as i64 as f64)),
            }
        },
        _ => panic!("illegal lesseq arity"),
    }
}
// ≢
fn shape(_arity: usize, _x: Vn, _w: Vn) -> Vs {
    panic!("shape not implemented");
}
// ⥊
fn reshape(arity: usize, x: Vn, w: Vn) -> Vs {
    match arity {
        1 => {
            match x.unwrap() {
                V::A(_a) => panic!("monadic reshape arr"),
                _ => panic!("monadic reshape no arr"),
            }
        },
        2 => {
            match (x.unwrap(),w.unwrap()) {
                (V::A(ax),V::A(aw)) => {
                    let sh = aw.r.iter().map(|e| match e {
                        V::Scalar(n) => *n as usize,
                        _ => panic!("W ravel is not a num"),
                    }).collect::<Vec<usize>>();
                    Vs::V(V::A(Cc::new(A::new(ax.r.clone(),sh))))
                },
                _ => panic!("dydic reshape no match"),
            }
        },
        _ => panic!("illegal reshape arity"),
    }
}
// ⊑
fn pick(_arity: usize, _x: Vn, _w: Vn) -> Vs {
    panic!("pick not implemented");
}
// ↕
fn windows(_arity: usize, _x: Vn, _w: Vn) -> Vs {
    panic!("windows not implemented");
}
// ⌜
fn table(arity: usize, f: Vn, x: Vn, w: Vn) -> Vs {
    match arity {
        1 => panic!("monadic table"),
        2 => {
            match (x.unwrap(),w.unwrap()) {
                (V::A(xa),V::A(wa)) => {
                    let ravel = (*wa).r.iter().map(|d| {
                        let ravel = (*xa).r.iter().map(|e| call(arity,f.clone(),Some(e.clone()),Some(d.clone())).to_ref().clone()).collect::<Vec<V>>();
                        let sh = ravel.len();
                        V::A(Cc::new(A::new(ravel,vec![sh])))
                    }).collect::<Vec<V>>();
                    let sh = ravel.len();
                    Vs::V(V::A(Cc::new(A::new(ravel,vec![sh]))))
                },
                _ => panic!("dyadic table not an array"),
            }
        },
        _ => panic!("illegal table arity"),
    }
}
// `
fn scan(_arity: usize, _f: Vn, _x: Vn, _w: Vn) -> Vs {
    panic!("scan not implemented");
}
// _fillBy_
fn fill_by(_arity: usize, _f: Vn, _g: Vn, _x: Vn, _w: Vn) -> Vs {
    panic!("fill_by not implemented");
}
// ⊘
fn cases(arity: usize, f: Vn, g: Vn, x: Vn, w: Vn) -> Vs {
    match arity {
        1 => call(arity,f,x,None),
        2 => call(arity,g,x,w),
        _ => panic!("illegal cases arity"),
    }
}
// ⎊
fn catches(_arity: usize, _f: Vn, _g: Vn, _x: Vn, _w: Vn) -> Vs {
    panic!("catches not implemented");
}

pub fn provide() -> A {
    let fns = vec![V::Fn(typ),
                   V::Fn(fill),
                   V::Fn(log),
                   V::Fn(group_len),
                   V::Fn(group_ord),
                   V::Fn(assert_fn),
                   V::Fn(plus),
                   V::Fn(minus),
                   V::Fn(times),
                   V::Fn(divide),
                   V::Fn(power),
                   V::Fn(floor),
                   V::Fn(equals),
                   V::Fn(lesseq),
                   V::Fn(shape),
                   V::Fn(reshape),
                   V::Fn(pick),
                   V::Fn(windows),
                   V::R1(table),
                   V::R1(scan),
                   V::R2(fill_by),
                   V::R2(cases),
                   V::R2(catches)];
    A::new(fns,vec![23])
}
