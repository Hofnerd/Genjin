use specs::prelude::*;

use crate::entity_components::*;

pub struct DecaySys;

impl<'a> System<'a> for DecaySys {
    type SystemData = (Entities<'a>, WriteStorage<'a, DecayLife>);

    fn run(&mut self, (entities, mut life): Self::SystemData) {
        (&entities, &mut life).par_join().for_each(|(e, life)| {
            if life.life == 0 {
                let _ = entities.delete(e);
            } else {
                life.life -= 1;
            }
        });
    }
}
