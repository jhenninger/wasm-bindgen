#![allow(non_snake_case)]

use super::project;

#[test]
fn abs() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn abs(x: f64) -> js::Number {
                js::Math::abs(x)
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.abs(-32), Math.abs(-32));
                assert.equal(wasm.abs(32), 32);
                assert.equal(wasm.abs(-4.7), Math.abs(-4.7));
            }
        "#,
        )
        .test()
}

#[test]
fn acos() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn acos(x: f64) -> js::Number {
                js::Math::acos(x)
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.acos(-1), Math.PI);
                assert.equal(wasm.acos(0.5), 1.0471975511965979);
                assert(Number.isNaN(wasm.acos(2)));
            }
        "#,
        )
        .test()
}

#[test]
fn acosh() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn acosh(x: f64) -> js::Number {
                js::Math::acosh(x)
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.acosh(1), 0);
                assert.equal(wasm.acosh(2), Math.acosh(2));
                assert(Number.isNaN(wasm.acosh(0.5)));
            }
        "#,
        )
        .test()
}

#[test]
fn asin() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn asin(x: f64) -> js::Number {
                js::Math::asin(x)
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.asin(1), Math.asin(1));
                assert.equal(wasm.asin(0.5), Math.asin(0.5));
                assert(Number.isNaN(wasm.asin(2)));
            }
        "#,
        )
        .test()
}

#[test]
fn asinh() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn asinh(x: f64) -> js::Number {
                js::Math::asinh(x)
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.asinh(1), Math.asinh(1));
                assert.equal(wasm.asinh(0.5), Math.asinh(0.5));
            }
        "#,
        )
        .test()
}

#[test]
fn atan() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn atan(x: f64) -> js::Number {
                js::Math::atan(x)
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.atan(1), Math.atan(1));
                assert.equal(wasm.atan(0.5), Math.atan(0.5));
            }
        "#,
        )
        .test()
}

#[test]
fn atan2() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn atan2(y: f64, x: f64) -> js::Number {
                js::Math::atan2(y, x)
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.atan2(1, 2), Math.atan2(1, 2));
                assert.equal(wasm.atan2(0.7, 3.8), Math.atan2(0.7, 3.8));
            }
        "#,
        )
        .test()
}

#[test]
fn atanh() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn atanh(x: f64) -> js::Number {
                js::Math::atanh(x)
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.atanh(1), Math.atanh(1));
                assert.equal(wasm.atanh(0.5), Math.atanh(0.5));
                assert(Number.isNaN(wasm.atanh(2)));
            }
        "#,
        )
        .test()
}

#[test]
fn cbrt() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn cbrt(x: f64) -> js::Number {
                js::Math::cbrt(x)
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.cbrt(27), 3);
                assert.equal(wasm.cbrt(12.3), Math.cbrt(12.3));
            }
        "#,
        )
        .test()
}

#[test]
fn ceil() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn ceil(x: f64) -> js::Number {
                js::Math::ceil(x)
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.ceil(1.1), 2);
                assert.equal(wasm.ceil(-1.1), -1);
            }
        "#,
        )
        .test()
}

#[test]
fn clz32() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn clz32(x: i32) -> js::Number {
                js::Math::clz32(x)
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.clz32(1), 31);
                assert.equal(wasm.clz32(1000), 22);
            }
        "#,
        )
        .test()
}

#[test]
fn cos() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn cos(x: f64) -> js::Number {
                js::Math::cos(x)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.cos(0), 1);
                assert.equal(wasm.cos(1.5), Math.cos(1.5));
            }
        "#)
        .test()
}

#[test]
fn cosh() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn cosh(x: f64) -> js::Number {
                js::Math::cosh(x)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.cosh(0), 1);
                assert.equal(wasm.cosh(2), 3.7621956910836314);
            }
        "#)
        .test()
}

#[test]
fn exp() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn exp(x: f64) -> js::Number {
                js::Math::exp(x)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.exp(0), 1);
                assert.equal(wasm.exp(-1), 0.36787944117144233);
                assert.equal(wasm.exp(2), 7.38905609893065);
            }
        "#)
        .test()
}

#[test]
fn expm1() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn expm1(x: f64) -> js::Number {
                js::Math::expm1(x)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.expm1(0), 0);
                assert.equal(wasm.expm1(1), 1.718281828459045);
                assert.equal(wasm.expm1(-1), -0.6321205588285577);
                assert.equal(wasm.expm1(2), 6.38905609893065);
            }
        "#)
        .test()
}

#[test]
fn floor() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn floor(x: f64) -> js::Number {
                js::Math::floor(x)
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.floor(5.95), 5);
                assert.equal(wasm.floor(-5.05), -6);
            }
        "#,
        )
        .test()
}

#[test]
fn fround() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn fround(x: f64) -> js::Number {
                js::Math::fround(x)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.fround(5.5), 5.5);
                assert.equal(wasm.fround(5.05), 5.050000190734863);
                assert.equal(wasm.fround(5), 5);
                assert.equal(wasm.fround(-5.05), -5.050000190734863);
            }
        "#)
        .test()
}

#[test]
fn imul() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn imul(x: i32, y:i32) -> js::Number {
                js::Math::imul(x, y)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.imul(3, 4), 12);
                assert.equal(wasm.imul(-5, 12), -60);
                assert.equal(wasm.imul(0xffffffff, 5), Math.imul(0xffffffff, 5));
            }
        "#)
        .test()
}

#[test]
fn log() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn log(x: f64) -> js::Number {
                js::Math::log(x)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.log(8) / wasm.log(2), 3);
                assert.equal(wasm.log(625) / wasm.log(5), 4);
            }
        "#)
        .test()
}

#[test]
fn log10() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn log10(x: f64) -> js::Number {
                js::Math::log10(x)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.log10(100000), 5);
                assert.equal(wasm.log10(1), 0);
                assert.equal(wasm.log10(2), 0.3010299956639812);
            }
        "#)
        .test()
}

#[test]
fn log1p() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn log1p(x: f64) -> js::Number {
                js::Math::log1p(x)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.log1p(1), 0.6931471805599453);
                assert.equal(wasm.log1p(0), 0);
                assert.equal(wasm.log1p(-1), -Infinity);
                assert(isNaN(wasm.log1p(-2)));
            }
        "#)
        .test()
}

#[test]
fn log2() {
    project()
        .file("src/lib.rs", r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn log2(x: f64) -> js::Number {
                js::Math::log2(x)
            }
        "#)
        .file("test.ts", r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.log2(3), 1.584962500721156);
                assert.equal(wasm.log2(2), 1);
                assert.equal(wasm.log2(1), 0);
                assert.equal(wasm.log2(0), -Infinity);
            }
        "#)
        .test()
}