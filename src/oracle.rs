use std::rc::Rc;

use ringbuf::{Consumer, Producer, RingBuffer};

pub struct TwoWayRing<S, H> {
    ask: Rc<Producer<S>>,
    read: Rc<Consumer<H>>,
}

impl<S, H> TwoWayRing<S, H> {
    fn with(ask: Rc<Producer<S>>, read: Rc<Consumer<H>>) -> Self {
        Self { ask, read }
    }

    pub fn read(&mut self) -> Option<H> {
        Rc::get_mut(&mut self.read).and_then(|q| q.pop())
    }

    pub fn tell(&mut self, s: S) -> bool {
        Rc::get_mut(&mut self.ask)
        .map(|q| q.push(s).is_ok())
        .unwrap_or(false)
    }
}

pub struct Oracle;
impl Oracle {
    pub fn create<L, R>() -> (TwoWayRing<L, R>, TwoWayRing<R, L>)  {
        let rb_god = RingBuffer::<L>::new(10);
        let rb_player = RingBuffer::<R>::new(1);

        let (god_prod, god_cons) = rb_god.split();
        let (player_prod, player_cons) = rb_player.split();

        let oracle: TwoWayRing<L, R> = TwoWayRing::with(Rc::new(god_prod), Rc::new(player_cons));
        let god: TwoWayRing<R, L> = TwoWayRing::with(Rc::new(player_prod), Rc::new(god_cons));

        (oracle, god)
    }
}