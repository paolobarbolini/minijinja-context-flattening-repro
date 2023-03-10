fn main() {
    #[derive(Debug, serde::Serialize)]
    struct A {
        val1: i32,
    }

    #[derive(Debug, serde::Serialize)]
    struct B<S> {
        val2: i32,
        #[serde(flatten)]
        flattened: S,
    }

    // This is ok
    test(B {
        val2: 2,
        flattened: A { val1: 1 },
    });

    // This is ok
    test(minijinja::context! {
        val1 => 1,
        val2 => 2,
    });

    // This is ok
    test(B {
        val2: 2,
        flattened: serde_json::json!({
            "val1": 1,
        }),
    });

    // This is not
    test(B {
        val2: 2,
        flattened: minijinja::context! {
            val1 => 1,
        },
    });

    // This works
    test(
        serde_json::to_value(&B {
            val2: 2,
            flattened: minijinja::context! {
                val1 => 1,
            },
        })
        .unwrap(),
    );
}

fn test(ctx: impl serde::Serialize) {
    {
        let env = minijinja::Environment::new();
        let expr = env.compile_expression("val1 == 1 and val2 == 2").unwrap();
        let rv = expr.eval(&ctx).unwrap();
        if rv.is_true() {
            println!("Expression is true");
        } else {
            println!("Expression is false");
        }
    }

    {
        let env = minijinja::Environment::new();
        let out = env
            .render_str("val1: '{{ val1 }}' - val2: '{{ val2 }}'", &ctx)
            .unwrap();
        if out == "val1: '1' - val2: '2'" {
            println!("Render is correct");
        } else {
            println!("Render is wrong ({out:?})");
        }
    }
}
