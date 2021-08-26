use std::rc::Rc;

use ringbuf::{Consumer, Producer, RingBuffer};

pub struct TwoWayRing<S, H> {
    ask: Producer<S>,
    read: Consumer<H>,
}

impl<S, H> TwoWayRing<S, H> {
    fn with(ask: Producer<S>, read: Consumer<H>) -> Self {
        Self { ask, read }
    }

    pub fn read(&mut self) -> Option<H> {
        self.read.pop()
    }

    pub fn tell(&mut self, s: S) -> bool {
        self.ask.push(s).is_ok()
    }
}

pub struct Oracle;
impl Oracle {
    pub fn create<L, R>(god: usize, prays: usize) -> (TwoWayRing<L, R>, TwoWayRing<R, L>)  {
        let rb_god = RingBuffer::<L>::new(god);
        let rb_player = RingBuffer::<R>::new(prays);

        let (god_prod, god_cons) = rb_god.split();
        let (player_prod, player_cons) = rb_player.split();

        let oracle: TwoWayRing<L, R> = TwoWayRing::with(god_prod, player_cons);
        let god: TwoWayRing<R, L> = TwoWayRing::with(player_prod, god_cons);

        (oracle, god)
    }
}