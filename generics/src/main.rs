enum Error<T> {
    Succeed(T),
    Fail,
}

impl Error<f32> {
    fn half(&self) -> Self {
        match self {
            Error::Succeed(t) => Error::Succeed(t/2.0),
            _ => Error::Fail,
        }
    }
}

impl<T> Error<T> {
    fn world(&self) {
        println!("Hello world");
    }
}


struct Vector3<T, U> {
    x: T,
    y: T,
    z: U,
}

impl<T, U> Vector3<T, U> {
    fn get_2d(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }

    fn mix<V, W> (self, target: Vector3<V, W>) -> Vector3<T, W> {
        Vector3 {x: self.x,y: self.y,z: target.z}
    }
}

impl<T> Vector3<T, i32> {
    fn pr(&mut self) {
        self.z = 1+&self.z;
    }
}

fn main() {
    let mut e = Vector3{x: 2.33, y: 33.11, z: 7};
    e.get_2d();
    e.pr();
    let e2 = Vector3{x:"Hello", y:"potato", z:"eric".to_string()};
    let e = e.mix(e2);
    println!("{}, {}, {}", e.x, e.y, e.z);
    let error = Error::Succeed(33.3);
    error.half();
    error.world();
    let error_int = Error::Succeed(33);
    error_int.world();
}
