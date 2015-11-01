pub struct IntMap<T> {
    p: u64,
    m: u64,
    notEmpty: bool,
    c: cntnts(T),
}

enum cntnts<T> {
    Branch ([IntMap(T); 2]),
    Leaf (T),
}

impl<T> IntMap<T> {
    fn new() -> IntMap<T> {
        IntMap {
            p: 0,
            m: 0,
            notEmpty: false,
            c: cntnts<T>,
        }
    }
    fn insert<'a>(&self, k: u64, v: &'a [T]) -> IntMap {

    }
}

impl<T: Copy> Copy for IntMap<T> {

}
