use log::info;

pub fn lesson_11_main() {
    info!("Lesson 11: start");
    template_example();
    template_impl_example();
    info!("Lesson 11: end");
}

fn template_example() {
    struct A;
    struct S(A);
    struct SGen<T>(T);

    fn reg_fn(_s: S) {}

    fn gen_spec_t(_s: SGen<A>) {}

    fn gen_spec_i32(_s: SGen<i32>) {}

    fn generic<T>(_s: SGen<T>) {}

    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));
    generic(SGen('c'));
}

fn template_impl_example() {
    struct S;
    struct GenericVal<T>(T);

    impl GenericVal<f32> {}
    impl GenericVal<S> {}

    impl<T> GenericVal<T> {}

    struct Val {
        val: f64,
    }
}
