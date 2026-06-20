use super::Mutex;

impl<A: Mutex> Mutex for (A,) {
    type Data<'a> = (A::Data<'a>,) where A: 'a;

    fn lock<'a, R>(&'a self, f: impl FnOnce(Self::Data<'a>) -> R) -> R {
        self.0.lock(move |d0| f((d0,)))
    }
}

impl<A: Mutex, B: Mutex> Mutex for (A, B) {
    type Data<'a> = (A::Data<'a>, B::Data<'a>) where A: 'a, B: 'a;

    fn lock<'a, R>(&'a self, f: impl FnOnce(Self::Data<'a>) -> R) -> R {
        self.0.lock(move |d0| self.1.lock(move |d1| f((d0, d1))))
    }
}

impl<A: Mutex, B: Mutex, C: Mutex> Mutex for (A, B, C) {
    type Data<'a> = (A::Data<'a>, B::Data<'a>, C::Data<'a>)
        where A: 'a, B: 'a, C: 'a;

    fn lock<'a, R>(&'a self, f: impl FnOnce(Self::Data<'a>) -> R) -> R {
        self.0.lock(move |d0| self.1.lock(move |d1| self.2.lock(move |d2| f((d0, d1, d2)))))
    }
}

impl<A: Mutex, B: Mutex, C: Mutex, D: Mutex> Mutex for (A, B, C, D) {
    type Data<'a> = (A::Data<'a>, B::Data<'a>, C::Data<'a>, D::Data<'a>)
        where A: 'a, B: 'a, C: 'a, D: 'a;

    fn lock<'a, R>(&'a self, f: impl FnOnce(Self::Data<'a>) -> R) -> R {
        self.0.lock(move |d0| self.1.lock(move |d1| self.2.lock(move |d2| self.3.lock(move |d3| f((d0, d1, d2, d3))))))
    }
}

impl<A: Mutex, B: Mutex, C: Mutex, D: Mutex, E: Mutex> Mutex for (A, B, C, D, E) {
    type Data<'a> = (A::Data<'a>, B::Data<'a>, C::Data<'a>, D::Data<'a>, E::Data<'a>)
        where A: 'a, B: 'a, C: 'a, D: 'a, E: 'a;

    fn lock<'a, R>(&'a self, f: impl FnOnce(Self::Data<'a>) -> R) -> R {
        self.0.lock(move |d0| self.1.lock(move |d1| self.2.lock(move |d2| self.3.lock(move |d3| self.4.lock(move |d4| f((d0, d1, d2, d3, d4)))))))
    }
}
