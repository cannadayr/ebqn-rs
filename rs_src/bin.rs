use log::{info};
use core::f64::{INFINITY,NEG_INFINITY};
use ebqn::init_log;
use ebqn::ebqn::{run,call,runtime,prog};
use ebqn::code::{r0,r1,c};
use ebqn::schema::{new_string,new_char,V,Vs};
use ebqn::prim::{provide,decompose,prim_ind};
use ebqn::schema::Body;
use ebqn::schema::new_scalar;
use ebqn::schema::Code;
#[cfg(feature = "dhat")]
use dhat::{Dhat, DhatAlloc};

#[cfg(feature = "dhat")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() {
    init_log();
    #[cfg(feature = "dhat")]
    let _dhat = Dhat::start_heap_profiling();

    const SIZE: usize = 128;
    const INIT: Vs = Vs::Nothing;
    let stack: [Vs; SIZE] = [INIT; SIZE];

    // each function one-at-a-time
    let runtime = runtime(&stack);
    let compiler = run(&stack,c(&runtime));
    let src = new_string("{×´1+↕𝕩}");
    let prog = prog(&stack,&compiler,src,&runtime);
    let exec = run(&stack,prog);
    let result = call(&stack,1,Some(&exec),Some(&V::Scalar(10.0)),None);
    println!("{}",result);

    // single line variations for copy-pasting
    //{ let runtimev = runtime(); let runtime = runtimev.as_a().unwrap();let compiler = c(&runtimev); }
    //{ let runtimev = runtime(); }
    //{ let runtimev = runtime(); let runtime = runtimev.as_a().unwrap();let compiler = c(&runtimev); call(1,Some(&run(prog(&compiler,src,&runtime))),Some(&V::Scalar(10.0)),None); }
    //{ let runtimev = runtime(); let runtime = runtimev.as_a().unwrap();let compiler = c(&runtimev); let result = call(1,Some(&run(prog(&compiler,src,&runtime))),Some(&V::Scalar(10.0)),None); pringln!("{}",result); }

}
