#[cfg(test)]
mod basic_example {
use crate::schematic_builder::{block_builders::*, blocks, schematic_builder::SchematicBuilder, Pos};

    fn make_sphere(
        sb: &mut SchematicBuilder,
        center: Pos,
        radius: f32,
        bb: impl BlockBuilder + Clone + 'static,
    ) {
        for pos in sb.pos_iter() {
            let dx = center.0 - pos.0;
            let dy = center.1 - pos.1;
            let dz = center.2 - pos.2;
            if dx * dx + dy * dy + dz * dz < (radius * radius) as i32 {
                sb.set(pos, Box::new(bb.clone()));
            }
        }
    }

    fn make_line_of_barrels(
        sb: &mut SchematicBuilder,
        start: Pos,
        bound: usize,
        bb: InventoryBlock,
    ) {
        for i in 0..bound {
            let pos = (start.0 + i as i32, start.1, start.2);
            let mut current = bb.clone();
            current.set_signal_strength(i as u16).unwrap();
            sb.set(pos, Box::new(current));
        }
    }
    #[test]
    pub fn run() {
        let mut sb: SchematicBuilder = SchematicBuilder::new(100, 40, 40);
        make_sphere(&mut sb, (30, 10, 5), 30.0, blocks::white_concrete());
        make_line_of_barrels(
            &mut sb,
            (30, 0, 10),
            50,
            blocks::empty_barrel(),
        );
        sb.build().save("output/sphere_and_barrel.schem");
    }
}
#[cfg(test)]
mod color_gradient{
    use crate::{schematic_builder::{schematic_builder::SchematicBuilder, block_from_color::{BlockSelector}}};



    fn perdicate(s: &String) -> bool{
        !s.contains("glass")
    }


    #[test]
    pub fn run(){
        let mut sb: SchematicBuilder = SchematicBuilder::new(64, 64, 64);
        let mut block_selector = BlockSelector::new();
        block_selector = block_selector.filtered(|x| perdicate(x));
        for pos in sb.pos_iter(){

            if pos.0 % 2 != 0 || pos.1 %2 != 0 || pos.2 % 2 != 0{
                continue;
            }

            let block = block_selector.get_closest((pos.0 as u8 * 4, pos.1 as u8 * 4, pos.2 as u8 * 4)).unwrap();
            sb.set(pos, Box::new(block));
        }
        sb.build().save("output/grad.schem")

    }
}
mod mandelbulb{
    use crate::schematic_builder::{schematic_builder::SchematicBuilder, blocks::white_concrete};

    type vec3 = (f64,f64,f64);

    fn n_th_pow((x,y,z): vec3, n: i32) -> vec3{

        let r = abs((x,y,z));
        let phi = x.atan2(y);
        let theta = (z/r).acos();
        let rn = r.powi(n);

        let (s1, c1) = (n as f64*theta).sin_cos();
        let (s2, c2) = (n as f64*phi).sin_cos();

         (
            rn*s1*c2,
            rn*s1*s1,
            rn*c1
         )
    }

    fn abs((x,y,z): vec3) -> f64{
        return (x*x + y*y + z*z).sqrt();
    }
    fn add((x1,y1,z1): vec3,(x2,y2,z2): vec3) -> vec3{
        (x1+x2,y1+y2,z1+z2)
    }

    fn escapes(val: vec3, c: vec3, n: usize, bound: f64) -> Option<usize>{
        let mut z = val;
        for i in 0..n {
            z = add(n_th_pow(z, 8),c);
            if abs(z) > bound{
                return Some(i);
            }
        }
        return None;
    }
    fn ratio(n1: f64, n2: f64) -> f64{
        n1 / n2
    }
    #[test]
    fn run(){
        const SIZE: usize = 200;
        let mut sb = SchematicBuilder::new(SIZE,SIZE,SIZE);
        for pos in sb.pos_iter(){
            let (x,y,z) = pos.clone();
            let (x, y, z) = (ratio(x as f64, SIZE as f64), ratio(y as f64, SIZE as f64), ratio(z as f64, SIZE as f64));
            let (x,y,z) = (x*2.0-1.0,y*2.0-1.0,z*2.0-1.0);
            let (x,y,z) = (x*1.0,y*1.0,z*1.0);
            if escapes((x,y,z), (x,y,z), 50, 10.0).is_none(){
                sb.set(pos, Box::new(white_concrete()));
            }
        }

        sb.build().save("output/madn.schem")
    }



}
