use crate::math::v2d;
// tools to manipulate systems

pub struct Modifier {

    // heat transfer enegy
    pub Q: f64,

    pub F: v2d,

}

impl Modifier {

    pub fn heat(&mut self, energy: f64) {
        self.Q += energy;

        println!("Q: {}", self.Q);
    }

    pub fn force(&mut self, force: v2d) {
        self.F = self.F.add(&force);

        println!("F: ({},{})",self.F.x,self.F.y);
    }


}