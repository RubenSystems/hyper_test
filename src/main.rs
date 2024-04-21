use rand::rngs::ThreadRng;
use thing::bundle;

use crate::thing::{bind, sim, Thing};

mod thing;

type Component = Thing<100_000>;

fn main() {
    let mut rng = rand::thread_rng();
    let noise = Component::from_rng(&mut rng);

    let lda = Component::from_rng(&mut rng);
    let add = Component::from_rng(&mut rng);
    let r1 = Component::from_rng(&mut rng);
    let l1 = Component::from_rng(&mut rng);

    let p1 = bundle(vec![&bind(&add, &r1), &bind(&lda, &r1)]).unwrap();
    let p2 = bind(&add, &l1);

    let both = bind(&p1, &p2);

    let un = bind(&l1, &both);

    sim(&r1, &un, &noise)
}
