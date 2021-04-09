#! /usr/bin/python3
from subprocess import run
import shutil

LINKER = 'clang++-11'
RUSTC = 'rustc'
OPT = 'opt-11'
LLC = 'llc-11'
TARGET_CPU = 'native'

def add_fast_intrinsics(lines):
    is_fast_f_met = False
    is_slow_f_met = False
    output = []
    for line in lines:
        if '@dot_product_fast' in line:
            is_fast_f_met = True
        if '@dot_product_slow' in line:
            is_slow_f_met = True
        if is_fast_f_met and not is_slow_f_met:
            if 'fmul' in line:
                line = line.replace('fmul', 'fmul nsz arcp contract reassoc ')
            if 'fadd' in line:
                line = line.replace('fadd', 'fadd nsz arcp contract reassoc ')
        output.append(line)
    return output

def build_and_run():
    # Generate not optimized IR for dylib
    run(f"{RUSTC} dot_product.rs -Copt-level=3 -Ccodegen-units=1 -Cpanic=abort -Cno-prepopulate-passes --crate-name=dot_product --emit=llvm-ir".split())
    with open('dot_product.ll') as f:
        lines = tuple(f)
    # Manually adding fast intrinsics to generated LLVM IR
    opt_lines = add_fast_intrinsics(lines)
    with open('dot_product_changed.ll', 'w') as f:
        f.write(''.join(opt_lines))

    # Build IR into dynamic lib
    run(f"{OPT} -O3 -mcpu={TARGET_CPU} -S dot_product_changed.ll -o dot_product_opt.ll".split())
    run(f"{LLC} -O3 -mcpu={TARGET_CPU} -filetype=obj dot_product_opt.ll -o dot_product.o".split())
    run(f"{LINKER} dot_product.o -shared -o dot_product.so".split())

    run("cargo build".split())

    shutil.copy('dot_product.so', 'target/debug/dot_product.so')
    run('./target/debug/runner'.split())
    print()
    run('./target/debug/runner --fast'.split())

if __name__ == '__main__':
    build_and_run()
