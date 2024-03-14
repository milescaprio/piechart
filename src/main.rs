use std::f32::consts::PI;

fn letter_piechart(values : Vec::<(char, f32)>, radius : usize) {
    /// All values should add up to exactly 1: 1 full pie chart.
    let mut angles = Vec::<f32>::new();
    let mut angle : f32 = 0.;
    for i in &values {
        angles.push(angle);
        angle += i.1 * 2.0 * PI
    }
    println!("{:?}", angles);
    let mut coordinate_plane = vec![vec![' '; radius*2+1];radius*2+1];
    for i in 0..(radius*2+1) {
        for j in 0..(radius*2+1) {
            // X and Y centered about the array
            let x = j as f32 - radius as f32 - 0.5;
            let y = -(i as f32 - radius as f32 - 0.5);

            //Angle that the iterated x,y is at from the center. Mapped to be in range [0, 2PI], as traditional unit circle value
            let mut angle = f32::atan2(y,x);
            if angle < 0. {
                angle += 2. * PI;
            }

            //Change angle coordinates to 0 at top and then rotating around clockwise...like a pi chart.
            let mut pieangle = PI/2. - angle;
            if pieangle < 0. {
                pieangle += 2. * PI;
            }

            //Get radius from center, to compare with radius of pie chart. If less than, then draw a letter; this will graph the circle
            //Then take the angle and compare it to each beginning-slice angle value. The LAST one that it is greater than is the one that bucket that it falls in.
            let r = x.hypot(y);
            let mut letter = ' ';
            if r < radius as f32 {
                for i in 0..angles.len() {
                    if pieangle > angles[i] {
                        letter = values[i].0;
                        
                    } else {
                        break;
                    }
                }
            }
            coordinate_plane[i][j] = letter;
        }
    }

    //Print out all the shit
    for i in &coordinate_plane {
        for j in i {
            print!("{}", j);
        }
        println!("");
    }
}

fn main() {
    letter_piechart(vec![('A', 0.2), ('B', 0.4), ('C', 0.15), ('X', 0.25)], 20);
}
